use std::{str::from_utf8, sync::Arc};


use anyhow::{ Result, Context };
use serde::Deserialize;
use serde_json::{from_value, from_str, Value, json};

use warp::{hyper::{ Response, body::Bytes }, Filter};

use crate::{model::{init_db, RecipeMac}, web::{handle_rejection, recipe_rest_filters}, entities::recipe};

#[tokio::test]
async fn web_recipe_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "1")
        .path("/api/recipes/")
        .reply(&recipe_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let recipes: Vec<recipe::Model> = extract_body_data(resp)?;

    // -- CHECK - recipes
    assert_eq!(3, recipes.len(), "number of recipes");
    assert_eq!(1, recipes[0].id);
    assert_eq!("Hunter`s Stew", recipes[0].title);

    Ok(())
}

#[tokio::test]
async fn web_recipe_get() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "1")
        .path("/api/recipes/1")
        .reply(&recipe_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let recipe: recipe::Model = extract_body_data(resp)?;

    // -- CHECK - .data (recipe)
    assert_eq!(1, recipe.id);
    assert_eq!("Hunter`s Stew", recipe.title);

    Ok(())
}

#[tokio::test]
async fn web_recipe_create_ok() -> Result<()> {
        // -- FIXTURE
        let db = init_db().await?;
        let db = Arc::new(db);
        let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

        let body = json!({
            "title": "Bag of Beans",
            "header": "Bean time!"
        });

        // -- ACTION
        let resp = warp::test::request()
            .method("POST")
            .header("X-Auth-Token", "firebase_auth_123")
            .path("/api/recipes")
            .json(&body)
            .reply(&recipe_apis)
            .await;
    
        // -- CHECK
        assert_eq!(200, resp.status(), "http status");
    
        // extract response.data
        let recipe: recipe::Model = extract_body_data(resp)?;
    
        // -- CHECK - .data (recipe)
        assert_eq!(4, recipe.id);
        assert_eq!("Bag of Beans", recipe.title);
    
        Ok(())
}

#[tokio::test]
async fn web_get_by_tag_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "firebase_auth_123")
        .path("/api/recipes/tags/Easy")
        .reply(&recipe_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");
    
    // extract response.data

    let recipes: Vec<recipe::Model> = extract_body_data(resp)?;

    // -- CHECK

    assert_eq!(2, recipes.len());

    Ok(())
}
#[tokio::test]
async fn web_recipe_delete_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "1")
        .path("/api/recipes/3")
        .reply(&recipe_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response.data
    let recipe_id: i64 = extract_body_data(resp)?;

    let recipes: Vec<recipe::Model> = RecipeMac::list(&db).await?;


    // -- CHECK
    assert_eq!(3, recipe_id, "recipe.id");
    assert_eq!(2, recipes.len(), "recipes length");

    Ok(())

}

#[tokio::test]
async fn web_recipe_update_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let recipe_apis = recipe_rest_filters("api", db.clone()).recover(handle_rejection);


    let body = json!({
        "title": "Stinky Baby Boys"
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "1")
        .path("/api/recipes/3")
        .json(&body)
        .reply(&recipe_apis)
        .await;

    // extract reponse.data

    let recipe: recipe::Model = extract_body_data(resp)?;

    // -- CHECK - status
    assert_eq!(3, recipe.id, "recipe.id");
    assert_eq!("Stinky Baby Boys", recipe.title);

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