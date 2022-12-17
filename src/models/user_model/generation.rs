use crate::helper;
use crate::utils::time_elapsed;

use super::model_struct::{ModelHelper, UserModel};

pub fn pop_stat(stat: ModelHelper, info: EntryInfo, has_status: bool) {
    stat.incr_percentage();
    stat.incr_score(info.score);
    stat.incr_score_deviation(info.deviation);
    stat.incr_scored_percentage(info.score_count);
    if has_status {
        stat.incr_status(info.status);
    };
}

struct EntryInfo {
    score: i32,
    deviation: i32,
    score_count: i32,
    status: usize,
}

pub async fn generate_base_model(user: String, reload: bool) -> Result<UserModel, u16> {
    let mut time = time_elapsed::start("model");

    let list = match helper::get_detailed_list(&user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    time.log(format!("[{}] list retrieved", user)).timestamp();

    let mut model = UserModel::empty();

    for i in 0..list.len() {
        let user_score: i32 = list[i].score();
        let score: i32 = match list[i].mean() {
            Some(mean) => mean as i32,
            None => continue,
        };

        let entry_info = EntryInfo {
            score,
            deviation: match user_score {
                0 => 0,
                _ => user_score - score as i32,
            },
            score_count: match user_score {
                0 => 0,
                _ => 1,
            },
            status: list[i].status() as usize,
        };

        pop_stat(model.general(), entry_info, false);

        pop_stat(model.general_status(entry_info.status), entry_info, false);

        match list[i].airing_date() {
            Some(data) => {
                pop_stat(model.date_to_model_airing_decade(data), entry_info, true);
            }
            None => (),
        }

        match list[i].rating() {
            Some(data) => {
                pop_stat(model.rating_id_to_rating(data), entry_info, true);
            }
            None => (),
        }

        match list[i].num_episodes() {
            Some(data) => {
                pop_stat(model.n_episodes_to_series_length(data), entry_info, true);
            }
            None => (),
        }

        match list[i].genres() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(data) => {
                            pop_stat(model.genre_id_to_genres(data), entry_info, true);
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    time.log(format!("[{}] model polulation", user)).timestamp();

    //  general stats > statuses
    for i in 1..6 {
        //  status average score
        model[0][i][1] = match model[0][i][0] {
            0 => 0,
            _ => model[0][i][1] / model[0][i][0],
        };
        //  total average score deviation
        model[0][i][2] = match model[0][i][3] {
            0 => 0,
            _ => model[0][i][2] / model[0][i][3],
        };
        //  total scored percentage
        model[0][i][3] = match model[0][i][0] {
            0 => 0,
            _ => model[0][i][3] * 1000 / model[0][i][0],
        };
        //  status percentage
        model[0][i][0] = match model[0][0][0] {
            0 => 0,
            _ => model[0][i][0] * 1000 / model[0][0][0],
        };
    }

    //  total average score
    model[0][0][1] = match model[0][0][0] {
        0 => 0,
        _ => model[0][0][1] / model[0][0][0],
    };

    //  total average score deviation
    model[0][0][2] = match model[0][0][3] {
        0 => 0,
        _ => model[0][0][2] / model[0][0][3],
    };

    //  stotal scored percentage
    model[0][0][3] = match model[0][0][0] {
        0 => 0,
        _ => model[0][0][3] * 1000 / model[0][0][0],
    };

    //  detailed stats
    for x in 1..model.len() {
        let mut tot_watched: i32 = 0;
        for y in 0..model[x].len() {
            tot_watched += model[x][y][0];
        }

        for y in 0..model[x].len() {
            //  average score
            model[x][y][1] = match model[x][y][0] {
                0 => 0,
                _ => model[x][y][1] / model[x][y][0],
            };
            //  total average score deviation
            model[x][y][2] = match model[x][y][3] {
                0 => 0,
                _ => model[x][y][2] / model[x][y][3],
            };
            //  total scored percentage
            model[x][y][3] = match model[x][y][0] {
                0 => 0,
                _ => model[x][y][3] * 1000 / model[x][y][0],
            };
            //  statuses percentages
            match model[x][y][0] {
                0 => (),
                _ => {
                    model[x][y][4] = model[x][y][4] * 1000 / model[x][y][0];
                    model[x][y][5] = model[x][y][5] * 1000 / model[x][y][0];
                    model[x][y][6] = model[x][y][6] * 1000 / model[x][y][0];
                    model[x][y][7] = model[x][y][7] * 1000 / model[x][y][0];
                    model[x][y][8] = model[x][y][8] * 1000 / model[x][y][0];
                }
            }
            //  stat percentage
            model[x][y][0] = match tot_watched {
                0 => 0,
                _ => model[x][y][0] * 1000 / tot_watched,
            };
        }
    }

    time.log(format!("[{}] base model generation", user))
        .timestamp();

    helper::save_user_model(&user, model.to_owned());

    time.log(format!("[{}] model saved", user)).timestamp();

    time.end();

    Ok(model)
}
