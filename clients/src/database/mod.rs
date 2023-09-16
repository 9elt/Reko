mod schema;

use diesel::prelude::*;
use std::env;

fn connect() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("missing env variable DATABASE_URL");

    MysqlConnection::establish(&database_url).expect("Cannot connect to database")
}
