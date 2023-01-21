use serde::Serialize;

use crate::algorithm::user;
use crate::helper;

use super::stats;

use crate::algorithm::user::recommendation::Reko;
use crate::algorithm::user::recommendation::UsersInfo;
use crate::controllers::public::RecommendationsSettings;
use crate::helper::AffinityUsers;

#[derive(Serialize)]
pub struct NextRequest {
    banned_ids: Vec<i32>,
    banned_users: Vec<String>,
    accuracy: i32,
    force_list_update: bool,
}

#[derive(Serialize)]
pub struct DevResult {
    next_request: NextRequest,
    users: Vec<UsersInfo>,
    recommendations: Vec<Reko>,
}

pub async fn get_user_recommendations(
    user: &String,
    settings: &RecommendationsSettings,
) -> Result<DevResult, u16> {
    let stats_model = match stats::get_user_model(user, &settings.force_update()).await {
        Ok(model) => model,
        Err(error) => return Err(error),
    };

    let user_list = match helper::get_user_list(user) {
        Ok(list) => list.list(),
        Err(_) => return Err(500),
    };

    let init_accuracy: i32 = settings.accuracy();
    let mut final_accuracy: i32 = 100;

    let mut similar_users: Vec<AffinityUsers> = vec![];

    for a in 0..10 {
        final_accuracy = init_accuracy - (a * 2);
        let affinity_model = match user::affinity::affinity_model(&stats_model, final_accuracy) {
            Ok(model) => model,
            Err(error) => return Err(error),
        };

        similar_users =
            match helper::get_affinity_users(affinity_model, user, &settings.banned_users()) {
                Ok(v) => v,
                Err(_) => return Err(500),
            };

        if similar_users.len() > 4 {
            break;
        }
    }

    let users_info = match user::recommendation::user_info(&similar_users, &stats_model) {
        Ok(v) => v,
        Err(_) => return Err(500),
    };

    let res = match user::recommendation::extract(
        stats_model,
        user_list,
        &similar_users,
        &settings.banned_ids(),
    )
    .await
    {
        Ok(v) => v,
        Err(error) => return Err(error),
    };

    let mut banned_users = res.banned_users;
    banned_users.append(&mut settings.banned_users());

    let next_request = NextRequest {
        banned_ids: settings.banned_ids().to_owned(),
        banned_users: banned_users,
        accuracy: final_accuracy,
        force_list_update: false,
    };

    Ok(DevResult {
        next_request,
        users: users_info,
        recommendations: res.recommendations,
    })
}
