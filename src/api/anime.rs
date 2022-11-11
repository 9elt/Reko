use crate::api::headers::mal_headers;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RelatedAnime {
    id: u32,
    relation: u8,
}

#[derive(Debug)]
pub struct AnimeDetails {
    id: u32,
    title: String,
    picture: String,
    date: String,
    mean: u16,
    status: u8,
    genres: Vec<u8>,
    num_episodes: u16,
    rating: u8,
    related_anime: Vec<RelatedAnime>,
}

//  anime details response
#[derive(Serialize, Deserialize, Debug)]
struct Genre {
    id: u8,
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
struct GetAnimeResponse {
    id: u32,
    title: String,
    main_picture: Option<MainPicture>,
    start_date: Option<String>,
    mean: Option<f32>,
    status: Option<String>,
    genres: Option<Vec<Genre>>,
    num_episodes: Option<u16>,
    rating: Option<String>,
    related_anime: Option<Vec<RawRelatedAnime>>,
}

pub async fn get(id: u32) -> Result<AnimeDetails, u16> {
    let anime_id: u32 = id;
    let query: &str = "fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
    let url: String = format!(
        "https://api.myanimelist.net/v2/anime/{}?{}",
        anime_id, query
    );

    let client = reqwest::Client::new();
    let res = client.get(url).headers(mal_headers()).send().await.unwrap();

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<GetAnimeResponse>().await {
            Ok(response) => Ok(res_to_anime_details(response)),
            Err(_) => return Err(1001),
        },
        e => Err(e.as_u16()),
    }
}

fn res_to_anime_details(res: GetAnimeResponse) -> AnimeDetails {
    let mut genres: Vec<u8> = vec![];

    if let Some(res_genres) = res.genres {
        for genre in res_genres.iter() {
            genres.push(genre.id);
        }
    }

    let mut related: Vec<RelatedAnime> = vec![];

    if let Some(res_related) = res.related_anime {
        for rel in res_related.iter() {
            related.push(RelatedAnime {
                id: match rel.node.id {
                    Some(id) => id,
                    None => 0,
                },
                relation: match &rel.relation_type {
                    Some(rel) => relation_to_u8(&rel),
                    None => 0,
                },
            });
        }
    }

    AnimeDetails {
        id: res.id,
        title: res.title,
        picture: match res.main_picture {
            Some(main) => main.large,
            None => format!(""),
        },
        date: match res.start_date {
            Some(date) => date,
            None => format!(""),
        },
        mean: match res.mean {
            Some(mean) => ((mean * 100.00) as u16).try_into().unwrap(),
            None => 0,
        },
        status: match res.status {
            Some(status) => status_to_u8(&status),
            None => 0,
        },
        genres: genres,
        num_episodes: match res.num_episodes {
            Some(episodes) => episodes,
            None => 0,
        },
        rating: match res.rating {
            Some(rating) => rating_to_u8(&rating),
            None => 0,
        },
        related_anime: related,
    }
}

fn rating_to_u8(rating: &String) -> u8 {
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

fn status_to_u8(status: &String) -> u8 {
    match status.as_str() {
        "finished_airing" => 1,
        "currently_airing" => 2,
        "not_yet_aired" => 3,
        _ => 4,
    }
}

fn relation_to_u8(relation: &String) -> u8 {
    match relation.as_str() {
        "prequel" => 1,
        "parent_story" => 2,
        "sequel" => 3,
        "side_story" => 4,
        "other" => 5,
        _ => 0,
    }
}
