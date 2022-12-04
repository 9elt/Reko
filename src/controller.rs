use axum::{extract::Path, extract::Query, Json};
use hyper::StatusCode;
use serde_json::{json, Value};

use crate::models::user_model;

pub async fn get_user_model(
    Path(user): Path<String>,
    reload: Query<bool>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();
    let reload: bool = reload.0;
    match user_model::get_user_model(user, reload).await {
        Ok(model) => Ok(Json(json!(model))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}
