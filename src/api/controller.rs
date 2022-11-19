use axum::{Json, extract::Path};
use crate::fetch;

use serde_json::{Value, json};

pub async fn get_user_recommendations(Path(user): Path<String>) -> Json<Value> {

    let s_user = user.to_lowercase();
    let list = fetch::fun::get_detailed_list(s_user, false).await;

    Json(json!(list))
}