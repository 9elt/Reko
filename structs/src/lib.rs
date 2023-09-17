use axum::response::IntoResponse;
use chrono::NaiveDateTime;
use hyper::StatusCode;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RekoError {
    pub code: u16,
    pub message: String,
}

impl RekoError {
    pub fn new<S: ToString>(code: u16, message: S) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for RekoError {
    fn into_response(self) -> axum::response::Response {
        (self.status(), json!(self).to_string()).into_response()
    }
}

pub type RekoResult<T> = Result<T, RekoError>;

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
    pub stats: Vec<i32>,
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
pub struct SimilarUser {
    pub username: String,
    pub hash: Hash,
    pub similarity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedListEntry {
    pub id: i32,
    pub stats: Vec<i32>,
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
    pub user: RecommendationUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationUser {
    pub username: String,
    pub hash: Hash,
    pub similarity: i32,
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

#[derive(Deserialize, Debug)]
pub enum Hash {
    BigInt(u64),
    Hex(String),
}

impl Hash {
    pub fn to_bigint(&self) -> u64 {
        match self {
            Self::BigInt(n) => n.to_owned(),
            Self::Hex(h) => u64::from_str_radix(h, 16).unwrap_or(0),
        }
    }
}

use std::fmt;

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
