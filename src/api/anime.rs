use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::api::headers::mal_headers;

use crate::db::models::AnimeDB;
use crate::helper::models::RelatedAnime;

//  anime details response
#[derive(Serialize, Deserialize, Debug)]
struct Genre {
    id: i16,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RelatedAnimeNode {
    id: Option<u32>,
    title: Option<String>,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawRelatedAnime {
    node: RelatedAnimeNode,
    relation_type: Option<String>,
    relation_type_formatted: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeAPI {
    id: i32,
    title: String,
    main_picture: Option<MainPicture>,
    start_date: Option<String>,
    mean: Option<f32>,
    status: Option<String>,
    genres: Option<Vec<Genre>>,
    num_episodes: Option<i16>,
    rating: Option<String>,
    related_anime: Option<Vec<RawRelatedAnime>>,
}

impl AnimeAPI {
    pub fn to_db(self) -> AnimeDB {
        AnimeDB {
            id: self.id,
            title: self.title,
            picture: match self.main_picture {
                Some(pic) => Some(pic.large),
                None => None,
            },
            mean: match self.mean {
                Some(m) => Some((m * 100.0) as i16),
                None => None,
            },
            airing_date: match self.start_date {
                Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => None,
                },
                None => None,
            },
            airing_status: match self.status {
                Some(s) => Some(status_to_i16(&s)),
                None => None,
            },
            num_episodes: self.num_episodes,
            rating: match self.rating {
                Some(r) => Some(rating_to_i16(&r)),
                None => None,
            },
            genres: genres_to_vec(self.genres),
            related: related_to_json(self.related_anime),
        }
    }
}

pub async fn get(id: &i32) -> Result<AnimeDB, u16> {
    let anime_id: i32 = id.to_owned();
    let query: &str = "fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
    let url: String = format!(
        "https://api.myanimelist.net/v2/anime/{}?{}",
        anime_id, query
    );

    let client = reqwest::Client::new();
    let res = client.get(url).headers(mal_headers()).send().await.unwrap();

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<AnimeAPI>().await {
            Ok(response) => Ok(response.to_db()),
            Err(_) => return Err(1001),
        },
        e => Err(e.as_u16()),
    }
}

//  conversions
fn related_to_json(api_relate: Option<Vec<RawRelatedAnime>>) -> Option<serde_json::Value> {
    let mut related: Vec<RelatedAnime> = vec![];
    if let Some(r) = api_relate {
        for rel in r.iter() {
            related.push(RelatedAnime {
                id: match rel.node.id {
                    Some(id) => id,
                    None => 0,
                },
                relation: match &rel.relation_type {
                    Some(rel) => relation_to_i16(rel),
                    None => 0,
                },
            });
        }

        let j = serde_json::to_string(&related).unwrap();
        let s = serde_json::from_str(&j).unwrap();

        Some(s)
    } else {
        None
    }
}

fn genres_to_vec(api_genres: Option<Vec<Genre>>) -> Option<Vec<Option<i16>>> {
    let mut genres: Vec<Option<i16>> = vec![];
    if let Some(r) = api_genres {
        for genre in r.iter() {
            genres.push(Some(genre.id));
        }
        Some(genres)
    } else {
        None
    }
}

fn rating_to_i16(rating: &String) -> i16 {
    match rating.as_str() {
        "g" => 1,
        "pg" => 2,
        "pg_13" => 3,
        "r" => 4,
        "r+" => 5,
        "rx" => 6,
        _ => 0,
    }
}

fn status_to_i16(status: &String) -> i16 {
    match status.as_str() {
        "finished_airing" => 1,
        "currently_airing" => 2,
        "not_yet_aired" => 3,
        _ => 0,
    }
}

fn relation_to_i16(relation: &String) -> i16 {
    match relation.as_str() {
        "sequel" => 1,
        "side_story" => 2,
        "summary" => 3,
        "other" => 4,
        "spin_off" => 5,
        "alternative_version" => 6,
        "prequel" => 7,
        "parent_story" => 8,
        _ => 0,
    }
}
