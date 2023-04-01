use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::json;
use warp::{Filter, reply::Json};

use crate::{model::{RecipeMac, RecipePatch}, security::UserCtx};

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

    list.or(get).or(create).or(delete).or(update).or(get_by_tag)
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

// region: Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}

// endregion: Utils

#[cfg(test)]
#[path = "../_tests/web_recipe.rs"]
mod tests;