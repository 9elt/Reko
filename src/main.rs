mod api;
mod fetch;
mod model;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> axum::Router {
    let cors = CorsLayer::new().allow_origin(Any);
    Router::new().nest("/", api::router::router()).layer(cors)
}
