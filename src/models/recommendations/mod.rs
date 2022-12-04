use crate::helper;
use super::user_model::empty;

pub fn get_user_recommendations(model: [Vec<Vec<[i32; 9]>>; 2]) -> Result<Vec<String>, u16> {

    let mut gte = empty::model();
    let mut lte = empty::model();

    for x in 0..gte.len() {
        for y in 0..gte[x].len() {
            for z in 0..gte[x][y].len() {
                gte[x][y][z] = model[0][x][y][z] - 1000;
                lte[x][y][z] = model[0][x][y][z] + 1000;
            }
        }
    }

    let affinity_model = [gte, lte];

    match helper::get_affinity_users(affinity_model) {
        Ok(v) => Ok(v),
        Err(_) => Err(500)
    }
}