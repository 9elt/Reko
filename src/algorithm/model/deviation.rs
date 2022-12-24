use super::Model;

/// # User deviation model
/// Generates a deviation model from a statistics model
pub fn deviation_model(stats_model: &Model) -> Model {
    let mut avg_model = Model::average();
    for x in 0..avg_model.len() {
        for y in 0..avg_model[x].len() {
            for z in 0..avg_model[x][y].len() {
                let v = &stats_model[x][y][z];
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