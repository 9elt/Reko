use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenvy::dotenv;
use std::env;

pub fn establish() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
