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
use structs::Hash;
use structs::ListEntry as PublicListEntry;
use structs::Recommendation as PublicRecommendation;
use structs::RecommendationDetails as PublicRecommendationDetails;
use structs::SimilarUser as PublicSimilarUser;
use structs::User as PublicUser;

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
    pub fn update_anime(&self, data: PublicAnime) -> bool {
        let mut conn = self.connect();

        diesel::update(anime::anime)
            .filter(anime::id.eq(data.id))
            .set(AnimeUpdate::from_public(&data))
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
    pub fn get_airing_anime(&self) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match anime::anime
            .filter(anime::aired.eq(false))
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
    pub fn get_missing_anime(&self) -> Vec<i32> {
        let mut conn = self.connect();

        match entries::entries
            .left_join(anime::anime.on(anime::id.eq(entries::anime)))
            .select(entries::anime)
            .filter(entries::anime.is_null())
            .load::<i32>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => Vec::new(),
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
    pub fn delete_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        diesel::delete(users::users)
            .filter(users::id.eq(user.id))
            .execute(&mut conn)
            .is_ok()
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
    pub fn get_recommendations(&self, user: &PublicUser, page: u8) -> Vec<PublicRecommendation> {
        let mut conn = self.connect();

        let raw = match diesel::sql_query(format!(
            "
            SELECT DISTINCT A.id, A.title, A.airing_date, A.length,
            A.mean, A.rating, A.picture, A.stats,
            E.score, U.username, U.hash, (
                BIT_COUNT({} ^ U.hash) +
                BIT_COUNT(({} >> 48) ^ (U.hash >> 48))
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
            user.hash.to_bigint(),
            user.hash.to_bigint(),
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
            BIT_COUNT(({} >> 48) ^ (hash >> 48))
        ) distance
        FROM users
        WHERE username != '{}'
        ORDER BY distance ASC
        LIMIT 16 OFFSET {};
        ",
            user.hash.to_bigint(),
            user.hash.to_bigint(),
            user.username,
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
}

use diesel::sql_types as sql;

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
                    .filter_map(|stat| genre_from_stat(stat))
                    .collect(),
            },
            user: PublicSimilarUser {
                username: self.username,
                hash: Hash::BigInt(self.hash),
                similarity: 100 - (self.distance * 100 / 80),
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
            similarity: 100 - (self.distance * 100 / 80),
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
struct UserUpdate {
    hash: u64,
    updated_at: NaiveDateTime,
}

impl UserUpdate {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            hash: user.hash.to_bigint(),
            updated_at: user.updated_at,
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
            hash: user.hash.to_bigint(),
            updated_at: user.updated_at,
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
            parent: ani.parent,
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
            parent: self.parent,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_anime)]
struct AnimeUpdate {
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

impl AnimeUpdate {
    fn from_public(ani: &PublicAnime) -> Self {
        Self {
            title: ani.title.to_owned(),
            airing_date: ani.airing_date,
            length: ani.length,
            mean: ani.mean,
            rating: ani.rating.to_owned(),
            picture: ani.picture.to_owned(),
            aired: ani.aired,
            stats: serde_json::json!(ani.stats).to_string(),
            updated_at: ani.updated_at,
            parent: ani.parent,
        }
    }
}

fn genre_from_stat(stat: &i32) -> Option<String> {
    let g = match stat {
        2 => Some("Action"),
        13 => Some("Adventure"),
        4 => Some("Comedy"),
        9 => Some("Drama"),
        3 => Some("Fantasy"),
        10 => Some("Romance"),
        17 => Some("SciFi"),
        15 => Some("Supernatural"),
        70 => Some("AvantGarde"),
        25 => Some("AwardWinning"),
        76 => Some("BoysLove"),
        77 => Some("GirlsLove"),
        68 => Some("Gourmet"),
        36 => Some("Horror"),
        19 => Some("Mystery"),
        39 => Some("SliceofLife"),
        41 => Some("Sports"),
        29 => Some("Suspense"),
        21 => Some("Ecchi"),
        86 => Some("Erotica"),
        71 => Some("Hentai"),
        74 => Some("Josei"),
        73 => Some("Kids"),
        18 => Some("Seinen"),
        38 => Some("Shoujo"),
        5 => Some("Shounen"),
        28 => Some("AdultCast"),
        47 => Some("GagHumor"),
        26 => Some("Gore"),
        27 => Some("Harem"),
        32 => Some("Historical"),
        30 => Some("Isekai"),
        53 => Some("Iyashikei"),
        43 => Some("LovePolygon"),
        49 => Some("MartialArts"),
        35 => Some("Mecha"),
        31 => Some("Military"),
        45 => Some("Music"),
        34 => Some("Mythology"),
        42 => Some("Parody"),
        20 => Some("Psychological"),
        8 => Some("School"),
        22 => Some("SuperPower"),
        37 => Some("Survival"),
        44 => Some("TimeTravel"),
        46 => Some("Vampire"),
        69 => Some("Anthropomorphic"),
        57 => Some("CGDCT"),
        67 => Some("Childcare"),
        82 => Some("CombatSports"),
        85 => Some("Crossdressing"),
        78 => Some("Delinquents"),
        55 => Some("Detective"),
        89 => Some("Educational"),
        64 => Some("HighStakesGame"),
        81 => Some("IdolsFemale"),
        90 => Some("IdolsMale"),
        88 => Some("MagicalSexShift"),
        66 => Some("MahouShoujo"),
        87 => Some("Medical"),
        60 => Some("OrganizedCrime"),
        52 => Some("OtakuCulture"),
        79 => Some("PerformingArts"),
        91 => Some("Pets"),
        84 => Some("Racing"),
        48 => Some("Reincarnation"),
        80 => Some("ReverseHarem"),
        51 => Some("RomanticSubtext"),
        63 => Some("Samurai"),
        83 => Some("Showbiz"),
        62 => Some("Space"),
        65 => Some("StrategyGame"),
        54 => Some("TeamSports"),
        56 => Some("VideoGame"),
        75 => Some("VisualArts"),
        58 => Some("Workplace"),
        _ => None,
    };

    match g {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}
