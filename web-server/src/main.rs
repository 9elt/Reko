mod controller;
mod util;

use reko::Reko;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use tower::builder::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let reko = Reko::new();

    println!("starting server at https://127.0.0.1:3000/health");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router(reko).into_make_service())
        .await
        .unwrap();
}

fn router(reko: Reko) -> Router {
    let cors = ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any).allow_headers(Any));

    Router::new()
        .route("/health", get(controller::health))
        .route("/similar/:user", get(controller::get_similar_users))
        .route(
            "/recommendations/:user",
            get(controller::get_recommendations),
        )
        .with_state(reko)
        .layer(cors)
}
