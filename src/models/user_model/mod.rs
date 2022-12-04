pub mod empty;
mod avg;
mod conversion;
mod gen;

use crate::helper;
type BaseModel = Vec<Vec<[i32; 9]>>;

pub async fn get_user_model(user: &String, reload: bool) -> Result<[BaseModel; 2], u16> {
    let mut base_model = vec![];
    let mut _update_required = false;

    let check_db = helper::get_user_model(&user);
    match check_db {
        Ok(model) => {
            _update_required = model.requires_update();
            match model.model() {
                Some(m) => {
                    base_model = m;
                }
                None => _update_required = true,
            }
        }
        Err(_) => _update_required = true,
    }

    if _update_required || reload {
        base_model = match gen::generate_base_model(user.to_owned(), reload).await {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
    }

    let avg_model = avg::model();

    let mut averaged: Vec<Vec<[i32; 9]>> = empty::model();

    for x in 0..avg_model.len() {
        for y in 0..avg_model[x].len() {
            for z in 0..avg_model[x][y].len() {
                let entry = base_model[x][y][z] - avg_model[x][y][z];
                averaged[x][y][z] = match avg_model[x][y][z] {
                    0 => 0,
                    _ => entry * 1000 / avg_model[x][y][z],
                };
            }
        }
    }

    Ok([base_model, averaged])
}
