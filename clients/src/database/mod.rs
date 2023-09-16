mod schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use schema::{anime, anime::dsl::*};
use std::env;
use structs::Anime as PublicAnime;

type DBConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

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
    pub fn insert_anime(&self, data: PublicAnime) -> bool {
        let data = Anime::from_public(data);
        let mut conn = self.connections.get().unwrap();

        let res = diesel::insert_into(anime).values(data).execute(&mut conn);

        res.is_ok()
    }
    pub fn get_anime(&self, anime_id: i32) -> Result<PublicAnime, u16> {
        let mut conn = self.connections.get().unwrap();

        match anime.filter(id.eq(anime_id)).first::<Anime>(&mut conn) {
            Ok(res) => Ok(res.to_public()),
            Err(_) => Err(404),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = anime)]
pub struct Anime {
    id: i32,
    title: String,
    airing_date: Option<NaiveDateTime>,
    length: Option<i32>,
    mean: Option<f32>,
    rating: Option<String>,
    picture: Option<String>,
    aired: bool,
    stats: String,
    updated_at: NaiveDateTime,
    parent: Option<i32>,
}

impl Anime {
    fn from_public(ani: PublicAnime) -> Self {
        Self {
            id: ani.id,
            title: ani.title,
            airing_date: ani.airing_date,
            length: ani.length,
            mean: ani.mean,
            rating: ani.rating,
            picture: ani.picture,
            aired: ani.aired,
            stats: serde_json::json!(ani.stats).to_string(),
            updated_at: ani.updated_at,
            parent: None,
        }
    }
    fn to_public(self) -> PublicAnime {
        PublicAnime {
            id: self.id,
            title: self.title,
            airing_date: self.airing_date,
            length: self.length,
            mean: self.mean,
            rating: self.rating,
            picture: self.picture,
            aired: self.aired,
            stats: serde_json::from_str::<Vec<i32>>(&self.stats).unwrap_or(Vec::new()),
            updated_at: self.updated_at,
            prequels: Vec::new(),
        }
    }
}
