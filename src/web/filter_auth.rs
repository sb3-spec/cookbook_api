use std::sync::Arc;

use sea_orm::DatabaseConnection;
use warp::{Rejection, Filter};

use crate::security::{UserCtx, utx_from_token};

use super::filter_utils::with_db;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn do_auth(db: Arc<DatabaseConnection>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
    warp::any()
        .and(with_db(db))
        .and(warp::header::optional::<String>(HEADER_XAUTH))
        .and_then(|db: Arc<DatabaseConnection>, xauth: Option<String>| async move {
            match xauth {
                Some(xauth) => {
                    let utx = utx_from_token(&db, &xauth).await?;
                    Ok::<UserCtx, Rejection>(utx)
                },
                None => Err(super::Error::FailAuthMissingXAuth.into())
            }
        })
}