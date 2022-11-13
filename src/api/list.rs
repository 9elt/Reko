use crate::api::headers::mal_headers;
use serde::{Deserialize, Serialize};

use crate::helper::models::ListEntry;

#[derive(Serialize, Deserialize, Debug)]
struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListNode {
    id: u32,
    title: String,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListStatus {
    status: String,
    score: u16,
    num_episodes_watched: u16,
    is_rewatching: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListEntryAPI {
    node: ListNode,
    list_status: ListStatus,
}

#[derive(Serialize, Deserialize, Debug)]
struct Paging {
    previous: Option<String>,
    next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListAPI {
    data: Vec<ListEntryAPI>,
    paging: Paging,
}

pub async fn get(user: &str) -> Result<Vec<ListEntry>, u16> {
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

fn store_to_user_list(store: Vec<ListAPI>) -> Vec<ListEntry> {
    let mut list =  vec![];
    for store_list in store.iter() {
        for store_entry in store_list.data.iter() {
            list.push(ListEntry {
                id: store_entry.node.id,
                status: status_to_u8(&store_entry.list_status.status),
                score: store_entry.list_status.score * 100,
                episodes_watched: store_entry.list_status.num_episodes_watched,
            });
        }
    }
    list
}

fn status_to_u8(status: &String) -> u8 {
    match status.as_str() {
        "completed" => 1,
        "plan_to_watch" => 2,
        "watching" => 3,
        "on_hold" => 4,
        "dropped" => 5,
        _ => 0,
    }
}
