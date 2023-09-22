mod anime;
mod recommendations;
mod schema;
mod similar;
mod user;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

type DBConnectionPool = Pool<ConnectionManager<MysqlConnection>>;
type DBConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct DBClient {
    connections: DBConnectionPool,
}

impl DBClient {
    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("missing env variable DATABASE_URL");

        Self {
            connections: DBConnectionPool::builder()
                .max_size(15)
                .build(ConnectionManager::new(url))
                .expect("Cannot connect to database"),
        }
    }
    fn connect(&self) -> DBConnection {
        self.connections.get().unwrap()
    }
}
