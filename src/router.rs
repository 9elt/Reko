use super::controllers;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use tower::builder::ServiceBuilder;
use tower_http::auth::RequireAuthorizationLayer;
use crate::utils::bearer;

////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////
pub fn public_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any));

    Router::new()
        .route("/health", get(controllers::public::health))
        .route("/recommendations/:user", post(controllers::public::get_user_recommendations))
        .layer(middleware)
}

////////////////////////////////////////////////////////////////////////////////
// Jobs
////////////////////////////////////////////////////////////////////////////////
pub fn jobs_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .layer(RequireAuthorizationLayer::bearer(bearer::jobs_auth_token().as_str()));

    Router::new()
        .route("/compute_all_models", get(controllers::jobs::compute_all_models))
        .route("/compute_normal_dist", get(controllers::jobs::compute_normal_dist))
        .route("/update_old_users", get(controllers::jobs::update_old_users))
        .route("/update_airing_anime", get(controllers::jobs::update_airing_anime))
        .layer(middleware)
}

////////////////////////////////////////////////////////////////////////////////
// Testing
////////////////////////////////////////////////////////////////////////////////
pub fn test_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any))
        .layer(RequireAuthorizationLayer::bearer(bearer::test_auth_token().as_str()));

    Router::new()
        .route("/stats/:user", get(controllers::test::get_user_model))
        .route("/stats/:user/graph.svg", get(controllers::test::svg))
        .layer(middleware)
}
