use crate::helper::database::anime::DBAnime;
use crate::helper::{AnimeDetails, RelatedAnime};
use crate::utils::conversion::common;
use crate::utils::mal_api::mal_headers;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
    pub fn to_db_anime(&self) -> DBAnime {
        DBAnime::new(
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
                Some(r) => Some(common::to_serde_value(&r)),
                None => None,
            }
        )
    }

    pub fn to_anime_details(&self) -> AnimeDetails {
        AnimeDetails {
            id: self.id,
            title: self.title.to_owned(),
            picture: self.main_picture(),
            airing_date: self.airing_date(),
            mean: self.mean(),
            airing_status: self.status(),
            genres: self.genres(),
            num_episodes: self.num_episodes,
            rating: self.rating(),
            related: self.related(),
        }
    }

    fn main_picture(&self) -> Option<String> {
        match self.main_picture.to_owned() {
            Some(pic) => Some(pic.large),
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
        match self.start_date.to_owned() {
            Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn related(&self) -> Option<Vec<RelatedAnime>> {
        match self.related_anime.to_owned() {
            Some(r) => {
                let mut related: Vec<RelatedAnime> = vec![];
                for rel in r.iter() {
                    related.push(RelatedAnime {
                        id: match rel.node.id {
                            Some(id) => id,
                            None => 0,
                        },
                        relation: relation_to_i16(&rel.relation_type),
                    });
                }
                Some(related)
            }
            None => None,
        }
    }

    fn genres(&self) -> Option<Vec<Option<i16>>> {
        let mut genres: Vec<Option<i16>> = vec![];
        if let Some(r) = self.genres.to_owned() {
            for genre in r.iter() {
                genres.push(Some(genre.id));
            }
            Some(genres)
        } else {
            None
        }
    }

    fn rating(&self) -> Option<i16> {
        match self.rating.to_owned() {
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
        match self.status.to_owned() {
            Some(s) => match s.as_str() {
                "finished_airing" => Some(1),
                "currently_airing" => Some(2),
                "not_yet_aired" => Some(3),
                _ => None
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
            _ => 0,
        },
        None => 0,
    }
}
