use super::util::{error, success};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use hyper::StatusCode;
use reko::Reko;
use serde::Deserialize;
use structs::{Data, PaginatedResponse, RequestingUser, Response};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_similar_users(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);

    println!("GET /{}/similar?page={}", &user, page);

    let user = unwrap!(reko.get_user(&user, false, false).await);

    let (result, pagination) = unwrap!(reko.get_similar_users(&user, page));

    response!(PaginatedResponse {
        requester: RequestingUser::from_user(&user),
        data: Data::Similar(result),
        pagination,
    })
}

pub async fn get_recommendations(
    Path(user): Path<String>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);
    let batch = query.batch.unwrap_or(1);

    println!(
        "GET /{}/recommendations?page={}&batch={}",
        &user, page, batch
    );

    let user = unwrap!(reko.get_user(&user, false, false).await);

    let (result, pagination) = unwrap!(reko.get_recommendations(&user, page, batch));

    response!(PaginatedResponse {
        requester: RequestingUser::from_user(&user),
        data: Data::Recommendation(result),
        pagination,
    })
}

pub async fn get_recommendations_from(
    Path(users): Path<Users>,
    Query(query): Query<GenericQuery>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1);

    println!(
        "GET /{}/recommendations/{}?page={}",
        &users.user, &users.other_user, page
    );

    let user = unwrap!(reko.get_user(&users.user, false, false).await);
    let other_user = unwrap!(reko.get_user(&users.other_user, false, true).await);

    let (result, pagination) = unwrap!(reko.get_recommendations_from(&user, &other_user, page));

    response!(PaginatedResponse {
        requester: RequestingUser::from_user(&user),
        data: Data::RecommendationFrom(result),
        pagination,
    })
}

pub async fn compare_users(
    Path(users): Path<Users>,
    State(reko): State<Reko>,
) -> impl IntoResponse {
    println!("GET /{}/compare/{}", &users.user, &users.other_user);

    let user = unwrap!(reko.get_user(&users.user, false, false).await);
    let other_user = unwrap!(reko.get_user(&users.other_user, false, true).await);

    let result = reko.compare_users(&user, &other_user);

    response!(Response {
        requester: RequestingUser::from_user(&user),
        data: Data::Compare(result),
    })
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
    batch: Option<i32>,
    // legacy
    #[allow(dead_code)]
    force_update: Option<bool>,
}
