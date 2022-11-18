use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::fetch::db::schema::anime;

//  anime details

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelatedAnime {
    pub id: u32,
    pub relation: i16,
}

#[derive(Debug, Clone)]
pub struct AnimeDetails {
    pub id: i32,
    pub title: String,
    pub picture: Option<String>,
    pub airing_date: Option<chrono::NaiveDate>,
    pub mean: Option<i16>,
    pub airing_status: Option<i16>,
    pub genres: Option<Vec<Option<i16>>>,
    pub num_episodes: Option<i16>,
    pub rating: Option<i16>,
    pub related: Option<Vec<RelatedAnime>>,
}

//  anime details mal api response

#[derive(Serialize, Deserialize, Debug)]
pub struct Genre {
    pub id: i16,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainPicture {
    medium: String,
    pub large: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelatedAnimeNode {
    pub id: Option<u32>,
    title: Option<String>,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawRelatedAnime {
    pub node: RelatedAnimeNode,
    pub relation_type: Option<String>,
    relation_type_formatted: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimeAPI {
    pub id: i32,
    pub title: String,
    pub main_picture: Option<MainPicture>,
    pub start_date: Option<String>,
    pub mean: Option<f32>,
    pub status: Option<String>,
    pub genres: Option<Vec<Genre>>,
    pub num_episodes: Option<i16>,
    pub rating: Option<String>,
    pub related_anime: Option<Vec<RawRelatedAnime>>,
}

//  anime database

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