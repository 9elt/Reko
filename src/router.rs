use axum::{Router, routing::get};

use super::controller;

pub fn router() -> axum::Router {
    Router::new()
        .route("/stats/:user", get(controller::get_user_model))
        .route("/recommendations/:user", get(controller::get_user_recommendations))

        .route("/jobs/compute_all_models", get(controller::compute_all_models))
        .route("/jobs/compute_std_dev", get(controller::compute_std_dev))
        .route("/jobs/update_old_users", get(controller::update_old_users))
}