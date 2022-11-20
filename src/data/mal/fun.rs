use crate::data::structs::anime::{AnimeAPI, AnimeDB};
use crate::data::structs::list::{ListAPI, ListEntry};

use crate::data::cast::list::store_to_user_list;

use super::headers::mal_headers;

pub async fn get_mal_anime(id: &i32) -> Result<AnimeDB, u16> {
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

pub async fn get_mal_list(user: &str) -> Result<Vec<ListEntry>, u16> {
    let client = reqwest::Client::new();
    let user_name: &str = user;
    let query: &str = "fields=list_status&sort=list_updated_at&limit=1000&nsfw=1&offset=0000";
    let mut url: String = format!(
        "https://api.myanimelist.net/v2/users/{}/animelist?{}",
        user_name, query
    );
    let mut store: Vec<ListAPI> = vec![];
    let mut res;

    loop {
        res = client.get(&url).headers(mal_headers()).send().await.unwrap();
        match res.status() {
            reqwest::StatusCode::OK => {
                match res.json::<ListAPI>().await {
                    Ok(response) => match &response.paging.next {
                        Some(next) => {
                            url = next.to_owned();
                            store.push(response);
                        }
                        None => break store.push(response),
                    },
                    Err(_) => return Err(1001),
                };
            }
            e => return Err(e.as_u16()),
        }
    }

    Ok(store_to_user_list(store))
}