use crate::model::base::generate_base_model;
use axum::{extract::Path, Json};

use hyper::StatusCode;
use serde_json::{json, Value};

pub async fn get_user_recommendations(Path(user): Path<String>) -> Result<Json<Value>, StatusCode> {
    let s_user = user.to_lowercase();

    let model = generate_base_model(s_user, false).await;

    match model {
        Ok(m) => Ok(Json(json!(m))),
        Err(e) => Err(StatusCode::from_u16(e).unwrap())
    }

}
