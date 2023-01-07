use axum::Json;
use hyper::StatusCode;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::models;

#[derive(Deserialize)]
pub struct RecommendationsSettings {
    user_name: String,
    banned_ids: Option<Vec<i32>>,
    banned_users: Option<Vec<String>>,
    accuracy: Option<i32>,
    force_list_update: Option<bool>,
}

impl RecommendationsSettings {
    pub fn user_name(&self) -> String {
        self.user_name.to_lowercase()
    }

    pub fn banned_ids(&self) -> Vec<i32> {
        match &self.banned_ids {
            Some(ids) => ids.to_owned(),
            None => vec![],
        }
    }

    pub fn banned_users(&self) -> Vec<String> {
        match &self.banned_users {
            Some(users) => users.iter().map(|u| u.to_lowercase()).collect(),
            None => vec![],
        }
    }

    pub fn accuracy(&self) -> i32 {
        match self.accuracy {
            Some(accuracy) => match accuracy > 100 || accuracy < 0 {
                true => 100,
                false => accuracy,
            },
            None => 100,
        }
    }

    pub fn force_update(&self) -> bool {
        match self.force_list_update {
            Some(force_update) => force_update,
            None => false,
        }
    }
}

pub async fn get_user_recommendations(
    Json(settings): Json<RecommendationsSettings>,
) -> Result<Json<Value>, StatusCode> {
    match models::recommendations::get_user_recommendations(&settings).await {
        Ok(users) => Ok(Json(json!(users))),
        Err(error) => Err(StatusCode::from_u16(error).unwrap()),
    }
}
