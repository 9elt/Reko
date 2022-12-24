use time_elapsed;

use crate::helper;

use super::indexer::Idx;
use super::Model;

struct EntryInfo<'a> {
    model: &'a mut Model,
    score: i32,
    deviation: i32,
    score_count: i32,
    status: usize,
}

/// # User statistics model
/// Generates a statistics model from an anime list
pub async fn stats_model(user: String, reload: bool) -> Result<Model, u16> {
    let time = time_elapsed::start("stats");

    let list = match helper::get_detailed_list(&user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    let mut model = Model::empty();

    for entry in list.iter() {
        let user_score: i32 = entry.score();

        let score: i32 = match entry.mean() {
            Some(mean) => mean as i32,
            None => continue,
        };

        let mut entry_info = EntryInfo {
            model: &mut model,
            score,
            deviation: match user_score {
                0 => 0,
                _ => user_score - score as i32,
            },
            score_count: match user_score {
                0 => 0,
                _ => 1,
            },
            status: entry.status() as usize,
        };

        // general
        pupulate_stat(Idx::general(), &mut entry_info);

        // status
        pupulate_stat(Idx::from_status(entry_info.status), &mut entry_info);

        // airing decade
        match entry.airing_date() {
            Some(data) => {
                pupulate_stat(Idx::from_date(data), &mut entry_info);
            }
            None => (),
        }

        // rating
        match entry.rating() {
            Some(data) => {
                pupulate_stat(Idx::from_rating(data), &mut entry_info);
            }
            None => (),
        }

        // series length
        match entry.num_episodes() {
            Some(data) => {
                pupulate_stat(Idx::from_num_episodes(data), &mut entry_info);
            }
            None => (),
        }

        // genres | themes | demographics
        match entry.genres().to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(data) => {
                            pupulate_stat(Idx::from_genre(data), &mut entry_info);
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    let list_length = model[0][0][0];

    // completed | plan to watch | watching | on hold | dropped
    for status in 1..6 {
        average_stat(&mut model, 0, status, list_length);
    }

    // general stats
    average_stat(&mut model, 0, 0, -1);

    // detailed stats
    for x in 1..model.len() {
        let stat_tot: i32 = model[x].iter().map(|i| i[0]).sum();
        for y in 0..model[x].len() {
            average_stat(&mut model, x, y, stat_tot);
        }
    }

    // errors reset
    model[0][6] = [0; 9];

    helper::save_user_model(&user, model.copy_to_vec());

    time.end();

    Ok(model)
}

fn average_stat(model: &mut Model, stat_type: usize, stat: usize, stat_tot: i32) {
    // mal score
    model[stat_type][stat][1] = div(model[stat_type][stat][1], model[stat_type][stat][0]);

    // average score deviation
    model[stat_type][stat][2] = div(model[stat_type][stat][2], model[stat_type][stat][3]);

    // scored percentage
    model[stat_type][stat][3] = perc(model[stat_type][stat][3], model[stat_type][stat][0]);

    // completed | plan to watch | watching | on hold | dropped percentages
    if stat_type > 0 {
        model[stat_type][stat][4] = perc(model[stat_type][stat][4], model[stat_type][stat][0]);
        model[stat_type][stat][5] = perc(model[stat_type][stat][5], model[stat_type][stat][0]);
        model[stat_type][stat][6] = perc(model[stat_type][stat][6], model[stat_type][stat][0]);
        model[stat_type][stat][7] = perc(model[stat_type][stat][7], model[stat_type][stat][0]);
        model[stat_type][stat][8] = perc(model[stat_type][stat][8], model[stat_type][stat][0]);
    }

    // stat percentage
    if stat_type > 0 && stat > 0 {
        model[stat_type][stat][0] = perc(model[stat_type][stat][0], stat_tot);
    }
}

fn pupulate_stat(idx: Idx, e: &mut EntryInfo) {
    e.model[idx.x][idx.y][0] += 1;
    e.model[idx.x][idx.y][1] += e.score;
    e.model[idx.x][idx.y][2] += e.deviation;
    e.model[idx.x][idx.y][3] += e.score_count;

    if idx.x != 0 {
        e.model[idx.x][idx.y][e.status + 3] += 1;
    }
}

fn div(num: i32, den: i32) -> i32 {
    match den {
        0 => 0,
        _ => num / den,
    }
}

fn perc(num: i32, den: i32) -> i32 {
    div(num * 1000, den)
}
