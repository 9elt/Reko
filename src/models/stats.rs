use crate::helper;

use crate::algorithm::model;
use crate::algorithm::model::{Model, user::UserModel};

pub async fn get_user_model(user: &String, reload: bool) -> Result<UserModel, u16> {
    let mut stats_model = Model::empty();

    let mut update_required: bool = false;

    if !reload {
        let check_db = helper::get_user_model(&user);
        match check_db {
            Ok(model) => {
                update_required = model.requires_update();
                match model.model() {
                    Some(m) => {
                        stats_model = Model::from_vec(m);
                    }
                    None => update_required = true,
                }
            }
            Err(_) => update_required = true,
        }
    }

    if update_required || reload {
        stats_model = match model::stats::stats_model(user.to_owned(), reload).await {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }

    let deviation_model: Model = model::deviation::deviation_model(&stats_model);

    Ok(UserModel::from(stats_model, deviation_model))
}
