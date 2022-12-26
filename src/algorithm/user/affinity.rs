use z_table;

use crate::algorithm::model::Model;
use crate::helper;

pub struct AffinityModel {
    pub min: Model<i16>,
    pub max: Model<i16>,
}

pub fn affinity_model(stats: Model<i16>) -> Result<AffinityModel, u16> {
    let normal_dist;
    match helper::get_normal_dist() {
        Ok(d) => normal_dist = d,
        Err(_) => return Err(500),
    }

    let tolerance = 250_000 / normal_dist.users_count();
    println!("tolerance: {tolerance}");

    let mut affinity = AffinityModel {
        min: Model::compare(),
        max: Model::compare(),
    };

    for x in 0..stats.len() {
        for y in 0..stats[x].len() {
            for z in 0..stats[x][y].len() {
                let tol = stat_tolerance(tolerance, x, y, z);
                if tol > 0.5 {
                    continue;
                }
                let deviation = calc_deviation(
                    stats[x][y][z],
                    normal_dist.mean(x, y, z),
                    normal_dist.std_dev(x, y, z),
                    tol,
                );
                affinity.min[x][y][z] = deviation[0];
                affinity.max[x][y][z] = deviation[1];
            }
        }
    }

    affinity.max[0][0][0] = 20_000;

    Ok(affinity)
}

fn calc_deviation(value: i16, mean: i16, std_dev: i16, tolerance: f32) -> [i16; 2] {
    let z_score = (value as f32 - mean as f32) / std_dev as f32;
    let usr = z_table::lookup(z_score);

    let mut bottom_dev = usr - tolerance;
    if bottom_dev < 0.0 {
        bottom_dev = 0.0;
    }
    if bottom_dev > 1.0 {
        bottom_dev = 1.0;
    }

    let mut top_dev = usr + tolerance;
    if top_dev < 0.0 {
        top_dev = 0.0;
    }
    if top_dev > 1.0 {
        top_dev = 1.0;
    }

    let bottom_z_score = z_table::reverse_lookup(bottom_dev);
    let top_z_score = z_table::reverse_lookup(top_dev);

    [
        mean + (bottom_z_score * std_dev as f32) as i16,
        mean + (top_z_score * std_dev as f32) as i16,
    ]
}

fn stat_tolerance(tolerance: i32, x: usize, _y: usize, z: usize) -> f32 {
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
