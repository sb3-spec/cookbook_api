use std::{sync::Arc, str::from_utf8};
use crate::{web::{chef_rest_filters, handle_rejection}, entities::{chef, recipe}, model::ChefMac};
use anyhow::{Result, Context};
use serde::Deserialize;
use serde_json::{Value, from_value, from_str, json};
use warp::{hyper::{body::Bytes, Response}, Filter};

use crate::model::init_db;

#[tokio::test]
async fn web_chef_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "1")
        .path("/api/chefs")
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let mut chefs: Vec<chef::Model> = extract_body_data(resp)?;

    // -- CHECK - recipes
    assert_eq!(1, chefs.len(), "number of recipes");
    // assert_eq!(1, chefs[0].id);
    assert_eq!("Goombah!", chefs[0].username.as_ref().unwrap());

    Ok(())
}

#[tokio::test]
async fn web_chef_get() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "1")
        .path("/api/chefs/firebase_auth_123")
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let mut chef: chef::Model = extract_body_data(resp)?;

    // -- CHECK - recipes
    // assert_eq!(1, chef.id);
    assert_eq!("Goombah!", chef.username.as_ref().unwrap());

    Ok(())
}

#[tokio::test]
async fn web_chef_delete() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    let body = json!({
        "username": "Nasty Mary",
        "firebase_id": "firebase_auth_1234"
    });

    // -- ACTION
    warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "firebase_auth_1234 ")
        .path("/api/chefs")
        .json(&body)
        .reply(&chef_apis)
        .await;


    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "firebase_auth_1234")
        .path("/api/chefs")
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let deleted_chef_message: String = extract_body_data(resp)?;

    let chefs = ChefMac::list(&db).await?;

    // -- CHECK - recipes
    // assert_eq!(2, deleted_chef_id);
    assert_eq!(1, chefs.len());

    Ok(())
}

#[tokio::test]
async fn web_chef_create() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    let body = json!({
        "username": "Nasty Mary",
        "firebase_id": "firebase_auth_1234"
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "1")
        .path("/api/chefs")
        .json(&body)
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let mut chef: chef::Model = extract_body_data(resp)?;

    let chefs: Vec<chef::Model> = ChefMac::list(&db).await?;

    // -- CHECK - recipes
    assert_eq!(2, chefs.len(), "number of chefs");
    // assert_eq!(2, chefs[1].id);
    assert_eq!("Nasty Mary", chef.username.unwrap());

    Ok(())
}

#[tokio::test]
async fn web_chef_create_fail() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    let body = json!({
        "username": "Nasty Mary",
        "firebase_id": "firebase_auth_123"
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "firebase_auth_123")
        .path("/api/chefs")
        .json(&body)
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(400, resp.status(), "http status");

    Ok(())
}

#[tokio::test]
async fn web_chef_update() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    let body = json!({
        "username": "Nasty Mary",
        "firebase_id": "firebase_auth_1234",
    });

    warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "firebase_auth_1234")
        .path("/api/chefs")
        .json(&body)
        .reply(&chef_apis)
        .await;

    // -- ACTION
    let update_body = json!({
        "username": "Scary Mary"
    });

    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "firebase_auth_1234")
        .path("/api/chefs/firebase_auth_1234")
        .json(&update_body)
        .reply(&chef_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let mut chef: chef::Model = extract_body_data(resp)?;


    // -- CHECK - recipes
    // assert_eq!(2, chef.id);
    assert_eq!("Scary Mary", chef.username.unwrap());

    Ok(())
}


#[tokio::test]
async fn web_chef_get_recipes() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let chef_apis = chef_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "firebase_auth_123")
        .path("/api/chefs/recipes/firebase_auth_123")
        .reply(&chef_apis)
        .await;

    // extract the data
    let recipes: Vec<recipe::Model> = extract_body_data(resp)?;

    // -- CHECK
    assert_eq!(recipes.len(), 3);

    Ok(())
}


// region: Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where 
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = 
    from_str(body).with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

    // extract the data
    let data = body["data"].take();

    // deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}
// endregion: Web Test Utils