use crate::helper;

use crate::algorithm::user;
// use crate::algorithm::fucker::user::UserModel;
use crate::algorithm::model::Model;

pub async fn get_user_model(user: &String, reload: bool) -> Result<Model<i16>, u16> {
    let mut stats_model = Model::<i16>::empty();
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
        stats_model = match user::stats::stats_model(user.to_owned(), reload, false).await {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }

    Ok(stats_model)
}
