use axum::{extract::Path, extract::Query, Json};
use hyper::StatusCode;

use serde_json::{json, Value};
use serde::Deserialize;

use crate::models;

use crate::helper;

#[derive(Deserialize)]
pub struct ModelQuery {
    reload: Option<bool>
}

////////////////////////////////////////////////////////////////////////////////
// testing
////////////////////////////////////////////////////////////////////////////////

pub async fn get_user_model(
    Path(user): Path<String>,
    qry: Query<ModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match  qry.0.reload {
        Some(val) => val,
        None => false
    };

    println!("\n(\x1b[34m\x1b[1mGET\x1b[0m: model) user: \x1b[33m\x1b[1m{}\x1b[0m, reload: \x1b[33m\x1b[1m{}\x1b[0m", user, reload);

    match models::stats::get_user_model(&user, reload).await {
        Ok(model) => Ok(Json(json!(model.to_vec()))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

pub async fn get_normal_dist() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute standard deviation\x1b[0m)");

    match helper::get_normal_dist() {
        Ok(v) => Ok(Json(json!(v))),
        Err(_) => Err(StatusCode::from_u16(500).unwrap())
    }
}

////////////////////////////////////////////////////////////////////////////////
// public
////////////////////////////////////////////////////////////////////////////////

pub async fn get_user_recommendations(
    Path(user): Path<String>,
    qry: Query<ModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match  qry.0.reload {
        Some(val) => val,
        None => false
    };

    println!("\n(\x1b[34m\x1b[1mGET\x1b[0m: recommendations) user: \x1b[33m\x1b[1m{}\x1b[0m, reload: \x1b[33m\x1b[1m{}\x1b[0m", user, reload);

    match models::recommendations::get_user_recommendations(&user, reload) {
        Ok(users) => Ok(Json(json!(users))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

////////////////////////////////////////////////////////////////////////////////
// jobs
////////////////////////////////////////////////////////////////////////////////

pub async fn compute_all_models() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute all models\x1b[0m)");

    match models::jobs::compute_all_models().await {
        Ok(status) => Ok(Json(json!(status))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap())
    }
}

pub async fn compute_normal_dist() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute standard deviation\x1b[0m)");

    match models::jobs::compute_normal_dist().await {
        Ok(_) => Ok(Json(json!(vec![1]))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap())
    }
}

pub async fn update_old_users() {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mupdate old users\x1b[0m)");
}
