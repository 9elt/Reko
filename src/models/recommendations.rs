use crate::helper;
use crate::algorithm::fucker::affinity::AffinityModel;

use crate::algorithm::{user, model::Model};

use super::stats;

type devResult = Vec<String>;

pub async fn get_user_recommendations(user: &String, reload: bool) -> Result<devResult, u16> {
    let stats_model = match stats::get_user_model(&user, reload).await {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let mut affinity_model = user::AffinityModel::new(stats_model);

    affinity_model.calc(10);

    match helper::get_affinity_users(affinity_model.to_array(), user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}