use axum::{Router, routing::get};

use super::controller;

pub fn router() -> axum::Router {
    Router::new()
        .route("/stats/:user", get(controller::get_user_model))
        .route("/recommendations/:user", get(controller::get_user_recommendations))
}