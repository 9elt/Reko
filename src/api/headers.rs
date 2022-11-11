use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
//use serde_json;
use std::env;

pub fn mal_headers() -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(
        HeaderName::from_static("x-mal-client-id"),
        HeaderValue::from_static(env!("MAL_CLIENT_ID", "missing MAL client id")),
    );
    headers
}