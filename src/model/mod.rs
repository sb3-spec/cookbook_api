use thiserror::Error as ThisError;

mod chef;
mod db;
mod recipe;

pub use chef::{ChefMac, ChefPatch};
pub use db::init_db;
pub use recipe::{RecipeMac, RecipePatch};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity with same firebase id already exists")]
    EntityAlreadyExists,

    #[error("Entity Not Found - {0}")]
    EntityNotFound(String),

    #[error(transparent)]
    SeaOrmErr(#[from] sea_orm::DbErr),

    #[error(transparent)]
    IOErr(#[from] std::io::Error),

    #[error(transparent)]
    SqlxErr(#[from] sqlx::Error),
}
