use crate::helper;
use super::user_model::empty;

pub fn get_user_recommendations(model: [Vec<Vec<[i32; 9]>>; 2]) -> Result<Vec<String>, u16> {

    let mut gte = empty::model();
    let mut lte = empty::model();

    for x in 0..gte.len() {
        for y in 0..gte[x].len() {

            gte[x][y][0] = model[0][x][y][0] - 100;
            gte[x][y][1] = 69420;
            gte[x][y][2] = 69420;
            gte[x][y][3] = 69420;
            gte[x][y][4] = 69420;
            gte[x][y][5] = 69420;
            gte[x][y][6] = 69420;
            gte[x][y][7] = 69420;
            gte[x][y][8] = 69420;

            lte[x][y][0] = model[0][x][y][0] + 100;
            lte[x][y][1] = 69420;
            lte[x][y][2] = 69420;
            lte[x][y][3] = 69420;
            lte[x][y][4] = 69420;
            lte[x][y][5] = 69420;
            lte[x][y][6] = 69420;
            lte[x][y][7] = 69420;
            lte[x][y][8] = 69420;
        }
    }

    let affinity_model = [gte, lte];

    match helper::get_affinity_users(affinity_model) {
        Ok(v) => Ok(v),
        Err(_) => Err(500)
    }
}