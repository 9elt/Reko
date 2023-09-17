mod schema;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use schema::{anime as table_anime, anime::dsl as anime};
use schema::{entries as table_entries, entries::dsl as entries};
use schema::{users as table_users, users::dsl as users};
use std::env;
use structs::Anime as PublicAnime;
use structs::DetailedListEntry as PublicDetailedListEntry;
use structs::ListEntry as PublicListEntry;
use structs::SimilarUser as PublicSimilarUser;
use structs::User as PublicUser;

type DBConnectionPool = Pool<ConnectionManager<MysqlConnection>>;
type DBConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

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
    pub fn insert_anime(&self, data: Vec<PublicAnime>) -> bool {
        let data = data
            .iter()
            .map(|ani| Anime::from_public(ani))
            .collect::<Vec<_>>();

        let mut conn = self.connect();

        diesel::insert_into(anime::anime)
            .values(data)
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_anime(&self, ids: Vec<i32>) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match anime::anime
            .filter(anime::id.eq_any(ids))
            .load::<Anime>(&mut conn)
        {
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
        let mut conn = self.connect();

        let res = diesel::insert_into(users::users)
            .values(UserInsert::from_public(user))
            .execute(&mut conn);

        if res.is_ok() {
            let uid = users::users
                .select(users::id)
                .order_by(users::id.desc())
                .first::<i32>(&mut conn)
                .unwrap();

            let data = etrs
                .iter()
                .map(|e| ListEntryInsert::from_public(uid, e))
                .collect::<Vec<_>>();

            diesel::insert_into(entries::entries)
                .values(data)
                .execute(&mut conn)
                .is_ok()
        } else {
            false
        }
    }
    pub fn update_user_entries(&self, user: &PublicUser, etrs: Vec<PublicListEntry>) -> bool {
        let mut conn = self.connect();

        let data = etrs
            .iter()
            .map(|e| ListEntryInsert::from_public(user.id, e))
            .collect::<Vec<_>>();

        let mut missing = Vec::new();

        for e in data {
            match diesel::update(entries::entries).set(&e).execute(&mut conn) {
                Ok(_) => continue,
                Err(_) => missing.push(e),
            };
        }

        if missing.len() > 0 {
            diesel::insert_into(entries::entries)
                .values(missing)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_nothing()
                .execute(&mut conn)
                .is_ok()
        } else {
            true
        }
    }
    pub fn get_user_entries(
        &self,
        user: &PublicUser,
        limit: usize,
    ) -> Vec<PublicDetailedListEntry> {
        let mut conn = self.connect();

        let raw: Vec<DetailedListEntry> = match anime::anime
            .inner_join(entries::entries.on(entries::id.eq(anime::id)))
            .select((anime::id, anime::stats, anime::mean, entries::score))
            .filter(entries::user.eq(user.id))
            .limit(limit as i64)
            .load::<DetailedListEntry>(&mut conn)
        {
            Ok(entries) => entries,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for e in raw {
            res.push(e.to_public());
        }

        res
    }
    pub fn get_user(&self, name: String) -> Option<PublicUser> {
        let mut conn = self.connect();

        match users::users
            .filter(users::username.eq(name))
            .first::<User>(&mut conn)
        {
            Ok(user) => Some(user.to_public()),
            Err(_) => None,
        }
    }
    pub fn update_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        diesel::update(users::users)
            .set(User::from_public(user))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_similar_users(&self, user: &PublicUser, page: u8) -> Vec<PublicSimilarUser> {
        let mut conn = self.connect();

        let raw = match diesel::sql_query(format!(
            "
        SELECT username, hash, BIT_COUNT({} ^ hash) distance
        FROM users
        WHERE username != '{}'
        ORDER BY distance ASC
        LIMIT 10 OFFSET {};
        ",
            user.hash,
            user.username,
            page * 10
        ))
        .load::<SimilarUser>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return Vec::new()
            },
        };

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
        }

        res
    }
}

use diesel::sql_types as sql;

#[derive(QueryableByName)]
struct SimilarUser {
    #[diesel(sql_type = sql::VarChar)]
    username: String,
    #[diesel(sql_type = sql::Unsigned<sql::Bigint>)]
    hash: u64,
    #[diesel(sql_type = sql::Integer)]
    distance: i32,
}

impl SimilarUser {
    fn to_public(self) -> PublicSimilarUser {
        PublicSimilarUser {
            username: self.username,
            hash: self.hash,
            distance: self.distance,
        }
    }
}

#[derive(Queryable)]
struct DetailedListEntry {
    id: i32,
    stats: String,
    mean: Option<f32>,
    score: i32,
}

impl DetailedListEntry {
    fn to_public(self) -> PublicDetailedListEntry {
        PublicDetailedListEntry {
            id: self.id,
            stats: serde_json::from_str::<Vec<i32>>(&self.stats).unwrap_or(Vec::new()),
            score: self.score,
            mean: self.mean.unwrap_or(0.0) as i32,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_users)]
struct User {
    id: i32,
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_users)]
struct UserInsert {
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
struct ListEntry {
    id: i32,
    user: i32,
    anime: Option<i32>,
    score: i32,
    watched: bool,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_entries)]
struct ListEntryInsert {
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
struct Anime {
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
