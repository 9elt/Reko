#[macro_use]
mod util;
mod controller;

use axum::{http::Request, middleware::Next, response::Response, routing::get, Router};
use dotenvy::dotenv;
use reko::Reko;
use tower::builder::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let reko = Reko::default();

    println!("listening on 127.0.0.1:3000");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(router(reko).into_make_service())
        .await
        .unwrap();
}

fn router(reko: Reko) -> Router {
    let cors = ServiceBuilder::new().layer(CorsLayer::new().allow_origin(Any).allow_headers(Any));

    Router::new()
        .route("/version", get(controller::version))
        .route("/:user/similar", get(controller::get_similar_users))
        .route("/:user/compare/:other_user", get(controller::compare_users))
        .route(
            "/:user/recommendations",
            get(controller::get_recommendations),
        )
        .route("/:user/random", get(controller::get_random_recommendations))
        .route(
            "/:user/recommendations/:other_user",
            get(controller::get_recommendations_from),
        )
        .fallback(controller::not_found)
        .with_state(reko)
        .layer(cors)
        .layer(axum::middleware::from_fn(logger))
}

async fn logger<B>(request: Request<B>, next: Next<B>) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;
    let status = response.status();

    println!("{} -- {} {}", status, method, uri);

    response
}
