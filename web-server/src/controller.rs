use super::util::{error, success};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use hyper::StatusCode;
use reko::Reko;
use serde::Deserialize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_similar_users(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);

    println!("GET /{}/similar?page={}", &user, page);

    let user = unwrap!(reko.get_user(&user, false, false).await);

    response!(reko.get_similar_users(&user, page))
}

pub async fn get_recommendations(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);

    println!("GET /{}/recommendations?page={}", &user, page);

    let user = unwrap!(reko.get_user(&user, false, false).await);

    response!(reko.get_recommendations(&user, query.page.unwrap_or(1)))
}

pub async fn compare_users(
    Path(users): Path<Users>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    println!("GET /{}/compare/{}", &users.user, &users.other_user);

    let user = unwrap!(reko.get_user(&users.user, false, false).await);
    let other_user = unwrap!(reko.get_user(&users.other_user, false, false).await);

    response!(reko.compare_users(&user, &other_user))
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Resource not found")
}

pub async fn version() -> impl IntoResponse {
    VERSION
}

#[derive(Deserialize)]
pub struct Users {
    user: String,
    other_user: String,
}

#[derive(Deserialize)]
pub struct GenericQuery {
    page: Option<i32>,
    // legacy
    #[allow(dead_code)]
    force_update: Option<bool>,
}
