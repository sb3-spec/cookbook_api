use crate::{model::{init_db, RecipeMac, RecipePatch}, entities::recipe, security::{UserCtx, utx_from_token}};

#[tokio::test]
async fn model_recipe_create_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let data_fx = RecipePatch {
        title: Some("Baby Dip".to_string()),
        header: Some("This baby dip is sure to ruin your life".to_string()),
        steps: Some(vec!["Pet the baby to prep him".to_string(), "Incorporate him into the dip".to_string()]),
        ingredients: Some(vec!["Baby's Boy".to_string(), "Baby's Joy".to_string()]),
        tags: Some(vec!["Nast".to_string()]),
        image_url: Some("".to_string()),
        cook_time: Some("".to_string()),
    };
    let utx = utx_from_token(&db, "firebase_auth_123").await?;

    // -- ACTION
    let recipe = RecipeMac::create(&db, data_fx.clone(), utx).await?;

    let recipes = RecipeMac::list(&db).await?;

    // -- CHECK

    assert!(recipe.id <= 1000, "Id should be less than 1000");
    assert_eq!(recipes.len(), 4);
    assert_eq!(recipe.title, data_fx.title.unwrap());
    assert_eq!(recipe.header.unwrap(), data_fx.header.unwrap());

    Ok(())
}

#[tokio::test]
async fn model_recipe_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    // -- ACTION
    let recipes: Vec<recipe::Model> = RecipeMac::list(&db).await?;

    // -- CHECK
    assert_eq!(recipes.len(), 3);
    assert_eq!(recipes[0].id, 1);

    Ok(())
}

#[tokio::test]
async fn model_recipe_delete() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let utx = UserCtx { user_id: "test_delete".to_owned()};

    // -- ACTION
    let deleted_id = RecipeMac::delete(&db, utx, 3).await?;

    let recipes = RecipeMac::list(&db).await?;

    // -- CHECK
    assert_eq!(recipes.len(), 2);
    assert_eq!(deleted_id, 3);

    Ok(())
}

#[tokio::test]
async fn model_recipe_update() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let data_fx = RecipePatch {
        title: Some("Baby Dip".to_string()),
        header: Some("This baby dip is sure to ruin your life".to_string()),
        steps: Some(vec!["Pet the baby to prep him".to_string(), "Incorporate him into the dip".to_string()]),
        ingredients: Some(vec!["Baby's Boy".to_string(), "Baby's Joy".to_string()]),
        tags: Some(vec!["Nast".to_string()]),
        image_url: Some("".to_string()),
        cook_time: Some("3 hours".to_string())
    };
    let utx = utx_from_token(&db, "firebase_auth_123").await?;

    // -- ACTION
    let recipe = RecipeMac::create(&db, data_fx.clone(), utx).await?;

    let recipes = RecipeMac::list(&db).await?;

    // -- CHECK

    assert!(recipe.id <= 1000, "Id should be less than 1000");
    assert_eq!(recipes.len(), 4);
    assert_eq!(recipe.title, data_fx.title.unwrap());
    assert_eq!(recipe.header.unwrap(), data_fx.header.unwrap());

    Ok(())
}

#[tokio::test]
async fn model_recipe_get_by_tags_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let utx: UserCtx = utx_from_token(&db, "firebase_auth_123").await?;

    // -- ACTION
    let recipes = RecipeMac::get_by_tag(&db, utx, "Easy").await?;

    // -- CHECK
    assert_eq!(recipes.len(), 2);

    Ok(())
}


#[tokio::test]
async fn model_recipe_get() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    // -- ACTION
    let recipe = RecipeMac::get(&db, 1).await?;

    // -- CHECK
    assert_eq!(recipe.id, 1);
    assert_eq!(recipe.title, "Hunter`s Stew");

    Ok(())
}