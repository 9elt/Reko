use super::UserModel;

/// # User standard deviation model
/// takes a user stats model and returns its standard deviation model
pub fn std_dev_model(base_model: &UserModel) -> UserModel {
    let mut avg_model = UserModel::average();
    for x in 0..avg_model.len() {
        for y in 0..avg_model[x].len() {
            for z in 0..avg_model[x][y].len() {
                let v = &base_model[x][y][z];
                let a = &avg_model[x][y][z];
                let interpolation = match v + a {
                    -25 => 26,
                    _ => 25
                };
                avg_model[x][y][z] = ((v - a) * 100) / (v + a + interpolation);
            }
        }
    }
    avg_model[0][6] = [0; 9];
    avg_model
}