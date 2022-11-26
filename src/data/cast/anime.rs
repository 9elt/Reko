use chrono::NaiveDate;

use super::common::{to_serde_value, from_serde_value};
use crate::data::structs::anime::{
    AnimeAPI, AnimeDB, AnimeDetails, Genre, RawRelatedAnime, RelatedAnime,
};

impl AnimeDetails {
    pub fn from_db(db: AnimeDB) -> Self {
        AnimeDetails {
            id: db.id,
            title: db.title,
            picture: db.picture,
            airing_date: db.airing_date,
            mean: db.mean,
            airing_status: db.airing_status,
            genres: db.genres,
            num_episodes: db.num_episodes,
            rating: db.rating,
            related: match db.related {
                Some(r) => Some(from_serde_value::<Vec<RelatedAnime>>(r)),
                None => None,
            },
        }
    }
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
            related: parse_raw_related(self.related_anime),
        }
    }
}

fn parse_raw_related(api_relate: Option<Vec<RawRelatedAnime>>) -> Option<serde_json::Value> {
    match api_relate {
        Some(r) => {
            let mut related: Vec<RelatedAnime> = vec![];
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
            Some(to_serde_value(&related))
        }
        None => None,
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
