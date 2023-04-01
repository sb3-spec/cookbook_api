use super::init_db;
use sea_orm::prelude::*;

use crate::entities::{recipe, chef, prelude::{Chef, Recipe}};


#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // ACTION
    let db = init_db().await?;

    let recipes: Vec<recipe::Model> = Recipe::find().all(&db).await?;

    let chefs: Vec<chef::Model> = Chef::find().all(&db).await?;

    assert_eq!(3, recipes.len());
    assert_eq!(1, chefs.len());



    Ok(())
}