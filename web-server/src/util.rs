use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;

pub fn success<V: serde::ser::Serialize>(value: V) -> impl IntoResponse {
    (StatusCode::OK, Json(json!(value)))
}
