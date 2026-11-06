mod anime;
mod recommendations;
mod schema;
mod similar;
mod user;

use diesel::Connection;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncMysqlConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type DBConnectionPool = Pool<AsyncMysqlConnection>;
type DBConnection<'a> = PooledConnection<'a, AsyncMysqlConnection>;

#[derive(Clone)]
pub struct DBClient {
    url: String,
    connections: DBConnectionPool,
}

impl DBClient {
    pub async fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("missing env variable DATABASE_URL");

        let connections = DBConnectionPool::builder()
            .max_size(15)
            .build(AsyncDieselConnectionManager::new(&url))
            .await
            .expect("Cannot connect to database");

        Self { url, connections }
    }
    pub async fn run_migrations(&self) {
        let url = self.url.clone();

        let applied = tokio::task::spawn_blocking(move || {
            let mut conn = AsyncConnectionWrapper::<AsyncMysqlConnection>::establish(&url)
                .expect("Cannot connect to database");

            conn.run_pending_migrations(MIGRATIONS)
                .map(|versions| versions.iter().map(|v| v.to_string()).collect::<Vec<_>>())
                .expect("Failed to run database migrations")
        })
        .await
        .expect("Migrations task panicked");

        for migration in applied {
            println!("applied migration {migration}");
        }
    }
    async fn connect(&self) -> DBConnection<'_> {
        self.connections
            .get()
            .await
            .expect("Cannot get a database connection")
    }
}
