use super::MALClient;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use structs::{Anime as PublicAnime, RekoError, RekoResult, Stat};
use util::now;

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
        let mut stats: Vec<Stat> = Vec::new();

        let airing_date = match self.start_date {
            Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date.and_time(NaiveTime::default())),
                _ => None,
            },
            None => None,
        };

        if let Some(date) = airing_date {
            stats.push(Stat::from_airing_date(date));
        }

        if let Some(num_episodes) = self.num_episodes {
            stats.push(Stat::from_series_len(num_episodes));
        }

        if let Some(rating) = self.rating.to_owned() {
            let stat = Stat::from_rating(rating);
            if stat.is_ok() {
                stats.push(stat);
            }
        }

        if let Some(genres) = self.genres {
            genres.iter().for_each(|genre| {
                let stat = Stat::from_genre(genre.id);
                if stat.is_ok() {
                    stats.push(stat);
                }
            });
        }

        let parent = self
            .related_anime
            .iter()
            .find(|r| PARENT.contains(&r.relation_type.as_str()))
            .map(|p| p.node.id);

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
            updated_at: now(),
        }
    }
}

fn anime_err(code: u16, id: i32) -> RekoError {
    RekoError::new(
        code,
        match code {
            404 => "AnimeNotFound",
            403 => "RateLimited",
            422 => "InvalidAnime",
            _ => "FetchFailed",
        },
        match code {
            404 => format!("Anime {id} not found"),
            403 => "Rate limited".to_string(),
            422 => format!("Could not parse Anime {id} details"),
            _ => format!("Could not fetch Anime {id} details"),
        },
    )
}
