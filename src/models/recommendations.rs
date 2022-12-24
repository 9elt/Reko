use crate::helper;
use crate::algorithm::model::affinity::AffinityModel;
use crate::algorithm::model::user::UserModel;

pub fn get_user_recommendations(
    model: UserModel,
    user: &String,
) -> Result<Vec<String>, u16> {

    let mut affinity_model = AffinityModel::new(model);
    affinity_model.calc(10);

    match helper::get_affinity_users(affinity_model.to_array(), user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}