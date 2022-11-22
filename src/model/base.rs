use crate::data::fun::get_detailed_list;
use std::time::Instant;

use super::cast::base::{
    date_to_model_index, genre_id_to_model_index, n_episodes_to_model_index, rating_to_model_index,
};

type BaseModel = Vec<Vec<Vec<i32>>>;

pub async fn generate_base_model(s_user: String, reload: bool) -> Result<BaseModel, u16> {
    let start = Instant::now();

    let list = match get_detailed_list(s_user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    println!(
        "generate_base_model > list retrieved in {} μs",
        start.elapsed().as_micros()
    );

    let mut model: BaseModel = new_model();
    let mut count: [i32; 6] = [0, 0, 0, 0, 0, 0];
    let mut score_count: [i32; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..list.len() {
        let status = (list[i].entry.status + 1) as usize;
        let score = list[i].entry.score;

        model[6][0][status] += 1;

        match score {
            0 => (),
            _ => match list[i].details.mean {
                Some(mean) => {
                    let deviation = (score as i16 - mean) as i32;
                    model[6][0][0] += score as i32;
                    model[6][0][1] += deviation;

                    model[6][status - 1][0] += score as i32;
                    model[6][status - 1][1] += deviation;

                    score_count[0] += 1;
                    score_count[status - 1] += 1;
                }
                None => (),
            },
        }

        //airing decade
        match list[i].details.airing_date {
            Some(a) => {
                let d = date_to_model_index(a);
                model[d[0]][d[1]][0] += score as i32;
                model[d[0]][d[1]][status] += 1;
                count[d[0]] += 1;
                match score {
                    0 => (),
                    _ => model[d[0]][d[1]][1] += 1,
                };
            }
            None => (),
        }

        //rating
        match list[i].details.rating {
            Some(a) => {
                let d = rating_to_model_index(a);
                model[d[0]][d[1]][0] += score as i32;
                model[d[0]][d[1]][status] += 1;
                count[d[0]] += 1;
                match score {
                    0 => (),
                    _ => model[d[0]][d[1]][1] += 1,
                };
            }
            None => (),
        }

        //number of episodes
        match list[i].details.num_episodes {
            Some(a) => {
                let d = n_episodes_to_model_index(a);
                model[d[0]][d[1]][0] += score as i32;
                model[d[0]][d[1]][status] += 1;
                count[d[0]] += 1;
                match score {
                    0 => (),
                    _ => model[d[0]][d[1]][1] += 1,
                };
            }
            None => (),
        }

        //genres
        match list[i].details.genres.to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(g) => {
                            let d = genre_id_to_model_index(g);
                            model[d[0]][d[1]][0] += score as i32;
                            model[d[0]][d[1]][status] += 1;
                            count[d[0]] += 1;
                            match score {
                                0 => (),
                                _ => model[d[0]][d[1]][1] += 1,
                            };
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    println!(
        "generate_base_model > model population done in {} μs",
        start.elapsed().as_micros()
    );

    for i in 0..6 {
        for c in 0..model[i].len() {
            let tot =
                model[i][c][2] + model[i][c][3] + model[i][c][4] + model[i][c][5] + model[i][c][6];

            model[i][c][2] = model[i][c][2] * 1000 / count[i];
            model[i][c][3] = model[i][c][3] * 1000 / count[i];
            model[i][c][4] = model[i][c][4] * 1000 / count[i];
            model[i][c][5] = model[i][c][5] * 1000 / count[i];
            model[i][c][6] = model[i][c][6] * 1000 / count[i];

            model[i][c][0] = match model[i][c][1] {
                0 => 0,
                _ => model[i][c][0] / model[i][c][1],
            };

            model[i][c][1] = match tot {
                0 => 0,
                _ => model[i][c][1] * 1000 / tot,
            };
        }
    }

    model[6][0][0] = match score_count[0] {
        0 => 0,
        _ => model[6][0][0] / score_count[0],
    };
    model[6][0][1] = match score_count[1] {
        0 => 0,
        _ => model[6][0][1] / score_count[1],
    };

    let totpct = model[6][0][2] + model[6][0][3] + model[6][0][4] + model[6][0][5] + model[6][0][6];

    for i in 1..model[6].len() {
        model[6][i][2] = match model[6][0][i + 1] {
            0 => 0,
            _ => score_count[i] * 1000 / model[6][0][i + 1],
        };

        model[6][0][i + 1] = match totpct {
            0 => 0,
            _ => model[6][0][i + 1] * 1000 / totpct,
        };

        match score_count[i] {
            0 => (),
            _ => {
                model[6][i][0] = model[6][i][0] / score_count[i];
                model[6][i][1] = model[6][i][1] / score_count[i];
            }
        }
    }

    println!(
        "generate_base_model > base model done in {} μs",
        start.elapsed().as_micros()
    );

    Ok(model)
}

fn new_model() -> BaseModel {
    vec![
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ],
    ]
}
