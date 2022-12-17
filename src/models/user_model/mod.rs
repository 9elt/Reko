mod conversion;
pub mod init;
mod generation;

use crate::helper;
type UserModel = Vec<Vec<[i32; 9]>>;

pub async fn get_user_model(user: &String, reload: bool) -> Result<[UserModel; 2], u16> {
    let mut base_model = vec![];

    let mut update_required: bool;
    let check_db = helper::get_user_model(&user);
    match check_db {
        Ok(model) => {
            update_required = model.requires_update();
            match model.model() {
                Some(m) => {
                    base_model = m;
                }
                None => update_required = true,
            }
        }
        Err(_) => update_required = true,
    }

    if update_required || reload {
        base_model = match generation::base_model(user.to_owned(), reload).await {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }

    let std_dev_model: UserModel = generation::std_dev_model(&base_model);

    Ok([base_model, std_dev_model])
}


