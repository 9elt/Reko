use diesel::prelude::*;
use super::schema::anime;
use super::schema::lists;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = anime)]
pub struct AnimeDB {
    pub id: i32,
    pub title: String,
    pub picture: Option<String>,
    pub mean: Option<i16>,
    pub airing_date: Option<chrono::NaiveDate>,
    pub airing_status: Option<i16>,
    pub num_episodes: Option<i16>,
    pub rating: Option<i16>,
    pub genres: Option<Vec<Option<i16>>>,
    pub related: Option<serde_json::Value>,
}

#[derive(Queryable, Insertable, Debug, Clone, AsChangeset)]
#[diesel(table_name = lists)]
pub struct ListsDB {
    pub user_hash: String,
    pub list: serde_json::Value,
    pub updated_at: chrono::NaiveDateTime,
}