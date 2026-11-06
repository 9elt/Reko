use crate::structs::RekoError;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use serde_json::json;

pub fn success<V: serde::ser::Serialize>(value: V) -> impl IntoResponse {
    (StatusCode::OK, Json(json!(value)))
}

pub fn error(err: RekoError) -> impl IntoResponse {
    let status = StatusCode::from_u16(err.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(json!(err))).into_response()
}

#[macro_export]
macro_rules! unwrap {
    ($result:expr) => {
        match $result {
            Ok(data) => data,
            Err(err) => return Err(error(err)),
        }
    };
}

#[macro_export]
macro_rules! response {
    ($data:expr) => {
        Ok(success($data))
    };
}
