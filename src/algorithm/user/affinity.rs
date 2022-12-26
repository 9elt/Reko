use crate::utils::z_table;

use crate::algorithm::model::Model;
use crate::helper;

pub struct AffinityModel {
    pub min: Model<i16>,
    pub max: Model<i16>,
}

pub fn affinity_model(stats: &Model<i16>, accuracy: i32) -> Result<AffinityModel, u16> {
    let normal_dist;
    match helper::get_normal_dist() {
        Ok(d) => normal_dist = d,
        Err(_) => return Err(500),
    }

    let costant_tolerance = (250_000 / normal_dist.users_count()) + (100 - accuracy);

    let mut affinity = AffinityModel {
        min: Model::compare(),
        max: Model::compare(),
    };

    for x in 0..stats.len() {
        for y in 0..stats[x].len() {
            for z in 0..stats[x][y].len() {
                let tolerance = stat_tolerance(costant_tolerance, x, z);
                if tolerance > 0.5 {
                    continue;
                }
                let range = deviation_range(
                    stats[x][y][z],
                    normal_dist.mean(x, y, z),
                    normal_dist.std_dev(x, y, z),
                    tolerance,
                );
                affinity.min[x][y][z] = range[0];
                affinity.max[x][y][z] = range[1];
            }
        }
    }

    affinity.max[0][0][0] = 20_000;

    Ok(affinity)
}

fn deviation_range(value: i16, mean: i16, std_dev: i16, tolerance: f32) -> [i16; 2] {
    let z_score = (value as f32 - mean as f32) / std_dev as f32;
    let cumulative_dist = z_table::lookup(z_score);

    let min_cd = unit_value(cumulative_dist - tolerance);
    let max_cd = unit_value(cumulative_dist + tolerance);

    let min_z_score = z_table::reverse_lookup(min_cd);
    let max_z_score = z_table::reverse_lookup(max_cd);

    [
        mean + (min_z_score * std_dev as f32) as i16,
        mean + (max_z_score * std_dev as f32) as i16,
    ]
}

fn stat_tolerance(tolerance: i32, x: usize, z: usize) -> f32 {
    let d: f32 = match z {
        0 => 1.0, // perc

        1 => 2.5, // mal mean score
        2 => 8.0, // score dev
        3 => 8.0, // scored perc

        _ => 8.0, // statuses
    };
    let c: f32 = match x {
        0 => 1.0 * d, // general

        1 => 3.0 * d, // airing decades
        2 => 3.0 * d, // ratings
        3 => 3.0 * d, // series length

        4 => 4.0 * d, // MAJOR genres
        5 => 8.0 * d, // minor genres

        6 => 4.0 * d, // MAJOR themes
        7 => 8.0 * d, // minor themes

        8 => 4.0 * d, // demographics

        _ => 10.0 * d, // none
    };
    (tolerance as f32) * c / 100.0
}

fn unit_value(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}