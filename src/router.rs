use axum::{Router, routing::get};

use super::controller;

pub fn router() -> axum::Router {
    Router::new()
        .route("/model/:user", get(controller::get_user_model))
}