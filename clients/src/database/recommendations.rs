use super::DBClient;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types as sql;
use std::str::FromStr;
use structs::Hash;
use structs::Pagination;
use structs::RecoUser as PublicRecoUser;
use structs::Recommendation as PublicRecommendation;
use structs::RecommendationDetails as PublicRecommendationDetails;
use structs::Stat;
use structs::User as PublicUser;
use structs::UserRecommendation as PublicUserRecommendation;
use util::{pub_page, similarity, HASH_MASK, MAX_PAGE_RECOMMENDATIONS};

const SIMILAR_PAGE_SIZE: i32 = 32;
const RANDOM_PAGE_SIZE: i32 = 32;
const RECO_PAGE_SIZE: i32 = 16;
const RECO_PAGE_TAKE: i32 = RECO_PAGE_SIZE + 1;

impl DBClient {
    pub fn get_recommendations(
        &self,
        user: &PublicUser,
        page: u8,
        batch: u8,
    ) -> (Vec<PublicRecommendation>, Pagination) {
        let mut pagination = Pagination::new(pub_page(page));

        let mut conn = self.connect();

        let id = user.id;
        let hash = user.hash.to_u64();
        let user_offeset = batch as i32 * SIMILAR_PAGE_SIZE;
        let offset = page as i32 * RECO_PAGE_SIZE;

        let users_query = match diesel::sql_query(format!(
            "
        SELECT id FROM users WHERE id != '{id}'
        ORDER BY (
            BIT_COUNT({hash} ^ hash) +
            BIT_COUNT(({hash} & {HASH_MASK}) ^ (hash & {HASH_MASK}))
        ) ASC
        LIMIT {SIMILAR_PAGE_SIZE} OFFSET {user_offeset};
        "
        ))
        .load::<GenericId>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return (Vec::new(), pagination);
            }
        }
        .iter()
        .map(|u| format!("OR E.user = {} ", u.id))
        .collect::<String>();

        let mut raw = match diesel::sql_query(format!(
            "
        SELECT

        DISTINCT A.id, A.title, A.airing_date, A.length,
        A.mean, A.rating, A.picture, A.stats,

        (SUM(E.score) / COUNT(E.score)) score, -- users mean score

        -- recommending users data

        GROUP_CONCAT(U.username) usernames,
        GROUP_CONCAT(U.hash) hashes,
        GROUP_CONCAT((
            BIT_COUNT({hash} ^ U.hash) +
            BIT_COUNT(({hash} & {HASH_MASK}) ^ (U.hash & {HASH_MASK}))
        )) distances,
        GROUP_CONCAT(E.score) scores

        FROM entries E
        INNER JOIN users U ON E.user = U.id
        INNER JOIN anime A ON E.anime = A.id

        WHERE (
            E.user = 2147483647 -- initialize or statement
            {users_query}
        )

        AND E.watched = 1
        AND E.score > 0
        AND A.mean IS NOT NULL

        AND E.anime != 21 -- we don't recommend One Piece in here

        AND E.anime NOT IN (
            SELECT anime
            FROM entries
            WHERE user = '{id}'
        )

        AND A.parent IS NULL -- no sequels/spinoffs

        GROUP BY A.id

        ORDER BY (
            SUM(E.score) / (COUNT(E.score) + 1) - FLOOR(
                SUM(DATEDIFF(NOW(), E.updated_at)) / (COUNT(E.score) * 730)
            )
        ) DESC

        LIMIT {RECO_PAGE_TAKE} OFFSET {offset};
        "
        ))
        .load::<Recommendation>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return (Vec::new(), pagination);
            }
        };

        let is_next_page = raw.len() == RECO_PAGE_TAKE as usize;
        if is_next_page {
            raw.pop();
        }

        let mut res = Vec::with_capacity(raw.len());
        for reco in raw {
            res.push(reco.to_public());
        }

        if is_next_page && pub_page(page) < MAX_PAGE_RECOMMENDATIONS {
            pagination.next = Some(pub_page(page) + 1);
        }

        (res, pagination)
    }
    pub fn get_recommendations_from(
        &self,
        user: &PublicUser,
        other: &PublicUser,
        page: u8,
    ) -> (Vec<PublicUserRecommendation>, Pagination) {
        let mut pagination = Pagination::new(pub_page(page));

        let mut conn = self.connect();

        let id = user.id;
        let other_id = other.id;
        let offset = page as i32 * RECO_PAGE_SIZE;

        let mut raw = match diesel::sql_query(format!(
            "
        SELECT

        DISTINCT A.id, A.title, A.airing_date, A.length,
        A.mean, A.rating, A.picture, A.stats, E.score

        FROM entries E
        INNER JOIN anime A ON E.anime = A.id

        WHERE E.user = {other_id}

        AND E.watched = 1
        AND E.score > 0
        AND A.mean IS NOT NULL

        AND E.anime != 21 -- we don't recommend One Piece in here

        AND E.anime NOT IN (
            SELECT anime
            FROM entries
            WHERE user = '{id}'
        )

        AND A.parent IS NULL -- no sequels/spinoffs

        ORDER BY (
            E.score - FLOOR(DATEDIFF(NOW(), E.updated_at) / 730)
        ) DESC

        LIMIT {RECO_PAGE_TAKE} OFFSET {offset};
        "
        ))
        .load::<UserRecommendation>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return (Vec::new(), pagination);
            }
        };

        let is_next_page = raw.len() == RECO_PAGE_TAKE as usize;
        if is_next_page {
            raw.pop();
        }

        let mut res = Vec::with_capacity(raw.len());
        for reco in raw {
            res.push(reco.to_public());
        }

        if is_next_page && pub_page(page) < MAX_PAGE_RECOMMENDATIONS {
            pagination.next = Some(pub_page(page) + 1);
        }

        (res, pagination)
    }
    pub fn get_random_recommendations(
        &self,
        user: &PublicUser,
        batch: u8,
    ) -> Vec<PublicRecommendation> {
        let mut conn = self.connect();

        let id = user.id;
        let hash = user.hash.to_u64();
        let user_offeset = batch as i32 * SIMILAR_PAGE_SIZE;

        let users_query = match diesel::sql_query(format!(
            "
        SELECT id FROM users WHERE id != '{id}'
        ORDER BY (
            BIT_COUNT({hash} ^ hash) +
            BIT_COUNT(({hash} & {HASH_MASK}) ^ (hash & {HASH_MASK}))
        ) ASC
        LIMIT {SIMILAR_PAGE_SIZE} OFFSET {user_offeset};
        "
        ))
        .load::<GenericId>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return Vec::new();
            }
        }
        .iter()
        .map(|u| format!("OR E.user = {} ", u.id))
        .collect::<String>();

        let mut raw = match diesel::sql_query(format!(
            "
        SELECT

        DISTINCT A.id, A.title, A.airing_date, A.length,
        A.mean, A.rating, A.picture, A.stats,

        (SUM(E.score) / COUNT(E.score)) score, -- users mean score

        -- recommending users data

        GROUP_CONCAT(U.username) usernames,
        GROUP_CONCAT(U.hash) hashes,
        GROUP_CONCAT((
            BIT_COUNT({hash} ^ U.hash) +
            BIT_COUNT(({hash} & {HASH_MASK}) ^ (U.hash & {HASH_MASK}))
        )) distances,
        GROUP_CONCAT(E.score) scores

        FROM entries E
        INNER JOIN users U ON E.user = U.id
        INNER JOIN anime A ON E.anime = A.id

        WHERE (
            E.user = 2147483647 -- initialize or statement
            {users_query}
        )

        AND E.watched = 1
        AND E.score > 0
        AND A.mean IS NOT NULL

        AND E.anime != 21 -- we don't recommend One Piece in here

        AND E.anime NOT IN (
            SELECT anime
            FROM entries
            WHERE user = '{id}'
        )

        AND A.parent IS NULL -- no sequels/spinoffs

        GROUP BY A.id

        ORDER BY RAND() * 10 + A.mean DESC

        LIMIT {RANDOM_PAGE_SIZE};
        "
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
        for reco in raw {
            res.push(reco.to_public());
        }

        res
    }
}

#[derive(QueryableByName)]
struct GenericId {
    #[diesel(sql_type = sql::Integer)]
    id: i32,
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
    scores: String,
    #[diesel(sql_type = sql::VarChar)]
    usernames: String,
    #[diesel(sql_type = sql::VarChar)]
    hashes: String,
    #[diesel(sql_type = sql::VarChar)]
    distances: String,
}

impl Recommendation {
    fn to_public(self) -> PublicRecommendation {
        let scores = group_concat::<i32>(&self.scores);
        let usernames = group_concat::<String>(&self.usernames);
        let hashes = group_concat::<u64>(&self.hashes);
        let distances = group_concat::<i32>(&self.distances);

        let n = scores.len();
        let mut users = Vec::with_capacity(n);

        for i in 0..n {
            users.push(PublicRecoUser {
                username: usernames[i].to_owned(),
                hash: Hash::BigInt(hashes[i]),
                score: scores[i],
                similarity: similarity(distances[i]),
            });
        }

        users.sort_by(|a, b| b.similarity.cmp(&a.similarity));

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
            users,
        }
    }
}

fn group_concat<T: FromStr>(group_concat: &String) -> Vec<T> {
    group_concat
        .split(",")
        .filter_map(|id| T::from_str(id).ok())
        .collect::<Vec<_>>()
}

#[derive(QueryableByName)]
struct UserRecommendation {
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
}

impl UserRecommendation {
    fn to_public(self) -> PublicUserRecommendation {
        PublicUserRecommendation {
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
        }
    }
}
