use sea_orm::sea_query::{any, PgFunc, Expr};
use sea_orm::{DatabaseConnection, Set, prelude::*};
use serde::{Serialize, Deserialize};

use crate::entities::{recipe, prelude::Recipe};

use crate::model::Error;
use crate::security::UserCtx;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RecipePatch {
    pub title: Option<String>,
    pub header: Option<String>,
    pub steps: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub cook_time: Option<String>,
    pub prep_time: Option<String>,
    pub total_time: Option<String>,
}



pub struct RecipeMac;

impl RecipeMac {
    pub async fn create(db: &DatabaseConnection, data: RecipePatch, utx: UserCtx) -> Result<recipe::Model, super::Error> {
        let recipe = recipe::ActiveModel {
            cid: Set(utx.user_id),
            title: Set(data.title.unwrap().into()),
            header: Set(Some(data.header.unwrap_or_default())),
            steps: Set(Some(data.steps.unwrap_or_default())),
            ingredients: Set(Some(data.ingredients.unwrap_or_default())),
            tags: Set(Some(data.tags.unwrap_or_default())),
            image_url: Set(Some(data.image_url.unwrap_or_default())),
            cook_time: Set(Some(data.cook_time.unwrap_or_default())),
            prep_time: Set(Some(data.prep_time.unwrap_or_default())),
            total_time: Set(Some(data.total_time.unwrap_or_default())),
            ..Default::default()
        }; 

        let recipe: recipe::Model = recipe.insert(db).await?;

        Ok(recipe)
    }

    pub async fn list(db: &DatabaseConnection) -> Result<Vec<recipe::Model>, super::Error> {
        let recipes: Vec<recipe::Model> = Recipe::find().all(db).await?;

        Ok(recipes)
    }

    pub async fn update(db: &DatabaseConnection, data: RecipePatch, utx: UserCtx, id: i64) -> Result<recipe::Model, super::Error> {
        let recipe: Option<recipe::Model> = Recipe::find_by_id(id).one(db).await?;

        let mut recipe: recipe::ActiveModel = recipe.unwrap().into();

        recipe.title = Set(data.title.unwrap_or_else(|| recipe.title.unwrap()));
        recipe.mid = Set(Some(utx.user_id));
        recipe.header = Set(Some(data.header.unwrap_or_else(|| recipe.header.unwrap().unwrap())));
        recipe.ingredients = Set(Some(data.ingredients.unwrap_or_else(|| recipe.ingredients.unwrap().unwrap())));
        recipe.steps = Set(Some(data.steps.unwrap_or_else(|| recipe.steps.unwrap().unwrap())));
        recipe.tags = Set(Some(data.tags.unwrap_or_else(|| recipe.tags.unwrap().unwrap())));
        recipe.cook_time = Set(Some(data.cook_time.unwrap_or_else(|| recipe.cook_time.unwrap().unwrap())));
        recipe.prep_time = Set(Some(data.prep_time.unwrap_or_else(|| recipe.prep_time.unwrap().unwrap())));
        recipe.total_time = Set(Some(data.total_time.unwrap_or_else(|| recipe.total_time.unwrap().unwrap())));


        let recipe: recipe::Model = recipe.update(db).await?;

        Ok(recipe)
    }

    pub async fn delete(db: &DatabaseConnection, _utx: UserCtx, id: i64) -> Result<i64, super::Error> {
        let recipe: Option<recipe::Model> = Recipe::find_by_id(id).one(db).await?;
        let recipe: recipe::Model = recipe.unwrap();

        let id = recipe.id;
        recipe.delete(db).await?;

        Ok(id)
    }

    pub async fn get(db: &DatabaseConnection, id: i64) -> Result<recipe::Model, super::Error> {
        let recipe: Option<recipe::Model> = Recipe::find_by_id(id).one(db).await?;
        let recipe: recipe::Model = recipe.unwrap();

        Ok(recipe)
    }

    pub async fn get_by_tag(db: &DatabaseConnection, utx: UserCtx, tag: &str) -> Result<Vec<recipe::Model>, super::Error> {
        let recipes: Vec<recipe::Model> = Recipe::find()
            .filter(recipe::Column::Cid.eq(utx.user_id))
            .filter(Expr::eq(
                Expr::val(tag), 
                Expr::expr(PgFunc::any(Expr::col(recipe::Column::Tags)))
            ))
            .all(db)
            .await?;

        Ok(recipes)
    }

}


// region: Utils
fn handle_fetch_one_result(result: Result<recipe::Model, sea_orm::DbErr>, id: i64) -> Result<recipe::Model, super::Error> {
    result.map_err(|sea_orm_err| match sea_orm_err {
        sea_orm::DbErr::Query(RuntimeErr::SqlxError(sea_orm_err)) => super::Error::EntityNotFound(id.to_string()),
        other => super::Error::SeaOrmError(other)
    })
}

// endregion: Utils


#[cfg(test)]
#[path = "../_tests/model_recipe.rs"]
mod tests;