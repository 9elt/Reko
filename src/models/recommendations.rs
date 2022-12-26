use serde::Serialize;

use crate::helper;
use crate::algorithm::user;

use super::stats;

use crate::helper::AffinityUsers;
use crate::algorithm::user::recommendation::Reko;

#[derive(Serialize)]
pub struct DevResult {
    passages: u8,
    users: Vec<String>,
    recommendations: Vec<Reko>,
}

pub async fn get_user_recommendations(user: &String, reload: bool) -> Result<DevResult, u16> {
    let stats_model = match stats::get_user_model(&user, reload).await {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let user_list = match helper::get_user_list(user) {
        Ok(list) => list.list(),
        Err(_) => return Err(500),
    };

    let mut passages = 0;
    let mut similar_users: Vec<AffinityUsers> = vec![];
    for a in 0..10 {
        passages += 1;
        let accuracy = 100 - (a * 2);

        let affinity_model = match user::affinity::affinity_model(&stats_model, accuracy) {
            Ok(model) => model,
            Err(error) => return Err(error),
        };

        similar_users = match helper::get_affinity_users(affinity_model, user) {
            Ok(v) => v,
            Err(_) => return Err(500),
        };

        if similar_users.len() > 2 {
            break;
        }
    }

    match user::recommendation::extract(stats_model, user_list, &similar_users).await {
        Ok(v) => {
            Ok(DevResult {
                passages,
                users: similar_users.iter().map(|u| u.user_name.to_owned()).collect(),
                recommendations: v
            })
        },
        Err(error) => Err(error) 
    }
}