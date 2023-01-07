mod algorithm;
mod controllers;
mod helper;
mod models;
mod router;
mod utils;

use axum::Router;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new()
        .nest("/", router::public_router())
        .nest("/jobs", router::jobs_router())
        .nest("/test", router::test_router())
}
