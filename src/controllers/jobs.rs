use axum::Json;
use hyper::StatusCode;

use serde_json::{json, Value};

use crate::models;

pub async fn compute_all_models() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute all models\x1b[0m)");

    match models::jobs::compute_all_models().await {
        Ok(status) => Ok(Json(json!(status))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

pub async fn compute_normal_dist() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute standard deviation\x1b[0m)");

    match models::jobs::compute_normal_dist().await {
        Ok(_) => Ok(Json(json!(vec![1]))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

pub async fn update_old_users() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mupdate old users\x1b[0m)");

    match models::jobs::update_old_users().await {
        Ok(ok_err) => Ok(Json(json!(ok_err))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

pub async fn update_airing_anime() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mupdate old users\x1b[0m)");

    match models::jobs::update_airing_anime().await {
        Ok(ok_err) => Ok(Json(json!(ok_err))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}
