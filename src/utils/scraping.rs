use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::json;
use warp::reply::Json;
use reqwest::get;

async fn scrape_recipe(url: String) -> String {
    let response = get(url).await.unwrap().text().await.unwrap();

    let html = Html::parse_document(&response);
    let meta_selector = Selector::parse("meta").unwrap();

    for element in html.select(&meta_selector) {
        println!("{:?}", element.value());
    }

    "Hello".to_owned()
}


fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({"data": data});
    Ok(warp::reply::json(&response))
}