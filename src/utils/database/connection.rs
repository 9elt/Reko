use diesel::pg::PgConnection;

use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use dotenvy::dotenv;
use std::env;

use lazy_static;

type DatabaseConnection = Pool::<ConnectionManager<PgConnection>>;

fn pool() -> DatabaseConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    DatabaseConnection::builder()
        .max_size(15)
        .build(ConnectionManager::new(db_url))
        .expect("failed creating db connection pool")
}

lazy_static! {
    pub static ref POOL: DatabaseConnection = pool();
}