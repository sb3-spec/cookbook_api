use std::{sync::Arc, ops::Deref, str::from_utf8};

use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::json;
use warp::{Filter, reply::Json};
use reqwest::get;
use scraper::{Html, Selector, ElementRef, Node};
use regex::Regex;

use crate::{model::{RecipeMac, RecipePatch}, security::UserCtx, entities::recipe};

use super::{filter_utils::with_db, filter_auth::do_auth};

pub fn recipe_rest_filters(
    base_path: &'static str,
    db: Arc<DatabaseConnection>
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let recipes_path = warp::path(base_path).and(warp::path("recipes"));
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    // LIST recipe 'GET recipes/'

    let list = recipes_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(recipe_list);

    let get = recipes_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and_then(recipe_get);

    let create = recipes_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json::<RecipePatch>())
        .and_then(recipe_create);

    let update = recipes_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and(warp::body::json::<RecipePatch>())
        .and_then(recipe_update);

    let delete = recipes_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param::<i64>())
        .and_then(recipe_delete);

    let get_by_tag = recipes_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path("tags"))
        .and(warp::path::param::<String>())
        .and_then(recipe_get_by_tag);

    let scrape_url = recipes_path
        .and(warp::get())
        // .and(common.clone())
        .and(warp::path("scrape"))
        .and(warp::path::param::<String>())
        .and_then(scrape_recipe);

    list.or(get).or(create).or(delete).or(update).or(get_by_tag).or(scrape_url)
} 

async fn recipe_list(db: Arc<DatabaseConnection>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    // FIXME: Add proper error handling
    let recipes = RecipeMac::list(&db).await?;

    json_response(recipes)
}

async fn recipe_get(db: Arc<DatabaseConnection>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::get(&db, id).await?;

    json_response(recipe)
}

async fn recipe_create(db: Arc<DatabaseConnection>, utx: UserCtx, patch: RecipePatch) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::create(&db, patch, utx).await?;
    json_response(recipe)
}


async fn recipe_delete(db: Arc<DatabaseConnection>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let deleted_id = RecipeMac::delete(&db, utx, id).await?;
    json_response(deleted_id)
}

async fn recipe_update(db: Arc<DatabaseConnection>, utx: UserCtx, id: i64, patch: RecipePatch) -> Result<Json, warp::Rejection> {
    let recipe = RecipeMac::update(&db, patch, utx, id).await?;

    json_response(recipe)
}

async fn recipe_get_by_tag(db: Arc<DatabaseConnection>, utx: UserCtx, tag: String) -> Result<Json, warp::Rejection> {
    let recipes = RecipeMac::get_by_tag(&db, utx, tag.as_str()).await?;

    json_response(recipes)
}

async fn scrape_recipe(encoded_url: String) -> Result<Json, warp::Rejection> {
    let url: String = urlencoding::decode(encoded_url.as_str()).unwrap().to_string();
    let response = get(url).await.unwrap().text().await.unwrap();

    let html = Html::parse_document(&response);

    // SCRAPE SELECTORS
    let meta_selector = Selector::parse("meta").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let list_selector = Selector::parse("ol").unwrap();
    let header_selector = Selector::parse("h2").unwrap();
    let div_selector = Selector::parse("div").unwrap();

    let mut recipe_data = json!({
        "ingredients": [],
        "steps": []
    });

    for element in html.select(&meta_selector) {
        let element_content = element.value().attr("content").unwrap_or("");
        // println!("{:?}", element.value());


        // Grabs meta data that website will include for search engine results
        match element.value().attr("property") {
            Some("og:title") => {
                if !element_content.is_empty() {
                    recipe_data["title"] = element_content.into();
                }               
            },
            Some("og:description") => {
                if !element_content.is_empty() {
                    recipe_data["description"] = element_content.into();
                }   
            },
            Some("og:image") => {
                if !element_content.is_empty() {
                    recipe_data["image_url"] = element_content.into();
                }
            },
            Some("og:image:height") => {
                if !element_content.is_empty() {
                    recipe_data["image_height"] = element_content.into();
                }
            },
            Some("og:image:width") => {
                if !element_content.is_empty() {
                    recipe_data["image_width"] = element_content.into();
                }
            },
            _=> {}, 
        }
    }

    // GET THE INGREDIENTS!!!
    let mut ingredient_list: Vec<String> = Vec::new();
    let mut step_list: Vec<String> = Vec::new();

    // This part mostly scrapes ingredient info
    // APPROACH 1 
    for element in html.select(&li_selector) {
        for class in element.value().classes() {
            if class.to_lowercase().contains("ingredient") {
                // Grabs all the text from the element
                let ingredient_string = element.text().collect::<Vec<_>>().join("");
                let parsed_string = ingredient_string.replace("\n", "").into_bytes();
                ingredient_list.push(from_utf8(&parsed_string).unwrap().into());
            } else if class.to_lowercase().contains("instruction") | class.contains("step") {
                let step_string_list = element.text().collect::<Vec<_>>().join("");
                step_list.push(step_string_list);
            }           
        } 
    }

    
    // Approach 2
    for element in html.select(&list_selector) {
        
        for class in element.value().classes() {
            if class.to_lowercase().contains("ingredient") {
                ingredient_list = element.children().map(|child| 
                    ElementRef::wrap(child).unwrap()).filter(|child| child.value().name() == "li").map(|child| 
                    child.text().collect::<Vec<_>>().join("").chars().filter(|c| c.is_alphanumeric()).collect::<String>()).collect::<Vec<String>>();
            } else if class.to_lowercase().contains("step") | class.to_lowercase().contains("instructions") {
                step_list = element.children().map(|child| 
                    ElementRef::wrap(child).unwrap()).filter(|child| child.value().name() == "li").map(|child| 
                    child.text().collect::<Vec<_>>().join("")).collect::<Vec<String>>();
            }
        }
    }

    // Approach 3
    for element in html.select(&div_selector) {
        for class in element.value().classes() {
            if class.to_lowercase().contains("step") | class.to_lowercase().contains("instruction") {
                for child in element.children() {

                    
                    let mut el = ElementRef::wrap(child.clone());;

                    if (el.is_none()) {
                        continue;
                    }

                    let unwrapped_el = el.unwrap();

                    let html_regex = Regex::new(r"<[^>]*>").unwrap();

                    if unwrapped_el.value().name() == "ul" || unwrapped_el.value().name() == "ol" {
                        step_list = unwrapped_el.children().filter(|child| ElementRef::wrap(*child).is_some()).map(|child| 
                            ElementRef::wrap(child).unwrap())
                            .filter(|child| child.value().name() == "li").map(|child| 
                            html_regex.replace(from_utf8(child.text().collect::<Vec<_>>().join("").replace("\n", "").as_bytes()).unwrap().into(), "").to_string()).collect::<Vec<String>>();
                    }
                    
                }
            }
        }
    }

    recipe_data["ingredients"] = ingredient_list.into();
    recipe_data["steps"] = step_list.into();

    json_response(recipe_data)
}

// region: Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}

// endregion: Utils

#[cfg(test)]
#[path = "../_tests/web_recipe.rs"]
mod tests;