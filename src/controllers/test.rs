use axum::{extract::Query, extract::Path, http::header, response::IntoResponse, Json};

use hyper::StatusCode;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::models;

/// ### (*testing*) SVG generation on request
pub async fn svg(Path(_user): Path<String>) -> impl IntoResponse {
    let svg = "\
    <svg x='0px' y='0px' viewBox='0 0 55 71' xmlns='http://www.w3.org/2000/svg'>\
        <path d='M45.7,8.6H9.3C8.8,8.6,8.5,9,8.5,9.4v52.2c0,0.4,0.4,0.8,0.8,0.8h5.4c-0.3-1.2,0.1-2.7,1.8-3c-1.1-4.5-1.8-14.8,0.4-22
          c1.9-5.9,2.2-10.5,5.8-15.6c0-1.5,0-6.8,1.9-10c0,0,1.9,3.8,3.4,5.7c2.2-0.9,4.9-1.2,7.3,1.1c1.1-0.5,4-2,5.1-3
          c1.2-1.1-1.8,4.8-2.8,6.4c3,6.8-1.4,13.7-8.3,13.7c-1.8,0-1.9,0.1-1.9,0.1s-2.2,2.1,2.9,4.1c10.2,3.9,9.8,14,7.4,19
          c0.8,0,1.6,0.3,2.4,1.1c1,1.2,0.5,2.4,0.5,2.4h5.2c0.4,0,0.8-0.4,0.8-0.8V9.4C46.5,9,46.2,8.6,45.7,8.6z'/>
    </svg>";

    ([(header::CONTENT_TYPE, "image/svg+xml")], svg)
}

#[derive(Deserialize)]
pub struct UserModelQuery {
    reload: Option<bool>,
}

/// ### (*testing*) get user statistics
pub async fn get_user_model(
    Path(user): Path<String>,
    query: Query<UserModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match query.0.reload {
        Some(val) => val,
        None => false,
    };

    match models::stats::get_user_model(&user, &reload).await {
        Ok(model) => Ok(Json(json!(model))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}