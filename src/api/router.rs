use axum::{Router, routing::get};

use super::controller;

pub fn router() -> axum::Router {
    Router::new()
        .route("/user/:user", get(controller::get_user_recommendations))
}