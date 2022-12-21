use crate::helper;

use crate::algorithm::model;
use crate::algorithm::model::{UserModel, Model};

pub async fn get_user_model(user: &String, reload: bool) -> Result<[Model; 2], u16> {
    let mut base_model = UserModel::empty();

    let mut update_required: bool = false;

    if !reload {
        let check_db = helper::get_user_model(&user);
        match check_db {
            Ok(model) => {
                update_required = model.requires_update();
                match model.model() {
                    Some(m) => {
                        base_model = UserModel::from(m);
                    }
                    None => update_required = true,
                }
            }
            Err(_) => update_required = true,
        }
    }

    if update_required || reload {
        base_model = match model::stats::stats_model(user.to_owned(), reload).await {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }

    let std_dev_model: UserModel = model::average::std_dev_model(&base_model);

    Ok([base_model.to_vec(), std_dev_model.to_vec()])
}


