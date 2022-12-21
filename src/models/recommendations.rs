use crate::helper;
use crate::algorithm::model::affinity::AffinityModel;

pub fn get_user_recommendations(
    model: [Vec<Vec<[i32; 9]>>; 2],
    user: &String,
) -> Result<Vec<String>, u16> {
    let mut affinity_model: AffinityModel = AffinityModel::new(&model[0], &model[1]);

    match helper::get_affinity_users(affinity_model.calc(10).to_array(), user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}