mod stat;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

pub type RekoResult<T> = Result<T, RekoError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RekoError {
    pub code: u16,
    pub id: String,
    pub message: String,
}

impl RekoError {
    pub fn new<I: ToString, M: ToString>(code: u16, id: I, message: M) -> Self {
        Self {
            code,
            id: id.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat(i32);

impl Stat {
    pub fn new(uid: &i32) -> Self {
        Self(uid.to_owned())
    }
    pub fn uid(self) -> i32 {
        self.0
    }
    pub fn is_ok(&self) -> bool {
        self.0 != -1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub airing_date: Option<NaiveDateTime>,
    pub length: Option<i32>,
    pub mean: Option<f32>,
    pub rating: Option<String>,
    pub picture: Option<String>,
    pub parent: Option<i32>,
    pub aired: bool,
    pub stats: Vec<Stat>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListEntry {
    pub id: i32,
    pub score: i32,
    pub watched: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: Hash,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoUser {
    pub username: String,
    pub hash: Hash,
    pub similarity: i32,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarUser {
    pub username: String,
    pub hash: Hash,
    pub similarity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestingUser {
    pub username: String,
    pub hash: Hash,
}

impl RequestingUser {
    pub fn from_user(user: &User) -> Self {
        Self {
            username: user.username.to_owned(),
            hash: user.hash.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedListEntry {
    pub id: i32,
    pub stats: Vec<Stat>,
    pub score: i32,
    pub mean: i32,
}

impl DetailedListEntry {
    pub fn new(anime: Anime, entry: &ListEntry) -> Self {
        Self {
            id: anime.id,
            stats: anime.stats,
            score: entry.score,
            mean: match anime.mean {
                Some(mean) => mean as i32,
                None => 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recommendation {
    pub id: i32,
    pub details: RecommendationDetails,
    pub score: i32,
    pub users: Vec<RecoUser>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationDetails {
    pub title: String,
    pub airing_date: Option<NaiveDateTime>,
    pub length: Option<i32>,
    pub mean: Option<f32>,
    pub rating: Option<String>,
    pub picture: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationsFrom {
    pub user: SimilarUser,
    pub recommendations: Vec<UserRecommendation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecommendation {
    pub id: i32,
    pub details: RecommendationDetails,
    pub score: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub requester: RequestingUser,
    pub data: Data,
}

impl Response {
    pub fn new(user: &User, data: Data) -> Self {
        Self {
            requester: RequestingUser::from_user(user),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResponse {
    pub requester: RequestingUser,
    pub data: Data,
    pub pagination: Pagination,
}

impl PaginatedResponse {
    pub fn new(user: &User, data: Data, pagination: Pagination) -> Self {
        Self {
            requester: RequestingUser::from_user(user),
            data,
            pagination,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    pub previous: Option<u8>,
    pub current: u8,
    pub next: Option<u8>,
}

impl Pagination {
    pub fn new(page: u8) -> Self {
        Self {
            previous: if page > 1 { Some(page - 1) } else { None },
            current: page,
            next: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Data {
    Compare(SimilarUser),
    Similar(Vec<SimilarUser>),
    Recommendation(Vec<Recommendation>),
    RecommendationFrom(RecommendationsFrom),
}

#[derive(Deserialize, Debug, Clone)]
pub enum Hash {
    BigInt(u64),
    Hex(String),
}

impl Hash {
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::BigInt(n) => n.to_owned(),
            Self::Hex(h) => u64::from_str_radix(h, 16).unwrap_or(0),
        }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Self::BigInt(n) => format!("{:02x}", n),
                Self::Hex(s) => s.to_owned(),
            }
            .as_str(),
        )
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
