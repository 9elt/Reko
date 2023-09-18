use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde_json::json;
use structs::RekoError;

pub fn success<V: serde::ser::Serialize>(value: V) -> impl IntoResponse {
    (StatusCode::OK, Json(json!(value)))
}

pub fn error(err: RekoError) -> impl IntoResponse {
    let status = StatusCode::from_u16(err.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(json!(err))).into_response()
}
