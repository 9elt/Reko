use dotenvy::dotenv;
use std::env;

pub fn jobs_auth_token() -> String {
    dotenv().ok();
    env::var("JOBS_AUTH_TOKEN").expect("missing jobs auth token")
}

pub fn test_auth_token() -> String {
    dotenv().ok();
    env::var("TEST_AUTH_TOKEN").expect("missing test auth token")
}