use crate::helper;
use super::user_model::empty;

pub fn get_user_recommendations(model: [Vec<Vec<[i32; 9]>>; 2]) -> Result<Vec<String>, u16> {

    let mut gte = empty::model();
    let mut lte = empty::model();

    for x in 0..gte.len() {
        for y in 0..gte[x].len() {

            gte[x][y][0] = model[0][x][y][0] - 500;
            gte[x][y][1] = model[0][x][y][1] - 500;
            gte[x][y][2] = model[0][x][y][2] - 500;
            gte[x][y][3] = model[0][x][y][3] - 500;
            gte[x][y][4] = model[0][x][y][4] - 500;
            gte[x][y][5] = model[0][x][y][5] - 500;
            gte[x][y][6] = model[0][x][y][6] - 500;
            gte[x][y][7] = model[0][x][y][7] - 500;
            gte[x][y][8] = model[0][x][y][8] - 500;

            lte[x][y][0] = model[0][x][y][0] + 500;
            lte[x][y][1] = model[0][x][y][1] + 500;
            lte[x][y][2] = model[0][x][y][2] + 500;
            lte[x][y][3] = model[0][x][y][3] + 500;
            lte[x][y][4] = model[0][x][y][4] + 500;
            lte[x][y][5] = model[0][x][y][5] + 500;
            lte[x][y][6] = model[0][x][y][6] + 500;
            lte[x][y][7] = model[0][x][y][7] + 500;
            lte[x][y][8] = model[0][x][y][8] + 500;
        }
    }

    let affinity_model = [gte, lte];

    match helper::get_affinity_users(affinity_model) {
        Ok(v) => Ok(v),
        Err(_) => Err(500)
    }
}