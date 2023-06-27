#![allow(unused)]

use std::{sync::Arc, env};

use model::init_db;
use web::start_web;

mod entities;
mod model;
mod web;
mod security;
mod utils;

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;


#[tokio::main]
async fn main() {
    // compute the web_folder
    let mut args: Vec<String> = env::args().collect();
    let web_folder = DEFAULT_WEB_FOLDER.to_string(); //args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());

    let web_port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_WEB_PORT
    };

    // get the database
    // TODO - loop until valid DB
    let db = init_db().await.expect("Cannot init db");
    let db = Arc::new(db);

    // Start the server
    match start_web(&web_folder, web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("ERROR - web server failed to start. Cause {:?}", ex)
    }
}
