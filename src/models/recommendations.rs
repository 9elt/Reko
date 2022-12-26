use crate::helper;
use crate::algorithm::user;

use super::stats;

type DevResult = Vec<String>;

pub async fn get_user_recommendations(user: &String, reload: bool) -> Result<DevResult, u16> {
    let stats_model = match stats::get_user_model(&user, reload).await {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let affinity_model = match user::affinity::affinity_model(stats_model) {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    match helper::get_affinity_users(affinity_model, user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}