use serde::{Serialize, Deserialize};

use crate::algorithm::model::Model;
use crate::helper::AffinityUsers;

#[derive(Serialize, Deserialize)]
pub struct EntryData {
    id: i32,
    users: Vec<u8>,
}

pub fn extract(
    user_model: Model<i16>,
    user_list: Vec<Vec<i32>>,
    similar_users: &Vec<AffinityUsers>,
) -> Result<Vec<EntryData>, u16> {

    let user_entries: Vec<i32> = user_list.iter().map(|x| x[0]).collect();

    let mut missing_entries: Vec<i32> = vec![];
    let mut missing_entries_with_data: Vec<EntryData> = vec![];

    for i in 0..similar_users.len() {
        let mut user_unique = 0;
        //let mut missing_entries = vec![];
        for entry in similar_users[i].list.iter() {
            if user_unique > 32 - (i * 2) {
                break;
            }
            // not completed
            if entry[1] > 1 {
                continue;
            }

            // no episodes watched
            if entry[3] == 0 {
                continue;
            }

            // already in user's list
            if user_entries.contains(&entry[0]) {
                continue;
            }

            if missing_entries.contains(&entry[0]) {
                for e in missing_entries_with_data.iter_mut() {
                    if e.id == entry[0] {
                        e.users.push(i as u8);
                        break;
                    }
                }
            } else {
                missing_entries_with_data.push(
                    EntryData {
                        id: entry[0],
                        users: vec![i as u8]
                    }
                );
                missing_entries.push(entry[0]);
                user_unique += 1;
            }
        }
    }

    Ok(missing_entries_with_data)
}
