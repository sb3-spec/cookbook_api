use std::{convert::Infallible, path::Path, sync::Arc};

use sea_orm::DatabaseConnection;

use serde_json::json;
use warp::{reject::MethodNotAllowed, Filter, Rejection, Reply};

use crate::{model, security, web::chef::chef_rest_filters};

use self::recipe::recipe_rest_filters;

mod chef;
mod filter_auth;
mod filter_utils;
mod recipe;

pub async fn start_web(
    web_folder: &str,
    web_port: u16,
    db: Arc<DatabaseConnection>,
) -> Result<(), Error> {
    // validate web_folder
    if !Path::new(web_folder).exists() {
        return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
    }

    let cors = warp::cors()
        .allow_origins([
            "http://localhost:8080",
            "http://localhost:5173",
            "https://digital-parsley.netlify.app",
        ])
        .allow_headers(vec!["X-Auth-Token", "Content-Type", "content-type"])
        .allow_methods(vec!["GET", "POST", "HEAD", "DELETE", "PATCH"]);

    // Apis
    let apis = recipe_rest_filters("api", db.clone()).or(chef_rest_filters("api", db.clone()));

    // Static content
    let content = warp::fs::dir(web_folder.to_string());

    let root_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}index.html", web_folder)));

    let static_site = root_index.or(content);

    // Combine all routes
    let routes = static_site.or(apis).with(cors).recover(handle_rejection);
    println!("Starting web server at 0.0.0.0:{}", web_port);
    warp::serve(routes).run(([0, 0, 0, 0], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = 405;
    let message;

    if err.is_not_found() {
        code = 404;
        message = "NOT FOUND";
    } else if let Some(e) = err.find::<WebErrorMessage>() {
        code = warp::http::StatusCode::BAD_REQUEST.into();
        message = &e.message;
    } else {
        message = "Unable to parse error"
    };
    // Print to server side
    println!("Error - {:?}", err);

    // TODO - Call log api for capture and store
    // Build user message
    let user_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.typ.to_string(),
        None => "Unhandled rejection".to_string(),
    };

    let result = json!({ "error": user_message });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error{"Fail authentication missing X-Auth-Token header."}]
    FailAuthMissingXAuth,
}

// region: Warp Custom Error
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}

impl From<security::Error> for warp::Rejection {
    fn from(other: security::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{:?}", other))
    }
}
// endregion: Warp Custom Error
