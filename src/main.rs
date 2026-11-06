#[macro_use]
mod response;
mod controller;
mod database;
mod jobs;
mod myanimelist;
mod reko;
mod structs;
mod util;

use axum::{extract::Request, middleware::Next, response::Response, routing::get, Router};
use dotenvy::dotenv;
use reko::Reko;
use std::env;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let reko = Reko::new().await;

    reko.run_migrations().await;

    jobs::spawn(reko.clone());

    let addr = env::var("BIND_ADDR").unwrap_or("127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Cannot bind address");

    println!("listening on {addr}");

    axum::serve(listener, router(reko))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn router(reko: Reko) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    Router::new()
        .route("/version", get(controller::version))
        .route("/{user}/similar", get(controller::get_similar_users))
        .route(
            "/{user}/compare/{other_user}",
            get(controller::compare_users),
        )
        .route(
            "/{user}/recommendations",
            get(controller::get_recommendations),
        )
        .route(
            "/{user}/random",
            get(controller::get_random_recommendations),
        )
        .route(
            "/{user}/recommendations/{other_user}",
            get(controller::get_recommendations_from),
        )
        .fallback(controller::not_found)
        .with_state(reko)
        .layer(cors)
        .layer(axum::middleware::from_fn(logger))
}

async fn logger(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let response = next.run(request).await;

    println!("{} - {} {}", response.status(), method, uri);

    response
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Cannot install SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => {},
            _ = term.recv() => {},
        }
    }
    #[cfg(not(unix))]
    ctrl_c.await.ok();
}
