use sea_orm::{DatabaseConnection, ModelTrait, Set};
use serde::{Serialize, Deserialize};

use crate::entities::{ chef, prelude::*, recipe};

use sea_orm::prelude::*;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ChefPatch {
    pub username: Option<String>,
    pub firebase_id: Option<String>
}

pub struct ChefMac;


impl ChefMac {
    pub async fn get(db: &DatabaseConnection, firebase_id: String) -> Result<chef::Model, super::Error> {
        // let chef: Option<chef::Model> = Chef::find_by_id(id).one(db).await?;
        let chef: Option<chef::Model> = Chef::find().filter(chef::Column::FirebaseId.eq(&firebase_id)).one(db).await?;

        if chef.is_none() {
            return Err(super::Error::EntityNotFound(firebase_id))
        }

        let chef = chef.unwrap();

        Ok(chef)    
    }

    pub async fn delete(db: &DatabaseConnection, id: String) -> Result<String, super::Error> {
        let chef: chef::Model = ChefMac::get(db, id).await?;

        chef.delete(db).await?;

        Ok("User successfully deleted".to_owned())
    }

    pub async fn list(db: &DatabaseConnection) -> Result<Vec<chef::Model>, super::Error> {
        let chefs: Vec<chef::Model> = Chef::find().all(db).await?;

        Ok(chefs)
    }

    pub async fn update(db: &DatabaseConnection, data: ChefPatch, id: String) -> Result<chef::Model, super::Error> {
        let chef: chef::Model = ChefMac::get(db, id).await?;
        let mut chef: chef::ActiveModel = chef.into();

        chef.username = Set(Some(data.username.unwrap_or_else(|| chef.username.unwrap().unwrap())));

        let chef: chef::Model = chef.update(db).await?;

        Ok(chef)
    }

    pub async fn create(db: &DatabaseConnection, data: ChefPatch) -> Result<chef::Model, super::Error> {
        let target_firebase_id = data.firebase_id.clone();
        let chef: Option<chef::Model> = Chef::find()
            .filter(chef::Column::FirebaseId.eq(target_firebase_id.unwrap()))
            .one(db)
            .await?;

        if chef.is_some() {
            return Err(super::Error::EntityAlreadyExists)
        } 

        let chef = chef::ActiveModel {
            username: Set(Some(data.username.unwrap_or_default())),
            firebase_id: Set(data.firebase_id.unwrap()),
            ..Default::default()
        };

        let chef: chef::Model = chef.insert(db).await?;

        Ok(chef)
    }

    pub async fn get_recipes(db: &DatabaseConnection, firebase_id: String) -> Result<Vec<recipe::Model>, super::Error> {

        let chef: chef::Model = ChefMac::get(db, firebase_id).await?;

        let recipes: Vec<recipe::Model> = chef.find_related(Recipe).all(db).await?;
        Ok(recipes)
    }

}

#[cfg(test)]
#[path = "../_tests/model_chef.rs"]
mod tests;