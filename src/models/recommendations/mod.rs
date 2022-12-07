mod affinity;

use crate::helper;

type Model = Vec<Vec<[i32; 9]>>;

pub fn get_user_recommendations(model: [Vec<Vec<[i32; 9]>>; 2], user: &String) -> Result<Vec<String>, u16> {

    let affinity_model = calc_affinity_model(&model[0], &model[1]);

    match helper::get_affinity_users(affinity_model, user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500)
    }
}

fn calc_affinity_model(values: &Model, avgs: &Model) -> [Model; 2] {

    let mut gte: Model = affinity::model();
    let mut lte: Model = affinity::model();

    let score_is_relavant = values[0][0][3] > 250;

    gte[0][0][0] = values[0][0][0] - 300;
    lte[0][0][0] = 100 + values[0][0][0] * 4;

    // general stats
    if score_is_relavant {
        gte[0][0][1] = values[0][0][1] - 25;
        lte[0][0][1] = values[0][0][1] + 25;

        gte[0][0][2] = values[0][0][2] - 15;
        lte[0][0][2] = values[0][0][2] + 15;
    }

    //  general Statuses stats

    //  for y in 1..gte[0].len() {
    //      gte[0][y][0] = values[0][y][0] - 100;
    //      lte[0][y][0] = values[0][y][0] + 100;
    //  }

    //  detailed stats
    for x in 1..gte.len() {
        for y in 0..gte[x].len() {

            if (avgs[x][y][0] > 60 && values[x][y][0] > 3)
                || (avgs[x][y][0] > 25 && values[x][y][0] > 10)
                || (avgs[x][y][0] < -100 && values[x][y][0] > 3)
                || (avgs[x][y][0] < -25 && values[x][y][0] > 10)
            {
                gte[x][y][0] = values[x][y][0] - 150;
                lte[x][y][0] = values[x][y][0] + 150;

                if score_is_relavant {
                    gte[x][y][1] = values[x][y][1] - 150;
                    lte[x][y][2] = values[x][y][2] + 150;
                }
            }
        }
    }

    [gte, lte]
}