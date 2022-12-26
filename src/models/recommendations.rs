use serde::Serialize;

use crate::helper;
use crate::algorithm::user;

use super::stats;

use crate::helper::AffinityUsers;
use crate::algorithm::user::recommendation::EntryData;

#[derive(Serialize)]
pub struct DevResult {
    users: Vec<AffinityUsers>,
    recommendations: Vec<EntryData>,
}

pub async fn get_user_recommendations(user: &String, reload: bool) -> Result<DevResult, u16> {
    let stats_model = match stats::get_user_model(&user, reload).await {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let affinity_model = match user::affinity::affinity_model(&stats_model) {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let user_list = match helper::get_user_list(user) {
        Ok(list) => list.list(),
        Err(_) => return Err(500),
    };

    let similar_users = match helper::get_affinity_users(affinity_model, user) {
        Ok(v) => v,
        Err(_) => return Err(500),
    };

    match user::recommendation::extract(stats_model, user_list, &similar_users) {
        Ok(v) => {
            Ok(DevResult {
                users: similar_users,
                recommendations: v
            })
        },
        Err(error) => Err(error) 
    }
}