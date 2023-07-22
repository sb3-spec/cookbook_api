use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::json;
use warp::{reply::Json, Filter};

use crate::entities::prelude::Chef;
use crate::{
    entities::{chef, recipe},
    model::{ChefMac, ChefPatch},
    security::UserCtx,
};

use super::{filter_auth::do_auth, filter_utils::with_db};

pub fn chef_rest_filters(
    base_path: &'static str,
    db: Arc<DatabaseConnection>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let chefs_path = warp::path(base_path).and(warp::path("chefs"));
    let common = with_db(db.clone()).and(do_auth(db));

    // region: api paths

    // LIST chefs 'GET /chefs/'

    // let list = chefs_path
    //     .and(warp::get())
    //     .and(warp::path::end())
    //     .and(common.clone())
    //     .and_then(chef_list);

    // GET chef 'GET /chefs'
    let get = chefs_path
        .and(warp::get())
        .and(common.clone())
        .and_then(chef_get);

    // POST chef 'POST /chefs/

    let create = chefs_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json::<ChefPatch>())
        .and_then(chef_create);

    // PATCH chef 'PATCH /chefs/1'

    let update = chefs_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::body::json::<ChefPatch>())
        .and_then(chef_update);

    // DELETE chef 'DELETE /chefs/1'
    let delete = chefs_path
        .and(warp::delete())
        .and(common.clone())
        .and_then(chef_delete);

    let get_recipes = chefs_path
        .and(warp::get())
        .and(common)
        .and(warp::path("recipes"))
        .and_then(chef_get_recipes);

    get_recipes.or(create).or(update).or(delete).or(get)

    // endregion: api paths
}

// region: Chef API functions
async fn chef_list(db: Arc<DatabaseConnection>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let chefs: Vec<chef::Model> = ChefMac::list(&db).await?;

    json_response(chefs)
}

async fn chef_get(db: Arc<DatabaseConnection>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let chef: chef::Model = ChefMac::get(&db, utx.user_id).await?;

    json_response(chef)
}

async fn chef_delete(db: Arc<DatabaseConnection>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let deleted_chef_id: String = ChefMac::delete(&db, utx.user_id).await?;

    json_response(deleted_chef_id)
}

async fn chef_create(
    db: Arc<DatabaseConnection>,
    utx: UserCtx,
    patch: ChefPatch,
) -> Result<Json, warp::Rejection> {
    let chef: chef::Model = ChefMac::create(&db, patch).await?;

    json_response(chef)
}

async fn chef_update(
    db: Arc<DatabaseConnection>,
    utx: UserCtx,
    patch: ChefPatch,
) -> Result<Json, warp::Rejection> {
    let chef: chef::Model = ChefMac::update(&db, patch, utx.user_id).await?;

    json_response(chef)
}

async fn chef_get_recipes(
    db: Arc<DatabaseConnection>,
    utx: UserCtx,
) -> Result<Json, warp::Rejection> {
    let recipes: Vec<recipe::Model> = ChefMac::get_recipes(&db, utx.user_id).await?;

    json_response(recipes)
}

// endregion: Chef API functions

fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
#[path = "../_tests/web_chef.rs"]
mod tests;
