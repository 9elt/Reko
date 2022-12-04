use std::vec;

use crate::utils::mal_api::mal_headers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListNode {
    id: i32,
    title: String,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize)]
pub struct ListStatus {
    status: String,
    score: u16,
    num_episodes_watched: u16,
    is_rewatching: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct APIListEntry {
    node: ListNode,
    list_status: ListStatus,
}

#[derive(Serialize, Deserialize)]
pub struct Paging {
    previous: Option<String>,
    next: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct APICastList {
    data: Vec<APIListEntry>,
    paging: Paging,
}

pub struct APIList {
    list: Vec<APIListEntry>,
}

pub async fn get(user: &str) -> Result<APIList, u16> {
    let client = reqwest::Client::new();
    let user_name: &str = user;
    let query: &str = "fields=list_status&sort=list_updated_at&limit=1000&nsfw=1&offset=0000";
    let mut url: String = format!(
        "https://api.myanimelist.net/v2/users/{}/animelist?{}",
        user_name, query
    );

    let mut complete_result: APIList = APIList { list: vec![] };

    let mut res;
    for _i in 0..64 {
        res = match client.get(&url).headers(mal_headers()).send().await {
            Ok(r) => r,
            Err(_) => return Err(500)
        };
        match res.status() {
            reqwest::StatusCode::OK => {
                match res.json::<APICastList>().await {
                    Ok(mut response) => match &response.paging.next {
                        Some(next) => {
                            url = next.to_owned();
                            complete_result.list.append(&mut response.data);
                        }
                        None => {
                            complete_result.list.append(&mut response.data);
                            break;
                        },
                    },
                    Err(_) => return Err(500),
                };
            }
            e => return Err(e.as_u16()),
        }
    }

    Ok(complete_result)
}

impl APIList {
    pub fn to_vec(&self) -> Vec<Vec<i32>> {
        self.list.iter().map(|entry| vec![
            entry.node.id,
            status_to_i32(&entry.list_status.status),
            (entry.list_status.score * 100) as i32,
            entry.list_status.num_episodes_watched as i32,
        ]).collect()
    }
}

fn status_to_i32(status: &String) -> i32 {
    match status.as_str() {
        "completed" => 1,
        "plan_to_watch" => 2,
        "watching" => 3,
        "on_hold" => 4,
        "dropped" => 5,
        _ => 0,
    }
}