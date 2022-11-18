use super::generic::from_serde_value;
use crate::fetch::structs::list::{ListAPI, ListEntry, List, ListsDB};

impl List {
    pub fn from_db(db: ListsDB) -> Self {
        List {
            user_hash: db.user_hash,
            list: from_serde_value::<Vec<ListEntry>>(db.list),
            updated_at: db.updated_at,
        }
    }
}

pub fn store_to_user_list(store: Vec<ListAPI>) -> Vec<ListEntry> {
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

pub fn status_to_u8(status: &String) -> u8 {
    match status.as_str() {
        "completed" => 1,
        "plan_to_watch" => 2,
        "watching" => 3,
        "on_hold" => 4,
        "dropped" => 5,
        _ => 0,
    }
}