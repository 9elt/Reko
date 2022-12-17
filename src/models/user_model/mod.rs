mod avg;
mod conversion;
pub mod empty;
mod gen;
mod model_struct;
mod generation;

use crate::helper;
type BaseModel = Vec<Vec<[i32; 9]>>;

pub async fn get_user_model(user: &String, reload: bool) -> Result<[BaseModel; 2], u16> {
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
                let v = &base_model[x][y][z];
                let a = &avg_model[x][y][z];
                let interpolation = match v + a {
                    -25 => 26,
                    _ => 25
                };
                averaged[x][y][z] = ((v - a) * 100) / (v + a + interpolation);
            }
        }
    }

    Ok([base_model, averaged])
}
