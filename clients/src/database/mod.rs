mod schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

use schema::{
    anime as table_anime,
    anime::dsl::{anime, id as anime_id},
};

use schema::{
    users as table_users,
    users::dsl::{id as user_id, username, users},
};

use schema::{
    entries as table_entries,
    entries::dsl::{anime as rel_anime, entries, id as entry_id},
};

use std::env;
use structs::Anime as PublicAnime;
use structs::ListEntry as PublicListEntry;
use structs::User as PublicUser;

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
    pub fn insert_anime(&self, data: Vec<PublicAnime>) -> bool {
        let data = data
            .iter()
            .map(|ani| Anime::from_public(ani))
            .collect::<Vec<_>>();

        let mut conn = self.connections.get().unwrap();

        diesel::insert_into(anime)
            .values(data)
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_anime(&self, mut ids: Vec<i32>) -> Vec<PublicAnime> {
        let mut conn = self.connections.get().unwrap();

        let mut query = anime.into_boxed().filter(anime_id.eq(ids.pop().unwrap()));
        for i in ids {
            query = query.or_filter(anime_id.eq(i));
        }

        let raw: Vec<Anime> = match query.load::<Anime>(&mut conn) {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for ani in raw {
            res.push(ani.to_public());
        }

        res
    }
    pub fn insert_user(&self, user: &PublicUser, etrs: Vec<PublicListEntry>) -> bool {
        let mut conn = self.connections.get().unwrap();

        let res = diesel::insert_into(users)
            .values(UserInsert::from_public(user))
            .execute(&mut conn);

        if res.is_ok() {
            let uid = users
                .select(user_id)
                .order_by(user_id.desc())
                .first::<i32>(&mut conn)
                .unwrap();

            let data = etrs
                .iter()
                .map(|e| ListEntryInsert::from_public(uid, e))
                .collect::<Vec<_>>();

            diesel::insert_into(entries)
                .values(data)
                .execute(&mut conn)
                .is_ok()
        } else {
            false
        }
    }
    pub fn get_user(&self, name: String) -> Option<PublicUser> {
        let mut conn = self.connections.get().unwrap();

        match users.filter(username.eq(name)).first::<User>(&mut conn) {
            Ok(user) => Some(user.to_public()),
            Err(_) => None,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_users)]
pub struct User {
    id: i32,
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_users)]
pub struct UserInsert {
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

impl UserInsert {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            username: user.username.to_owned(),
            hash: user.hash,
            updated_at: user.updated_at,
        }
    }
}

impl User {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            id: user.id,
            username: user.username.to_owned(),
            hash: user.hash,
            updated_at: user.updated_at,
        }
    }
    fn to_public(self) -> PublicUser {
        PublicUser {
            id: self.id,
            username: self.username,
            hash: self.hash,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_entries)]
pub struct ListEntry {
    id: i32,
    user: i32,
    anime: Option<i32>,
    score: i32,
    watched: bool,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_entries)]
pub struct ListEntryInsert {
    user: i32,
    anime: i32,
    score: i32,
    watched: bool,
    updated_at: NaiveDateTime,
}

impl ListEntryInsert {
    fn from_public(usr_id: i32, entry: &PublicListEntry) -> Self {
        Self {
            user: usr_id,
            anime: entry.id,
            score: entry.score,
            watched: entry.watched,
            updated_at: entry.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_anime)]
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
    fn from_public(ani: &PublicAnime) -> Self {
        Self {
            id: ani.id,
            title: ani.title.to_owned(),
            airing_date: ani.airing_date,
            length: ani.length,
            mean: ani.mean,
            rating: ani.rating.to_owned(),
            picture: ani.picture.to_owned(),
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
