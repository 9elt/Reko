use axum::{extract::Path, extract::Query, Json};
use hyper::StatusCode;
use serde_json::{json, Value};
use serde::{Deserialize};

use crate::models::user_model;

#[derive(Deserialize)]
pub struct ModelQuery {
    reload: Option<bool>
}

pub async fn get_user_model(
    Path(user): Path<String>,
    qry: Query<ModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match  qry.0.reload {
        Some(val) => val,
        None => false
    };

    println!("(\x1b[34m\x1b[1mGET\x1b[0m: model) user: \x1b[33m\x1b[1m{}\x1b[0m, reload: \x1b[33m\x1b[1m{}\x1b[0m", user, reload);

    match user_model::get_user_model(user, reload).await {
        Ok(model) => Ok(Json(json!(model))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}
