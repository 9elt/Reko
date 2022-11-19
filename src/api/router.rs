use axum::{Router, routing::get};

use super::controller::get_user_recommendations;

pub fn router() -> axum::Router {
    Router::new()
        .route("/user/:user", get(get_user_recommendations))
}