use sea_orm::Set;

use crate::{model::{init_db, ChefMac, ChefPatch}, entities::{chef, recipe}};


#[tokio::test]
async fn model_chef_get() -> Result<(), Box<dyn std::error::Error>> {

    // -- FIXTURE
    let db = init_db().await?;


    // -- ACTION
    let chef = ChefMac::get(&db, String::from("firebase_auth_123")).await?;

    // -- CHECK

    // assert_eq!(chef.id, 1);
    assert_eq!(chef.username.unwrap_or_default(), "Goombah!");
    assert_eq!(chef.firebase_id, "firebase_auth_123");

    Ok(())
    
}

#[tokio::test]
async fn model_chef_create_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = ChefPatch {
        username: Some("Scary Mary!".to_string()),
        firebase_id: Some("firebase_auth_1234".to_owned()),
    };

    // -- ACTION
    let new_chef = ChefMac::create(&db, data_fx).await?;

    // -- CHECK
    let chefs = ChefMac::list(&db).await?;

    assert_eq!(chefs.len(), 2);
    assert_eq!(chefs[1], new_chef);
    
    Ok(())
}

#[tokio::test]
async fn model_chef_get_recipes() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    // -- ACTION
    let recipes: Vec<recipe::Model> = ChefMac::get_recipes(&db, String::from("firebase_auth_123")).await?;

    // -- CHECK
    assert_eq!(recipes.len(), 3);
    assert_eq!(recipes[0].title, "Hunter`s Stew");

    Ok(())
}

#[tokio::test]
async fn model_chef_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    // -- ACTION
    let mut chefs: Vec<chef::Model> = ChefMac::list(&db).await?;

    // -- CHECK
    assert_eq!(chefs.len(), 1);
    assert_eq!(chefs[0].username.as_mut().unwrap(), "Goombah!");

    Ok(())
}

#[tokio::test]
async fn model_chef_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = ChefPatch {
        username: Some("Scary Mary!".to_string()),
        firebase_id: Some("firebase_auth_1234".to_owned()),
    };

    let new_chef = ChefMac::create(&db, data_fx).await?;
    

    // -- ACTION
    let update_data = ChefPatch {
        username: Some("Nasty Bug!".to_owned()),
        ..Default::default()
    };

    let chef = ChefMac::update(&db, update_data, "firebase_auth_1234".to_owned()).await?;

    // -- CHECK
    // assert_eq!(chef.id, new_chef.id);
    // assert_ne!(chef.username, new_chef.username);

    Ok(())
}

#[tokio::test]
async fn model_chef_delete() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = ChefPatch {
        username: Some("Scary Mary!".to_string()),
        firebase_id: Some("firebase_auth_1234".to_owned()),
    };

    let new_chef = ChefMac::create(&db, data_fx).await?;

    // -- ACTION
    let deleted_id: String = ChefMac::delete(&db, "firebase_auth_1234".to_owned()).await?;

    // -- CHECK

    let chefs = ChefMac::list(&db).await?;
    assert_eq!(chefs.len(), 1);
    // assert_eq!(deleted_id, new_chef.id);

    Ok(())
}

