use super::controllers;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use tower::builder::ServiceBuilder;
use tower_http::auth::RequireAuthorizationLayer;

////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////
pub fn public_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
    //.layer(RequireAuthorizationLayer::bearer("token"))
    .layer(CorsLayer::new().allow_origin(Any).allow_headers(Any));

    Router::new()
        .route("/recommendations/:user", post(controllers::public::get_user_recommendations))
        .layer(middleware)
}

////////////////////////////////////////////////////////////////////////////////
// Jobs
////////////////////////////////////////////////////////////////////////////////
pub fn jobs_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
        .layer(RequireAuthorizationLayer::bearer("token"))
        .layer(CorsLayer::new().allow_origin(Any));

    Router::new()
        .route("/compute_all_models", get(controllers::jobs::compute_all_models))
        .route("/compute_normal_dist", get(controllers::jobs::compute_normal_dist))
        .route("/update_old_users", get(controllers::jobs::update_old_users))
        .layer(middleware)
}

////////////////////////////////////////////////////////////////////////////////
// Testing
////////////////////////////////////////////////////////////////////////////////
pub fn test_router() -> axum::Router {
    let middleware = ServiceBuilder::new()
        //.layer(RequireAuthorizationLayer::bearer("token"))
        .layer(CorsLayer::new().allow_origin(Any));

    Router::new()
        .route("/stats/:user", get(controllers::test::get_user_model))
        .route("/stats/:user/graph.svg", get(controllers::test::svg))
        .layer(middleware)
}
