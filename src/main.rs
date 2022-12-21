mod algorithm;
mod controller;
mod helper;
mod models;
mod router;
mod utils;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> axum::Router {
    let cors = CorsLayer::new().allow_origin(Any);
    Router::new().nest("/", router::router()).layer(cors)
}
