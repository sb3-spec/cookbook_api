use thiserror::Error as ThisError;

use sea_orm::DatabaseConnection;

pub struct UserCtx {
    pub user_id: String,
}

pub async fn utx_from_token(_db: &DatabaseConnection, token: &str) -> Result<UserCtx, Error> {
    match token.parse::<String>() {
        Ok(user_id) => Ok(UserCtx { user_id }),
        Err(_) => Err(Error::InvalidToken(token.to_string()))
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String)
}