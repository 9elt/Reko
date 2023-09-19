use super::schema::{entries as table_entries, entries::dsl as entries};
use super::schema::{users as table_users, users::dsl as users};
use super::DBClient;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types as sql;
use structs::DetailedListEntry as PublicDetailedListEntry;
use structs::Hash;
use structs::ListEntry as PublicListEntry;
use structs::Recommendation as PublicRecommendation;
use structs::RecommendationDetails as PublicRecommendationDetails;
use structs::SimilarUser as PublicSimilarUser;
use structs::Stat;
use structs::User as PublicUser;
use util::{similarity, SHF_HASH};

impl DBClient {
    pub fn get_recommendations(&self, user: &PublicUser, page: u8) -> Vec<PublicRecommendation> {
        let mut conn = self.connect();

        let raw = match diesel::sql_query(format!(
            "
            SELECT DISTINCT A.id, A.title, A.airing_date, A.length,
            A.mean, A.rating, A.picture, A.stats,
            E.score, U.username, U.hash, (
                BIT_COUNT({} ^ U.hash) +
                BIT_COUNT(({} >> {SHF_HASH}) ^ (U.hash >> {SHF_HASH}))
            ) distance
            FROM anime A
            INNER JOIN entries E ON E.anime = A.id
            INNER JOIN users U ON E.user = U.id
            WHERE U.id != {}
            AND E.watched = 1
            AND A.mean IS NOT NULL
            AND NOT EXISTS (SELECT E.id from entries E WHERE E.user = {} AND E.anime = A.id)
            AND (
                A.parent IS NULL
                -- 
                -- Uncomment to allow sequels/side stories into recommendations
                -- 
                -- OR EXISTS (
                --     SELECT E.id from entries E
                --     WHERE E.user = {} AND E.anime = A.parent AND E.watched = 1
                -- )
            )
            GROUP BY A.id
            ORDER BY distance * (5 - A.mean / 2) ASC
            LIMIT 16 OFFSET {};
        ",
            user.hash.to_u64(),
            user.hash.to_u64(),
            user.id,
            user.id,
            user.id,
            page * 16
        ))
        .load::<Recommendation>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return Vec::new();
            }
        };

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
        }

        res
    }
    pub fn get_similar_users(&self, user: &PublicUser, page: u8) -> Vec<PublicSimilarUser> {
        let mut conn = self.connect();

        let raw = match diesel::sql_query(format!(
            "
        SELECT username, hash, (
            BIT_COUNT({} ^ hash) +
            BIT_COUNT(({} >> {SHF_HASH}) ^ (hash >> {SHF_HASH}))
        ) distance
        FROM users
        WHERE id != '{}'
        ORDER BY distance ASC
        LIMIT 16 OFFSET {};
        ",
            user.hash.to_u64(),
            user.hash.to_u64(),
            user.id,
            page * 16
        ))
        .load::<SimilarUser>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return Vec::new();
            }
        };

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
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
    pub fn insert_user(&self, user: &PublicUser, etrs: Vec<PublicListEntry>) -> Option<i32> {
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
                .ok();

            Some(uid)
        } else {
            None
        }
    }
    pub fn update_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        let u = diesel::update(users::users)
            .filter(users::id.eq(user.id))
            .set(UserUpdate::from_public(user))
            .execute(&mut conn)
            .unwrap();
        println!("number of updated users {u}");
        true
    }
    pub fn delete_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        diesel::delete(users::users)
            .filter(users::id.eq(user.id))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_user_entries(
        &self,
        user: &PublicUser,
        limit: usize,
    ) -> Vec<PublicDetailedListEntry> {
        let mut conn = self.connect();

        let raw: Vec<DetailedListEntry> = match diesel::sql_query(format!(
            "
            SELECT A.id, A.mean, A.stats, E.score
            FROM anime A
            INNER JOIN entries E ON E.anime = A.id
            WHERE E.user = {}
            AND E.watched = 1
            ORDER BY E.updated_at DESC
            LIMIT {};
        ",
            user.id, limit
        ))
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
    pub fn update_user_entries(&self, user: &PublicUser, etrs: Vec<PublicListEntry>) -> bool {
        let mut conn = self.connect();

        let mut missing = Vec::new();

        for e in etrs {
            let ie = ListEntryInsert::from_public(user.id, &e);

            let res = match diesel::update(entries::entries)
                .filter(entries::id.eq(e.id))
                .set(&ie)
                .execute(&mut conn)
            {
                Ok(n) => n,
                Err(_) => 0,
            };

            if res == 0 {
                missing.push(ie);
            }
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
    pub fn get_old_users(&self) -> Vec<PublicUser> {
        let mut conn = self.connect();

        let raw = match users::users
            .order_by(users::updated_at.asc())
            .limit(100)
            .get_results::<User>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
        }

        res
    }
}

#[derive(QueryableByName)]
struct Recommendation {
    #[diesel(sql_type = sql::Integer)]
    id: i32,
    #[diesel(sql_type = sql::VarChar)]
    title: String,
    #[diesel(sql_type = sql::Nullable<sql::Timestamp>)]
    airing_date: Option<NaiveDateTime>,
    #[diesel(sql_type = sql::Nullable<sql::Integer>)]
    length: Option<i32>,
    #[diesel(sql_type = sql::Nullable<sql::Float>)]
    mean: Option<f32>,
    #[diesel(sql_type = sql::Nullable<sql::VarChar>)]
    rating: Option<String>,
    #[diesel(sql_type = sql::Nullable<sql::VarChar>)]
    picture: Option<String>,
    #[diesel(sql_type = sql::Longtext)]
    stats: String,
    #[diesel(sql_type = sql::Integer)]
    score: i32,
    #[diesel(sql_type = sql::VarChar)]
    username: String,
    #[diesel(sql_type = sql::Unsigned<sql::Bigint>)]
    hash: u64,
    #[diesel(sql_type = sql::Integer)]
    distance: i32,
}

impl Recommendation {
    fn to_public(self) -> PublicRecommendation {
        PublicRecommendation {
            id: self.id,
            score: self.score,
            details: PublicRecommendationDetails {
                title: self.title,
                airing_date: self.airing_date,
                length: self.length,
                mean: self.mean,
                rating: self.rating,
                picture: self.picture,
                genres: serde_json::from_str::<Vec<i32>>(&self.stats)
                    .unwrap_or(Vec::new())
                    .iter()
                    .filter_map(|stat| Stat::new(stat).to_genre())
                    .collect(),
            },
            user: PublicSimilarUser {
                username: self.username,
                hash: Hash::BigInt(self.hash),
                similarity: similarity(self.distance),
            },
        }
    }
}

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
            hash: Hash::BigInt(self.hash),
            similarity: similarity(self.distance),
        }
    }
}

#[derive(Queryable, QueryableByName)]
struct DetailedListEntry {
    #[diesel(sql_type = sql::Integer)]
    id: i32,
    #[diesel(sql_type = sql::VarChar)]
    stats: String,
    #[diesel(sql_type = sql::Nullable<sql::Float>)]
    mean: Option<f32>,
    #[diesel(sql_type = sql::Integer)]
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
#[diesel(table_name = table_users)]
struct User {
    id: i32,
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

impl User {
    fn to_public(self) -> PublicUser {
        PublicUser {
            id: self.id,
            username: self.username,
            hash: Hash::BigInt(self.hash),
            updated_at: self.updated_at,
        }
    }
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
            hash: user.hash.to_u64(),
            updated_at: user.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_users)]
struct UserUpdate {
    hash: u64,
    updated_at: NaiveDateTime,
}

impl UserUpdate {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            hash: user.hash.to_u64(),
            updated_at: user.updated_at,
        }
    }
}
