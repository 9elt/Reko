use serde::Serialize;

use crate::algorithm::model::{Indexer, Model};
use crate::helper::AffinityUsers;
use crate::helper::{self, AnimeDetails};
use crate::utils::z_table;

#[derive(Serialize)]
pub struct Reko {
    id: i32,
    users: Vec<u8>,
    details: AnimeDetails,
    predictions: EntryPredictions,
}

pub async fn extract(
    user_model: Model<i16>,
    user_list: Vec<Vec<i32>>,
    similar_users: &Vec<AffinityUsers>,
    banned: &Vec<i32>,
) -> Result<Vec<Reko>, u16> {
    let entries = missing_unique_entries(&user_list, similar_users, banned);
    let entries_ids: Vec<i32> = entries.iter().map(|x| x.id).collect();
    let detailed = helper::get_anime_details(entries_ids.to_owned()).await;

    let mut recommendations = vec![];

    for entry_details in detailed.iter() {

        if is_sequel(entry_details) {
            continue;
        }

        let users = match entries.iter().find(|missing_e| missing_e.id == entry_details.id) {
            Some(entry) => entry.users.to_owned(),
            None => vec![]
        };

        recommendations.push(Reko {
            id: entry_details.id,
            users,
            details: entry_details.to_owned(),
            predictions: EntryPredictions::from_entry(entry_details, &user_model),
        });
    }

    recommendations.sort_unstable_by_key(|x| 1000 - x.predictions.enjoyment);

    let mut parsed_reko: Vec<Reko> = vec![];
    let min_exp = recommendations[0].predictions.enjoyment / 2;

    for reko in recommendations {
        if reko.predictions.enjoyment < min_exp {
            break;
        }
        parsed_reko.push(reko);
    }

    Ok(parsed_reko)
}

fn is_sequel(entry: &AnimeDetails) -> bool {
    match &entry.related {
        // `relation > 6` means the entry has a prequel
        Some(related) => related.iter().any(|r| r.relation > 6),
        None => false,
    }
}

////////////////////////////////////////////////////////////////////////////////
// Users affinity
////////////////////////////////////////////////////////////////////////////////
#[derive(Serialize)]
pub struct UsersInfo {
    user_name: String,
    affinity: i16,
}

pub fn user_info(similar_users: &Vec<AffinityUsers>, user_model: &Model<i16>) -> Result<Vec<UsersInfo>, u16> {
    let normal_dist = match helper::get_normal_dist() {
        Ok(v) => v,
        Err(_) => return Err(500) 
    };

    let mut users_info = vec![];

    for similar in similar_users.iter() {
        let mut tot_dev: i32 = 0;
        let mut counter: i32 = 0;

        for x in 0..user_model.len() {
            for y in 0..user_model[x].len() {
                for z in 0..user_model[x][y].len() {
                    tot_dev += user_deviation(
                        user_model[x][y][z],
                        similar.model[x][y][z],
                        normal_dist.mean(x, y, z),
                        normal_dist.std_dev(x, y, z)
                    ) as i32;
                    counter += 1;
                }
            }
        };

        users_info.push(UsersInfo {
            user_name: similar.user_name.to_owned(),
            affinity: 1000 - (tot_dev / counter) as i16
        })
    }

    Ok(users_info)
}

fn user_deviation(user_value: i16, other_value: i16, mean: i16, std_dev: i16) -> i16 {
    let user_z_score = (user_value as f32 - mean as f32) / std_dev as f32;
    let other_z_score = (other_value as f32 - mean as f32) / std_dev as f32;

    let user_cumulative_dist = z_table::cumulative_dist(user_z_score);
    let other_cumulative_dist = z_table::cumulative_dist(other_z_score);

    (user_cumulative_dist * 1000.0 - other_cumulative_dist * 1000.0).abs() as i16
}

////////////////////////////////////////////////////////////////////////////////
// Entry predictions
////////////////////////////////////////////////////////////////////////////////

#[derive(Serialize)]
pub struct EntryPredictions {
    score: i16,
    enjoyment: i16,
}

impl EntryPredictions {
    fn from_entry(entry: &AnimeDetails, model: &Model<i16>) -> Self {
        let mut score_devs = 0;
        let mut score_devs_counter = 0;

        let mut percs = 0;
        let mut percs_counter = 0;

        let mut smean = 0;
        let mut smean_counter = 0;

        match &entry.airing_date {
            Some(date) => {
                let i = Indexer::date(date);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
                match entry.mean {
                    Some(mean) => {
                        smean += mean - model[i.x][i.y][1];
                        smean_counter += 1;
                    }
                    None => (),
                }
            }
            None => (),
        };

        match &entry.rating {
            Some(rating) => {
                let i = Indexer::rating(rating);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
                match entry.mean {
                    Some(mean) => {
                        smean += mean - model[i.x][i.y][1];
                        smean_counter += 1;
                    }
                    None => (),
                }
            }
            None => (),
        };

        match &entry.num_episodes {
            Some(num_episodes) => {
                let i = Indexer::num_episodes(num_episodes);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
                match entry.mean {
                    Some(mean) => {
                        smean += mean - model[i.x][i.y][1];
                        smean_counter += 1;
                    }
                    None => (),
                }
            }
            None => (),
        };

        match &entry.genres {
            Some(genres) => {
                for genre in genres.iter() {
                    match genre {
                        Some(g) => {
                            let i = Indexer::genre(g);
                            score_devs += model[i.x][i.y][2];
                            score_devs_counter += 1;
                            percs += model[i.x][i.y][0];
                            percs_counter += 1;
                            match entry.mean {
                                Some(mean) => {
                                    smean += mean - model[i.x][i.y][1];
                                    smean_counter += 1;
                                }
                                None => (),
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        };

        let score = match score_devs_counter {
            0 => 0,
            _ => match entry.mean {
                Some(mean) => mean + (score_devs / score_devs_counter),

                None => 0,
            },
        };

        let percentages = match percs_counter {
            0 => 0,
            _ => percs / percs_counter,
        };

        let meansdev = match smean_counter {
            0 => 0,
            _ => smean / smean_counter,
        };

        //let enjoyment = (((percentages + meansdev) as i32 * 100) / prediction_max as i32) as i16;
        let enjoyment = percentages + meansdev;
        Self { score, enjoyment }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Missing unique entries
////////////////////////////////////////////////////////////////////////////////

const MAX_ENTRIES_PER_USER: u8 = 32;

pub struct UniqueEntry {
    id: i32,
    users: Vec<u8>,
}

/// ### get unique entries with associated users
fn missing_unique_entries(
    user_list: &Vec<Vec<i32>>,
    similar_users: &Vec<AffinityUsers>,
    banned: &Vec<i32>,
) -> Vec<UniqueEntry> {
    let user_entries: Vec<i32> = user_list.iter().map(|x| x[0]).collect();

    let mut missing_entries_ids: Vec<i32> = vec![];
    let mut missing_entries: Vec<UniqueEntry> = vec![];

    for user_i in 0..similar_users.len() {
        let mut user_unique_entries = 0;
        let list_len = similar_users[user_i].list.len();

        for j in 0..list_len {
            let entry = &similar_users[user_i].list[j];

            if user_unique_entries > MAX_ENTRIES_PER_USER {
                break;
            }

            if
                // entry is dropped or on hold
                entry[1] > 3
                // user hash no episodes watched
                || entry[3] == 0
                // entry is banned
                || banned.contains(&entry[0])
                // entry is already in user's list
                || user_entries.contains(&entry[0])
            {
                continue;
            }

            // entry already recommended by another user
            if missing_entries_ids.contains(&entry[0]) {
                match missing_entries
                    .iter_mut()
                    .find(|e| e.id == entry[0])
                {
                    Some(entry) => entry.users.push(user_i as u8),
                    None => (),
                };
            // entry is first recommended by current user
            } else {
                missing_entries.push(UniqueEntry {
                    id: entry[0],
                    users: vec![user_i as u8],
                });
                missing_entries_ids.push(entry[0]);
                user_unique_entries += 1;
            }
        }
    }

    missing_entries
}
