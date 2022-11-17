use super::models::AnimeAPI;
use super::headers::mal_headers;

use crate::db::models::AnimeDB;

pub async fn get(id: &i32) -> Result<AnimeDB, u16> {
    let query: &str = "fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
    let url: String = format!("https://api.myanimelist.net/v2/anime/{}?{}", id, query);

    let client = reqwest::Client::new();
    let res = client.get(url).headers(mal_headers()).send().await;

    match res {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => match res.json::<AnimeAPI>().await {
                Ok(response) => Ok(response.to_db()),
                Err(_) => return Err(1001),
            },
            e => Err(e.as_u16()),
        },
        Err(_) => Err(2001),
    }
}
