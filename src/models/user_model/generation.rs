use crate::utils::time_elapsed;
use crate::helper;
use super::init;
use super::conversion::{
    date_to_index,
    genre_id_to_index,
    n_episodes_to_index,
    rating_to_index
};

type UserModel = Vec<Vec<[i32; 9]>>;

pub async fn base_model(user: String, reload: bool) -> Result<UserModel, u16> {
    let mut time = time_elapsed::start("model");

    let list = match helper::get_detailed_list(&user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    time.log(format!("[{}] list retrieved", user)).timestamp();

    let mut model: UserModel = init::empty();

    for i in 0..list.len() {

        let status: usize = list[i].status() as usize;
        let st_idx: usize = status + 3;

        let user_score: i32 = list[i].score();

        let score: i32 = match list[i].mean() {
            Some(mean) =>  mean as i32,
            None => continue,
        };

        let dev: i32 = match list[i].mean() {
            Some(mean) => match user_score {
                0 => 0,
                _ => user_score - mean as i32,
            },
            None => 0,
        };

        let s_cnt = match user_score {
            0 => 0,
            _ => 1,
        };

        //  general stats > Score Stats
        pupulate_stat(&mut model, 0, 0, score, dev, s_cnt, None);

        //  general stats > Status Stats
        pupulate_stat(&mut model, 0, status, score, dev, s_cnt, None);

        //  detailed stats > airing decades
        match list[i].airing_date() {
            Some(data) => {
                let v: [usize; 2] = date_to_index(data);
                pupulate_stat(&mut model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > ratings
        match list[i].rating() {
            Some(data) => {
                let v: [usize; 2] = rating_to_index(data);
                pupulate_stat(&mut model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > series length
        match list[i].num_episodes() {
            Some(data) => {
                let v: [usize; 2] = n_episodes_to_index(data);
                pupulate_stat(&mut model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > genres | themes | demographics
        match list[i].genres().to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(data) => {
                            let v: [usize; 2] = genre_id_to_index(data);
                            pupulate_stat(&mut model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
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

    time.log(format!("[{}] base model generation", user)).timestamp();

    helper::save_user_model(&user, model.to_owned());

    time.log(format!("[{}] model saved", user)).timestamp();

    time.end();

    Ok(model)
}

fn pupulate_stat(
    m: &mut UserModel,
    stat_type: usize,
    stat: usize,
    score: i32,
    deviation: i32,
    scored_perc: i32,
    status: Option<usize>,
) {
    m[stat_type][stat][0] += 1;
    m[stat_type][stat][1] += score;
    m[stat_type][stat][2] += deviation;
    m[stat_type][stat][3] += scored_perc;
    match status {
        Some(s) => m[stat_type][stat][s] += 1,
        None => (),
    };
}

pub fn std_dev_model(base_model: &UserModel) -> UserModel {
    let mut avg_model = init::average();

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

    avg_model
}