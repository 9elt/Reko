use serde::Serialize;

use crate::algorithm::analysis::NormalDist;
use crate::algorithm::model::{Indexer, Model};
use crate::helper::AffinityUsers;
use crate::helper::{self, AnimeDetails};
use crate::utils::z_table;

#[derive(Serialize, Clone)]
pub struct Reko {
    id: i32,
    users: Vec<u8>,
    details: AnimeDetails,
    predictions: EntryPredictions,
}

pub struct RekoResult {
    pub recommendations: Vec<Reko>,
    pub banned_users: Vec<String>,
}

// ban user if < 4 recommendations
const MIN_ENTRIES_PER_USER: u8 = 4;

const MAX_ENTRIES_PER_USER: u8 = 32;

pub async fn extract(
    user_model: Model<i16>,
    user_list: Vec<Vec<i32>>,
    similar_users: &Vec<AffinityUsers>,
    banned: &Vec<i32>,
) -> Result<RekoResult, u16> {
    let entries = missing_unique_entries(&user_list, similar_users, banned);
    let entries_ids: Vec<i32> = entries.iter().map(|x| x.id).collect();
    let detailed = helper::get_anime_details(entries_ids.to_owned()).await;

    let mut recommendations = vec![];

    let normal_dist = match helper::get_normal_dist() {
        Ok(v) => v,
        Err(_) => return Err(500),
    };

    let mut score_distribution = [0; 11];
    // let score_mean = user_model[0][0][1] + user_model[0][0][2];
    for list_entry in user_list.iter() {
        let score_i = match list_entry[2] > 0 {
            true => (list_entry[2] / 100 - 1) as usize,
            false => 10,
        };
        score_distribution[score_i] += 1;
    }

    for entry_details in detailed.iter() {
        if is_sequel(entry_details) {
            continue;
        }

        let users = match entries
            .iter()
            .find(|missing_e| missing_e.id == entry_details.id)
        {
            Some(entry) => entry.users.to_owned(),
            None => vec![],
        };

        recommendations.push(Reko {
            id: entry_details.id,
            users,
            details: entry_details.to_owned(),
            predictions: EntryPredictions::new(
                entry_details,
                &user_model,
                &normal_dist,
                // &score_distribution,
            ),
        });
    }

    recommendations.sort_unstable_by_key(|x| 1000 - x.predictions.enjoyment);

    let mut parsed_reko: Vec<Reko> = vec![];
    // let min_exp = recommendations[0].predictions.enjoyment / 5;

    for reko in recommendations[0..48].iter() {
        parsed_reko.push(reko.to_owned());
    }

    let mut banned_users: Vec<String> = vec![];
    for u in 0..similar_users.len() {
        let n_entries: u8 = parsed_reko
            .iter()
            .map(|r| if r.users.contains(&(u as u8)) { 1 } else { 0 })
            .sum();

        if n_entries < MIN_ENTRIES_PER_USER {
            banned_users.push(similar_users[u].user_name.to_owned());
        }
    }

    Ok(RekoResult {
        recommendations: parsed_reko,
        banned_users,
    })
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

pub fn user_info(
    similar_users: &Vec<AffinityUsers>,
    user_model: &Model<i16>,
) -> Result<Vec<UsersInfo>, u16> {
    let normal_dist = match helper::get_normal_dist() {
        Ok(v) => v,
        Err(_) => return Err(500),
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
                        normal_dist.std_dev(x, y, z),
                    ) as i32;
                    counter += 1;
                }
            }
        }

        users_info.push(UsersInfo {
            user_name: similar.user_name.to_owned(),
            affinity: 100 - (tot_dev / counter / 10) as i16,
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

#[derive(Serialize, Clone)]
pub struct EntryPredictions {
    score: i16,
    enjoyment: i16,
}

impl EntryPredictions {
    fn new(
        entry: &AnimeDetails,
        model: &Model<i16>,
        normal_dist: &NormalDist,
        // score_distribution: &[i32; 11],
    ) -> Self {
        let mut enjoyment = 0;
        let mut score_dev = 0;
        let mut counter = 0;

        counter += stat_enj(
            &entry.airing_date,
            Indexer::date,
            &mut enjoyment,
            &mut score_dev,
            entry,
            model,
            normal_dist,
        );

        counter += stat_enj(
            &entry.rating,
            Indexer::rating,
            &mut enjoyment,
            &mut score_dev,
            entry,
            model,
            normal_dist,
        );

        counter += stat_enj(
            &entry.num_episodes,
            Indexer::num_episodes,
            &mut enjoyment,
            &mut score_dev,
            entry,
            model,
            normal_dist,
        );

        if let Some(genres) = &entry.genres {
            let mut genres_enj = 0;
            let mut genres_score_dev = 0;
            let mut genres_counter = 0;
            for genre in genres.iter() {
                genres_counter += stat_enj(
                    genre,
                    Indexer::genre,
                    &mut genres_enj,
                    &mut genres_score_dev,
                    entry,
                    model,
                    normal_dist,
                );
            }
            if genres_counter != 0 {
                enjoyment += genres_enj / genres_counter;
                score_dev += genres_score_dev / genres_counter;
                counter += 1;
            };
        }

        enjoyment = enjoyment / counter;
        score_dev = match entry.mean {
            Some(mean) => mean as i32 + (score_dev / counter),
            None => -1,
        };

        let r_score = (score_dev / 100) as usize - 1;
        let remainder = (score_dev - (r_score as i32 * 100)) / 10;

        // let r_ratio = score_distribution[r_score + 1] + score_distribution[r_score];
        // let max_rem = score_distribution[r_score] * 100
        //     / match r_ratio > 0 {
        //         true => r_ratio,
        //         false => 1,
        //     };

        let score = match remainder > 5 {
            true => r_score + 2,
            false => r_score + 1,
        };

        Self {
            score: score as i16,
            enjoyment: enjoyment as i16,
        }
    }
}

fn stat_enj<T>(
    value: &Option<T>,
    indexer: fn(value: &T) -> Indexer,
    enjoyment: &mut i32,
    score: &mut i32,
    entry: &AnimeDetails,
    model: &Model<i16>,
    normal_dist: &NormalDist,
) -> i32 {
    let counter: i32;

    match value {
        Some(date) => {
            let i = indexer(date);

            *enjoyment += value_probability(
                model[i.x][i.y][0],
                normal_dist.mean(i.x, i.y, 0),
                normal_dist.std_dev(i.x, i.y, 0),
            );

            if let Some(mean) = &entry.mean {
                let score_adj = mean - model[i.x][i.y][1];
                *enjoyment += match score_adj > 0 {
                    true => 0,
                    false => score_adj as i32,
                };
            }

            *score += model[i.x][i.y][2] as i32;

            counter = 1;
        }
        None => counter = 0,
    };

    counter
}

fn value_probability(value: i16, mean: i16, std_dev: i16) -> i32 {
    let z_score = (value as f32 - mean as f32) / std_dev as f32;
    ((z_table::cumulative_dist(z_score) - 0.5) * 1000.0) as i32
}

////////////////////////////////////////////////////////////////////////////////
// Missing unique entries
////////////////////////////////////////////////////////////////////////////////

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
                // user has no episodes watched
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
                match missing_entries.iter_mut().find(|e| e.id == entry[0]) {
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
