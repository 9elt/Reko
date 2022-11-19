mod api;
mod fetch;

use axum::Router;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> axum::Router {
    Router::new().nest("/", api::router::router())
}