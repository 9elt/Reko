use crate::helper::database::anime::RawAnime;
use crate::helper::{AnimeDetails, RelatedAnime};
use crate::utils::mal_api::mal_headers;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Genre {
    id: i16,
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RelatedAnimeNode {
    id: Option<u32>,
    title: Option<String>,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawRelatedAnime {
    node: RelatedAnimeNode,
    relation_type: Option<String>,
    relation_type_formatted: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct APIAnime {
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

pub async fn get(id: &i32) -> Result<APIAnime, u16> {
    let query: &str = "fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
    let url: String = format!("https://api.myanimelist.net/v2/anime/{}?{}", id, query);

    let client = reqwest::Client::new();
    let response = client.get(url).headers(mal_headers()).send().await;

    match response {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.json::<APIAnime>().await {
                Ok(r) => Ok(r),
                Err(_) => return Err(500),
            },
            e => Err(e.as_u16()),
        },
        Err(_) => Err(500),
    }
}

impl APIAnime {
    pub fn serialize(&self) -> RawAnime {
        RawAnime::new(
            self.id,
            self.title.to_owned(),
            self.main_picture(),
            self.mean(),
            self.airing_date(),
            self.status(),
            self.num_episodes,
            self.rating(),
            self.genres(),
            match self.related() {
                Some(r) => Some(json!(&r)),
                None => None,
            },
        )
    }

    pub fn deserialize(&self) -> AnimeDetails {
        AnimeDetails::new(
            self.id,
            self.title.to_owned(),
            self.main_picture(),
            self.airing_date(),
            self.mean(),
            self.status(),
            self.genres(),
            self.num_episodes,
            self.rating(),
            self.related(),
        )
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Getters
    ////////////////////////////////////////////////////////////////////////////////

    fn main_picture(&self) -> Option<String> {
        match &self.main_picture {
            Some(pic) => Some(pic.large.to_owned()),
            None => None,
        }
    }

    fn mean(&self) -> Option<i16> {
        match self.mean {
            Some(m) => Some((m * 100.0) as i16),
            None => None,
        }
    }

    fn airing_date(&self) -> Option<NaiveDate> {
        match &self.start_date {
            Some(d) => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn related(&self) -> Option<Vec<RelatedAnime>> {
        match &self.related_anime {
            Some(r) => Some(
                r.iter()
                    .map(|e| {
                        RelatedAnime::new(
                            match e.node.id {
                                Some(id) => id,
                                None => 0,
                            },
                            relation_to_i16(&e.relation_type),
                        )
                    })
                    .collect(),
            ),
            None => None,
        }
    }

    fn genres(&self) -> Option<Vec<Option<i16>>> {
        match &self.genres {
            Some(genres) => Some(genres.iter().map(|g| Some(g.id)).collect()),
            None => None
        }
    }

    fn rating(&self) -> Option<i16> {
        match &self.rating {
            Some(r) => match r.as_str() {
                "g" => Some(1),
                "pg" => Some(2),
                "pg_13" => Some(3),
                "r" => Some(4),
                "r+" => Some(5),
                "rx" => Some(6),
                _ => Some(0),
            },
            None => None,
        }
    }

    fn status(&self) -> Option<i16> {
        match &self.status {
            Some(s) => match s.as_str() {
                "finished_airing" => Some(1),
                "currently_airing" => Some(2),
                "not_yet_aired" => Some(3),
                _ => None,
            },
            None => None,
        }
    }
}

fn relation_to_i16(relation: &Option<String>) -> i16 {
    match relation {
        Some(s) => match s.as_str() {
            "sequel" => 1,
            "side_story" => 2,
            "summary" => 3,
            "other" => 4,
            "spin_off" => 5,
            "alternative_version" => 6,
            "prequel" => 7,
            "parent_story" => 8,
            "full_story" => 9,
            _ => 0,
        },
        None => 0,
    }
}
