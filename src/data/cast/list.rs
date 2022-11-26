use crate::data::structs::list::ListAPI;

pub fn store_to_user_list(store: Vec<ListAPI>) -> Vec<[i32; 4]> {
    let mut list = vec![];
    for store_list in store.iter() {
        for store_entry in store_list.data.iter() {
            list.push([
                store_entry.node.id,
                status_to_u8(&store_entry.list_status.status) as i32,
                (store_entry.list_status.score * 100) as i32,
                store_entry.list_status.num_episodes_watched as i32,
            ]);
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
