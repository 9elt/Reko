use crate::data::fun::get_detailed_list;
use crate::data::db::user::set_model;

use super::cast::base::{date_to_index, genre_id_to_index, n_episodes_to_index, rating_to_index};

use super::empty::new_model;
use crate::utils::benchmark;

type BaseModel = Vec<Vec<[i32; 9]>>;

fn pupulate_stat(
    mut m: BaseModel,
    x: usize,
    y: usize,
    s: i32,
    d: i32,
    c: i32,
    st: Option<usize>,
) -> BaseModel {
    m[x][y][0] += 1;
    m[x][y][1] += s;
    m[x][y][2] += d;
    m[x][y][3] += c;
    match st {
        Some(st) => m[x][y][st] += 1,
        None => (),
    };
    m
}

pub async fn generate_base_model(s_user: String, reload: bool) -> Result<BaseModel, u16> {
    let mut benchmark = benchmark::Time::start("model generation");

    let list = match get_detailed_list(&s_user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    benchmark.millis(format!("[{}] list retrieved", s_user));

    let mut model: BaseModel = new_model();

    for i in 0..list.len() {
        let status: usize = list[i].entry.status as usize;
        let st_idx: usize = status + 3;

        let score: i32 = list[i].entry.score;

        let dev: i32 = match list[i].details.mean {
            Some(mean) => match score {
                0 => 0,
                _ => score - mean as i32,
            },
            None => 0,
        };

        let s_cnt = match score {
            0 => 0,
            _ => 1,
        };

        //  general stats > Score Stats
        model = pupulate_stat(model, 0, 0, score, dev, s_cnt, None);

        //  general stats > Status Stats
        model = pupulate_stat(model, 0, status, score, dev, s_cnt, None);

        //  detailed stats > airing decades
        match list[i].details.airing_date {
            Some(data) => {
                let v: [usize; 2] = date_to_index(data);
                model = pupulate_stat(model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > ratings
        match list[i].details.rating {
            Some(data) => {
                let v: [usize; 2] = rating_to_index(data);
                model = pupulate_stat(model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > series length
        match list[i].details.num_episodes {
            Some(data) => {
                let v: [usize; 2] = n_episodes_to_index(data);
                model = pupulate_stat(model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
            }
            None => (),
        }

        //  detailed stats > genres | themes | demographics
        match list[i].details.genres.to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(data) => {
                            let v: [usize; 2] = genre_id_to_index(data);
                            model =
                                pupulate_stat(model, v[0], v[1], score, dev, s_cnt, Some(st_idx));
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    benchmark.millis(format!("[{}] model polulation", s_user));

    //  general stats > statuses
    for i in 1..6 {
        //  status average score
        model[0][i][1] = match model[0][i][3] {
            0 => 0,
            _ => model[0][i][1] / model[0][i][3],
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
    model[0][0][1] = match model[0][0][3] {
        0 => 0,
        _ => model[0][0][1] / model[0][0][3],
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
            model[x][y][1] = match model[x][y][3] {
                0 => 0,
                _ => model[x][y][1] / model[x][y][3],
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

    benchmark.micros(format!("[{}] model generation", s_user));

    set_model(&s_user, model.to_owned());

    Ok(model)
}
