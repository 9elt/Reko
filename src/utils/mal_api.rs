use dotenvy::dotenv;
use std::env;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};

pub fn mal_headers() -> HeaderMap {
    dotenv().ok();

    let b = env::var("MAL_CLIENT_ID").expect("missing mal client id");
    let client_id = b.as_str();

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        HeaderName::from_static("x-mal-client-id"),
        HeaderValue::from_str(client_id).unwrap(),
    );
    headers
}