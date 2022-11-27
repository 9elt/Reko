use crate::data::db::user::get_model;
use crate::model::base::generate_base_model;
use axum::{extract::Path, Json};

use hyper::StatusCode;
use serde_json::{json, Value};

pub async fn get_user_recommendations(Path(user): Path<String>) -> Result<Json<Value>, StatusCode> {
    let s_user = user.to_lowercase();

    let check = get_model(&s_user);

    let reload = false;

    if reload == false {
        match check {
            Ok(o) => match o {
                Some(m) => {
                    println!("{} model retrieved", user);
                    return Ok(Json(json!(m)))
                },
                None => (),
            },
            Err(_) => (),
        };
    }

    let model = generate_base_model(s_user, reload).await;
    match model {
        Ok(m) => Ok(Json(json!(m))),
        Err(e) => Err(StatusCode::from_u16(e).unwrap()),
    }
}
