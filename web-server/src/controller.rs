use super::util::success;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use hyper::StatusCode;
use reko::Reko;
use serde::Deserialize;

pub async fn get_similar_users(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let user = reko
        .get_user(&user, query.force_update.unwrap_or(false), false)
        .await?;

    match reko.get_similar_users(&user, query.page.unwrap_or(1)) {
        Ok(res) => Ok(success(res)),
        Err(err) => Err(err),
    }
}

pub async fn get_recommendations(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let user = reko
        .get_user(&user, query.force_update.unwrap_or(false), false)
        .await?;

    match reko.get_recommendations(&user, query.page.unwrap_or(1)) {
        Ok(res) => Ok(success(res)),
        Err(err) => Err(err),
    }
}

pub async fn compare_users(
    Path(cmp): Path<ComparePath>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let user = reko.get_user(&cmp.user, false, false).await?;

    let other_user = reko.get_user(&cmp.other_user, false, false).await?;

    match reko.compare_users(&user, &other_user) {
        Ok(res) => Ok(success(res)),
        Err(err) => Err(err),
    }
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Reko API is up and running")
}

#[derive(Deserialize)]
pub struct ComparePath {
    user: String,
    other_user: String,
}

#[derive(Deserialize)]
pub struct GenericQuery {
    page: Option<i32>,
    force_update: Option<bool>,
}
