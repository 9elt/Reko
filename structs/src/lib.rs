use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub airing_date: Option<NaiveDateTime>,
    pub length: Option<i32>,
    pub mean: Option<f32>,
    pub rating: Option<String>,
    pub picture: Option<String>,
    pub prequels: Vec<i32>,
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
    pub hash: u64,
    pub updated_at: NaiveDateTime,
}

pub struct SimilarUser {
    pub username: String,
    pub hash: u64,
    pub distance: i32,
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
