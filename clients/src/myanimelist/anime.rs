use super::MALClient;
use chrono::{NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use structs::{Anime as PublicAnime, RekoError, RekoResult, Stat};

const ANIME_QUERY: &str =
    "?fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
const PARENT: &[&str] = &["prequel", "parent_story", "full_story"];

impl MALClient {
    pub async fn anime(&self, id: i32) -> RekoResult<PublicAnime> {
        match self.get::<Anime>(format!("/anime/{id}{ANIME_QUERY}")).await {
            Ok(res) => Ok(res.to_public()),
            Err(code) => Err(anime_err(code, id)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Anime {
    id: i32,
    title: String,
    main_picture: Option<AnimePicture>,
    start_date: Option<String>,
    mean: Option<f32>,
    status: String,
    genres: Option<Vec<AnimeGenre>>,
    num_episodes: Option<i32>,
    rating: Option<String>,
    related_anime: Vec<AnimeRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeRelated {
    node: AnimeNode,
    relation_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeNode {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeGenre {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimePicture {
    pub medium: String,
}

impl Anime {
    fn to_public(self) -> PublicAnime {
        let mut stats: Vec<i32> = Vec::new();

        let airing_date = match self.start_date {
            Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date.and_time(NaiveTime::default())),
                _ => None,
            },
            None => None,
        };

        if let Some(date) = airing_date {
            stats.push(Stat::from_airing_date(date).uid());
        }

        if let Some(num_episodes) = self.num_episodes {
            stats.push(Stat::from_series_len(num_episodes).uid());
        }

        if let Some(rating) = self.rating.to_owned() {
            let stat = Stat::from_rating(rating);
            if stat.is_ok() {
                stats.push(stat.uid());
            }
        }

        if let Some(genres) = self.genres {
            genres.iter().for_each(|genre| {
                let stat = Stat::from_genre(genre.id);
                if stat.is_ok() {
                    stats.push(stat.uid());
                }
            });
        }

        let parent = match self
            .related_anime
            .iter()
            .find(|r| PARENT.contains(&r.relation_type.as_str()))
        {
            Some(p) => Some(p.node.id),
            None => None,
        };

        PublicAnime {
            id: self.id,
            title: self.title,
            airing_date,
            length: self.num_episodes,
            mean: self.mean,
            rating: self.rating,
            picture: match self.main_picture {
                Some(pic) => Some(pic.medium),
                None => None,
            },
            stats,
            parent,
            aired: self.status == "finished_airing",
            updated_at: Utc::now().naive_utc(),
        }
    }
}

fn anime_err(code: u16, id: i32) -> RekoError {
    RekoError {
        code,
        message: match code {
            404 => format!("Anime {id} not found"),
            403 => format!("Rate limited"),
            422 => format!("Could not parse Anime {id} details"),
            _ => format!("Could not fetch Anime {id} details"),
        },
    }
}
