use std::{collections::BTreeSet, str::from_utf8};

use regex::Regex;
use reqwest::get;
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;
use serde_json::json;
use warp::reply::Json;

pub async fn scrape_recipe_content(encoded_url: String) -> Result<Json, warp::Rejection> {
    let url: String = urlencoding::decode(encoded_url.as_str())
        .unwrap()
        .to_string();
    let response = get(url).await.unwrap().text().await.unwrap();

    let html = Html::parse_document(&response);

    // SCRAPE SELECTORS
    let meta_selector = Selector::parse("meta").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let list_selector = Selector::parse("ol").unwrap();
    let header_selector = Selector::parse("h2").unwrap();
    let div_selector = Selector::parse("div").unwrap();
    let span_selector = Selector::parse("span").unwrap();
    let test = Selector::parse("span").unwrap();

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
            }
            Some("og:description") => {
                if !element_content.is_empty() {
                    recipe_data["header"] = element_content.into();
                }
            }
            Some("og:image") => {
                if !element_content.is_empty() {
                    recipe_data["image_url"] = element_content.into();
                }
            }
            Some("og:image:height") => {
                if !element_content.is_empty() {
                    recipe_data["image_height"] = element_content.into();
                }
            }
            Some("og:image:width") => {
                if !element_content.is_empty() {
                    recipe_data["image_width"] = element_content.into();
                }
            }
            _ => {}
        }
    }

    let mut ingredient_list: Vec<String> = Vec::new();
    let mut step_list: Vec<String> = Vec::new();

    // Grabbing steps and ingredients
    for element in html.select(&div_selector) {
        for class in element.value().classes() {
            // Looks through the children of the element if it (element) has a class that contains the string instruction
            if class.to_lowercase().contains("step")
                | class.to_lowercase().contains("instruction")
                | class.to_lowercase().contains("prep")
            {
                for child in element.children() {
                    let mut el = ElementRef::wrap(child);

                    if (el.is_none()) {
                        continue;
                    }

                    let unwrapped_el = el.unwrap();
                    let unwrapped_el_name = unwrapped_el.value().name();

                    let html_regex = Regex::new(r"<[^>]*>").unwrap();

                    if unwrapped_el_name == "ul" || unwrapped_el_name == "ol" {
                        step_list = unwrapped_el
                            .children()
                            .filter_map(ElementRef::wrap)
                            .filter(|child| child.value().name() == "li")
                            .map(|child| {
                                html_regex
                                    .replace(
                                        from_utf8(
                                            child
                                                .text()
                                                .collect::<Vec<_>>()
                                                .join("")
                                                .replace('\n', "")
                                                .as_bytes(),
                                        )
                                        .unwrap(),
                                        "",
                                    )
                                    .to_string()
                            })
                            .collect::<Vec<String>>();
                    }
                }
            }

            // Grabbing the ingredients

            if class.to_lowercase().contains("ingredient") {
                for child in element.children() {
                    let mut el = ElementRef::wrap(child);

                    if (el.is_none()) {
                        continue;
                    }

                    let unwrapped_el = el.unwrap();
                    let unwrapped_el_name = unwrapped_el.value().name();

                    let html_regex = Regex::new(r"<[^>]*>").unwrap();

                    if unwrapped_el_name == "ul" || unwrapped_el_name == "ol" {
                        ingredient_list = unwrapped_el
                            .children()
                            .filter_map(ElementRef::wrap)
                            .filter(|child| child.value().name() == "li")
                            .map(|child| {
                                html_regex
                                    .replace(
                                        from_utf8(
                                            child
                                                .text()
                                                .collect::<Vec<_>>()
                                                .join("")
                                                .replace('\t', "")
                                                .as_bytes(),
                                        )
                                        .unwrap(),
                                        "",
                                    )
                                    .to_string()
                                    .chars()
                                    .filter(|c| c.is_ascii())
                                    .collect::<String>()
                                    .trim()
                                    .to_owned()
                            })
                            .collect::<Vec<String>>();
                    }
                }
            }
        }
    }

    // Grabbing cook, prep, and total time values by searching all span elements
    for element in html
        .select(&span_selector)
        .chain(html.select(&div_selector))
    {
        let time_types = ["prep time", "cook time", "total time"];

        let el_text = element.text().collect::<Vec<_>>().join("");
        for typ in time_types {
            if el_text.trim().eq_ignore_ascii_case(typ)
                || el_text
                    .trim()
                    .eq_ignore_ascii_case(&format!("{}{}", typ, ':'))
            {
                if let Some(time) = scrape_recipe_time(element) {
                    recipe_data[typ.replace(' ', "_")] = time.into();
                }
            }
        }
    }

    recipe_data["ingredients"] = ingredient_list.into();
    recipe_data["steps"] = step_list.into();
    recipe_data["tags"] = Vec::<String>::new().into();

    json_response(recipe_data)
}

/// Takes in an html element reference and returns the text from within
/// This is to be used on recipes to grab the prep time, cook time and total time
pub fn scrape_recipe_time(element: ElementRef) -> Option<String> {
    // Grab the sibling of the element
    if let Some(sibling) = element.next_sibling() {
        let mut time = String::new();
        if let Some(sibling_element) = ElementRef::wrap(sibling) {
            time = sibling_element
                .text()
                .map(|s| s.trim())
                .collect::<BTreeSet<&str>>()
                .into_iter()
                .collect::<Vec<&str>>()
                .join(" ");
        }
        return Some(time);
    }
    None
}

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}
