mod affinity;

use crate::helper;

pub fn get_user_recommendations(model: [Vec<Vec<[i32; 9]>>; 2], user: &String) -> Result<Vec<String>, u16> {

    let mut gte = affinity::model();
    let mut lte = affinity::model();

    let model_values = &model[0];
    let model_avgs = &model[1];

    //let needs_big_list = model_values[0][0][0] > 500;
    let score_is_relavant = model_values[0][0][3] > 250;

    gte[0][0][0] = model_values[0][0][0] - 300;
    lte[0][0][0] = 100 + model_values[0][0][0] * 4;

    // general stats
    if score_is_relavant {
        gte[0][0][1] = model_values[0][0][1] - 25;
        lte[0][0][1] = model_values[0][0][1] + 25;

        gte[0][0][2] = model_values[0][0][2] - 15;
        lte[0][0][2] = model_values[0][0][2] + 15;
    }

    //  general Statuses stats
    //  for y in 1..gte[0].len() {
    //      gte[0][y][0] = model_values[0][y][0] - 100;
    //      lte[0][y][0] = model_values[0][y][0] + 100;
    //  }

    //  detailed stats
    for x in 1..gte.len() {
        for y in 0..gte[x].len() {

            if (model_avgs[x][y][0] > 60 && model_values[x][y][0] > 3)
                || (model_avgs[x][y][0] > 25 && model_values[x][y][0] > 10)
                || (model_avgs[x][y][0] < -100 && model_values[x][y][0] > 3)
                || (model_avgs[x][y][0] < -25 && model_values[x][y][0] > 10)
            {
                gte[x][y][0] = model_values[x][y][0] - 150;
                lte[x][y][0] = model_values[x][y][0] + 150;

                if score_is_relavant {
                    gte[x][y][1] = model_values[x][y][1] - 150;
                    lte[x][y][2] = model_values[x][y][2] + 150;
                }
            }
        }
    }

    let affinity_model = [gte, lte];

    match helper::get_affinity_users(affinity_model, user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500)
    }
}