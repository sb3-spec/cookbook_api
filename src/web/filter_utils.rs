use std::{convert::Infallible, sync::Arc};

use sea_orm::DatabaseConnection;
use warp::Filter;

pub fn with_db(db: Arc<DatabaseConnection>) -> impl Filter<Extract = (Arc<DatabaseConnection>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}