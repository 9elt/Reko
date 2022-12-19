use time_elapsed;
use crate::helper;
use super::conversion::{
    date_to_index,
    genre_id_to_index,
    n_episodes_to_index,
    rating_to_index
};

use super::model_struct::UserModel;

struct EntryInfo<'a> {
    model: &'a mut UserModel,
    score: i32,
    deviation: i32,
    score_count: i32,
    status: usize,
}

/// # User statistics model
/// takes a user and generates his statistics model from his anime list
pub async fn stats_model(user: String, reload: bool) -> Result<UserModel, u16> {
    let mut time = time_elapsed::start("model");

    // retrieve list
    let list = match helper::get_detailed_list(&user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    time.log(format!("[{}] list retrieved", user)).timestamp();

    let mut model = UserModel::empty();

    for entry in list.iter() {
        let user_score: i32 = entry.score();
        let score: i32 = match entry.mean() {
            Some(mean) =>  mean as i32,
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
        pupulate_stat([0, 0], &mut entry_info);
        // status
        pupulate_stat([0, entry_info.status], &mut entry_info);
        // airing decades
        match entry.airing_date() {
            Some(data) => {
                pupulate_stat(date_to_index(data), &mut entry_info);
            }
            None => (),
        }
        // ratings
        match entry.rating() {
            Some(data) => {
                pupulate_stat(rating_to_index(data), &mut entry_info);
            }
            None => (),
        }
        // series length
        match entry.num_episodes() {
            Some(data) => {
                pupulate_stat(n_episodes_to_index(data), &mut entry_info);
            }
            None => (),
        }
        // genres | themes | demographics
        match entry.genres().to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(data) => {
                            pupulate_stat(genre_id_to_index(data), &mut entry_info);
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    time.log(format!("[{}] model polulation", user)).timestamp();

    let list_length = model[0][0][0];

    // completed
    average_stat(&mut model, 0, 1, list_length);
    // plan to watch
    average_stat(&mut model, 0, 2, list_length);
    // watching
    average_stat(&mut model, 0, 3, list_length);
    // on hold
    average_stat(&mut model, 0, 4, list_length);
    // dropped
    average_stat(&mut model, 0, 5, list_length);
    //  general stats
    average_stat(&mut model, 0, 0, -1);
    //  DETAILED STATS
    for x in 1..model.len() {
        let stat_tot_perc: i32 = model[x].iter().map(|i| i[0]).sum();
        for y in 0..model[x].len() {
            average_stat(&mut model, x, y, stat_tot_perc);
        }
    }

    time.log(format!("[{}] base model generation", user)).timestamp();

    helper::save_user_model(&user, model.copy_to_vec());

    time.log(format!("[{}] model saved", user)).timestamp();
    time.end();

    Ok(model)
}

/// # User standard deviation model
/// takes a user stats model and returns its standard deviation model
pub fn std_dev_model(base_model: &UserModel) -> UserModel {
    let mut avg_model = UserModel::average();
    for x in 0..avg_model.len() {
        for y in 0..avg_model[x].len() {
            for z in 0..avg_model[x][y].len() {
                let v = &base_model[x][y][z];
                let a = &avg_model[x][y][z];
                let interpolation = match v + a {
                    -25 => 26,
                    _ => 25
                };
                avg_model[x][y][z] = ((v - a) * 100) / (v + a + interpolation);
            }
        }
    }
    avg_model[0][6] = [0; 9];
    avg_model
}

////////////////////////////////////////////////////////////////////////////////
// Utilities
////////////////////////////////////////////////////////////////////////////////
fn average_stat(
    model: &mut UserModel,
    stat_type: usize,
    stat: usize,
    stat_tot: i32,
) {
    model[stat_type][stat][1] = div(model[stat_type][stat][1], model[stat_type][stat][0]);
    //  total average score deviation
    model[stat_type][stat][2] = div(model[stat_type][stat][2], model[stat_type][stat][3]);
    //  total scored percentage
    model[stat_type][stat][3] = perc(model[stat_type][stat][3], model[stat_type][stat][0]);
    if stat_type > 0 {
        //  statuses percentages
        model[stat_type][stat][4] = perc(model[stat_type][stat][4], model[stat_type][stat][0]);
        model[stat_type][stat][5] = perc(model[stat_type][stat][5], model[stat_type][stat][0]);
        model[stat_type][stat][6] = perc(model[stat_type][stat][6], model[stat_type][stat][0]);
        model[stat_type][stat][7] = perc(model[stat_type][stat][7], model[stat_type][stat][0]);
        model[stat_type][stat][8] = perc(model[stat_type][stat][8], model[stat_type][stat][0]);
    }
    if stat_type > 0 && stat > 0 {
        //  stat percentage
        model[stat_type][stat][0] = perc(model[stat_type][stat][0], stat_tot);
    }
}

fn pupulate_stat(
    indexes: [usize; 2],
    e: &mut EntryInfo
) {
    e.model[indexes[0]][indexes[1]][0] += 1;
    e.model[indexes[0]][indexes[1]][1] += e.score;
    e.model[indexes[0]][indexes[1]][2] += e.deviation;
    e.model[indexes[0]][indexes[1]][3] += e.score_count;
    if indexes[0] != 0 {
        e.model[indexes[0]][indexes[1]][e.status + 3] += 1
    };
}

fn div(num: i32, den: i32) -> i32 {
    match den {
        0 => 0,
        _ => num / den
    }
}

fn perc(num: i32, den: i32) -> i32 {
    div(num * 1000, den)
}