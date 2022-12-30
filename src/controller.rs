use axum::{extract::Path, extract::Query, http::header, response::IntoResponse, Json};
use hyper::StatusCode;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::algorithm::model::Model;
use crate::models;

use crate::helper;

#[derive(Deserialize)]
pub struct ModelQuery {
    reload: Option<bool>,
}

////////////////////////////////////////////////////////////////////////////////
// testing
////////////////////////////////////////////////////////////////////////////////

pub async fn get_user_model(
    Path(user): Path<String>,
    qry: Query<ModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match qry.0.reload {
        Some(val) => val,
        None => false,
    };

    println!("\n(\x1b[34m\x1b[1mGET\x1b[0m: model) user: \x1b[33m\x1b[1m{}\x1b[0m, reload: \x1b[33m\x1b[1m{}\x1b[0m", user, reload);

    match models::stats::get_user_model(&user, reload).await {
        Ok(model) => Ok(Json(json!(model))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}

pub async fn get_normal_dist() -> Result<Json<Value>, StatusCode> {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mcompute standard deviation\x1b[0m)");

    match helper::get_normal_dist() {
        Ok(v) => Ok(Json(json!(v))),
        Err(_) => Err(StatusCode::from_u16(500).unwrap()),
    }
}

pub async fn svg(Path(user): Path<String>) -> impl IntoResponse {
    let model = match models::stats::get_user_model(&user, false).await {
        Ok(model) => model,
        Err(_) => Model::<i16>::empty(),
    };

    let svg = format!(
        "<svg x='0px' y='0px' viewBox='0 0 {} {}' xmlns='http://www.w3.org/2000/svg'>\
            <style>\
                svg{{\
                    display: block;\
                    margin-inline: auto;\
                    width: 50rem;\
                    padding: 1rem;\
                    stroke-linejoin: round;\
                    stroke-linecap: round;\
                }}\
                .blue{{fill: none; stroke: #36a1ff; stroke-width: 2}}\
                .axis{{fill: none; stroke: #0007; stroke-width: 0.5}}\
            </style>\
            <!--Airing Decades-->
            {}
            <!--Ratings-->
            {}
            <!--Series Length-->
            {}
            <!--Genres-->
            {}
            <!--Themes-->
            {}
            <!--Demographics-->
            {}
        </svg>",
        210, 400,
        path_from_model(&model, 1, 0.0,   0.0,   100.0, 100.0),
        path_from_model(&model, 2, 110.0, 0.0,   100.0, 100.0),
        path_from_model(&model, 3, 0.0,   100.0, 100.0, 100.0),
        path_from_model(&model, 4, 110.0, 100.0, 100.0, 100.0),
        path_from_model(&model, 6, 0.0,   200.0, 200.0, 100.0),
        path_from_model(&model, 8, 0.0,   300.0, 100.0, 100.0),
    );

    ([(header::CONTENT_TYPE, "image/svg+xml")], svg)
}

fn path_from_model(model: &Model<i16>, index: usize, x_offset: f32, y_offset: f32, chart_width: f32, chart_height: f32) -> String {
    let num_points = model[index].len() as f32;
    let x_spacing: f32 = chart_width / (num_points - 1.0);
    let heigths: Vec<f32> = model[index].iter().map(|x| (chart_height - (x[0] as f32 / 10.0))).collect();
    let mut path = format!("\
        <path class='axis' d='M{},{} {},{} {},{}'/>\
        <path class='blue' d='M",
        x_offset, y_offset + 10.0,
        x_offset, y_offset + chart_height,
        x_offset + chart_width - 10.0, y_offset + chart_height,
    );
    let mut x: f32 = 0.0;
    for y in heigths {
        path = format!("{path}{},{} ", x + x_offset, y + y_offset);
        x += x_spacing;
    }
    format!("{path}'/>")
}

////////////////////////////////////////////////////////////////////////////////
// public
////////////////////////////////////////////////////////////////////////////////

pub async fn get_user_recommendations(
    Path(user): Path<String>,
    qry: Query<ModelQuery>,
) -> Result<Json<Value>, StatusCode> {
    let user: String = user.to_lowercase();

    let reload: bool = match qry.0.reload {
        Some(val) => val,
        None => false,
    };

    println!("\n(\x1b[34m\x1b[1mGET\x1b[0m: recommendations) user: \x1b[33m\x1b[1m{}\x1b[0m, reload: \x1b[33m\x1b[1m{}\x1b[0m", user, reload);

    match models::recommendations::get_user_recommendations(&user, reload).await {
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

pub async fn update_old_users() {
    println!("\n(\x1b[34m\x1b[1mJOB\x1b[0m: \x1b[1mupdate old users\x1b[0m)");
}
