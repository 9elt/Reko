use serde::{Deserialize, Serialize};

use super::anime::AnimeDetails;

//  user list
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListEntry {
    pub id: i32,
    pub status: i32,
    pub score: i32,
    pub episodes_watched: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedListEntry {
    pub entry: ListEntry,
    pub details: AnimeDetails,
}

//  list mal api response

#[derive(Serialize, Deserialize, Debug)]
struct MainPicture {
    medium: String,
    large: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListNode {
    pub id: i32,
    title: String,
    main_picture: Option<MainPicture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListStatus {
    pub status: String,
    pub score: u16,
    pub num_episodes_watched: u16,
    is_rewatching: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListEntryAPI {
    pub node: ListNode,
    pub list_status: ListStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paging {
    previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAPI {
    pub data: Vec<ListEntryAPI>,
    pub paging: Paging,
}