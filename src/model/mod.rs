use thiserror::Error as ThisError;

mod db;
mod recipe;
mod chef;

pub use db::{init_db};
pub use recipe::{ RecipeMac, RecipePatch };
pub use chef::{ ChefMac, ChefPatch };

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity with same firebase id already exists")]
    EntityAlreadyExists,


    #[error("Entity Not Found - {0}")]
    EntityNotFound(String),

    #[error(transparent)]
    SeaOrmError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error)
}