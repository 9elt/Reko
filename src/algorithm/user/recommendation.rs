use serde::{Deserialize, Serialize};

use crate::algorithm::model::{Model, Indexer};
use crate::helper::{self, AnimeDetails};
use crate::helper::AffinityUsers;

#[derive(Serialize)]
pub struct Reko {
    id: i32,
    users: Vec<u8>,
    details: AnimeDetails,
    expected: Expected,
}

pub async fn extract(
    user_model: Model<i16>,
    user_list: Vec<Vec<i32>>,
    similar_users: &Vec<AffinityUsers>,
) -> Result<Vec<Reko>, u16> {
    let entries = get_entries(&user_list, similar_users);
    let entries_ids: Vec<i32> = entries.iter().map(|x| x.id).collect();
    let detailed = helper::get_anime_details(entries_ids.to_owned()).await;

    let mut recommendations = vec![];

    for e in detailed.iter() {
        let related = match &e.related {
            Some(r) => r.to_owned(),
            None => vec![]
        };
        let mut skip = false;
        for rel in related.iter() {
            if rel.relation > 6 || entries_ids.contains(&(rel.id as i32)) {
                skip = true;
                break;
            }
        }
        if skip {
            continue;
        }

        let mut users: Vec<u8> = vec![];
        for ent in entries.iter() {
            if ent.id == e.id {
                users = ent.users.to_owned();
                break;
            }
        }

        recommendations.push(
            Reko {
                id: e.id,
                users,
                details: e.to_owned(),
                expected: Expected::from_entry(e, &user_model)
            }
        );
    }

    recommendations.sort_unstable_by_key(|x| 1000 - x.expected.enjoyment);

    let mut parsed_reko: Vec<Reko> = vec![];
    let min_exp = recommendations[0].expected.enjoyment * 12 / 20;

    for reko in recommendations {
        if reko.expected.enjoyment < min_exp {
            break;
        }
        parsed_reko.push(reko);
    }

    Ok(parsed_reko)
}

#[derive(Serialize)]
pub struct Expected {
    score: i16,
    enjoyment: i16
}

impl Expected {
    fn from_entry(entry: &AnimeDetails, model: &Model<i16>) -> Self {

        let mut score_devs = 0;
        let mut score_devs_counter = 0;

        let mut percs = 0;
        let mut percs_counter = 0;

        match &entry.airing_date {
            Some(date) => {
                let i = Indexer::date(date);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
            },
            None => (),
        };

        match &entry.rating {
            Some(rating) => {
                let i = Indexer::rating(rating);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
            },
            None => (),
        };

        match &entry.num_episodes {
            Some(num_episodes) => {
                let i = Indexer::num_episodes(num_episodes);
                score_devs += model[i.x][i.y][2];
                score_devs_counter += 1;
                percs += model[i.x][i.y][0];
                percs_counter += 1;
            },
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
                        },
                        None => ()
                    }
                }
            },
            None => (),
        };

        let score = match score_devs_counter {
            0 => 0,
            _ => {
                match entry.mean {
                    Some(mean) => {
                        mean + (score_devs / score_devs_counter)
                    },

                    None => 0
                }
            }
        };

        let enjoyment = match percs_counter {
            0 => 0,
            _ => percs / percs_counter
        }; 

        Self { score, enjoyment }
    }
}


#[derive(Serialize, Deserialize)]
pub struct EntryData {
    id: i32,
    users: Vec<u8>,
}

fn get_entries(user_list: &Vec<Vec<i32>>, similar_users: &Vec<AffinityUsers>) -> Vec<EntryData> {
    let user_entries: Vec<i32> = user_list.iter().map(|x| x[0]).collect();

    let mut missing_entries: Vec<i32> = vec![];
    let mut missing_entries_with_data: Vec<EntryData> = vec![];

    for i in 0..similar_users.len() {
        let mut user_unique = 0;
        let list_len = similar_users[i].list.len();
        let limit = match list_len > 1000 {
            true => 1000,
            false => list_len,
        };

        for j in 0..limit {
            let entry = &similar_users[i].list[j];

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
                missing_entries_with_data.push(EntryData {
                    id: entry[0],
                    users: vec![i as u8],
                });
                missing_entries.push(entry[0]);
                user_unique += 1;
            }
        }
    }

    missing_entries_with_data
}
