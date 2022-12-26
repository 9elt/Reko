use std::intrinsics::assert_uninit_valid;

use z_table;

use crate::algorithm::model::Model;
use crate::helper;

pub struct AffinityModel {
    greater_than: Model<i16>,
    lower_than: Model<i16>,
}

pub fn affinity_model(stats: Model<i16>) {
    let normal_dist;
    match helper::get_normal_dist() {
        Ok(d) => normal_dist = d,
        Err(_) => return ()
    }

    let tolerance = 500_000 / normal_dist.users_count();

    let mut affinity = AffinityModel {
        greater_than: Model::compare(),
        lower_than: Model::compare(),
    };

    for x in 0..stats.len() {
        for y in 0..stats[x].len() {
            for z in 0..stats[x][y].len() {
                let deviation = calc_deviation(
                    stats[x][y][z],
                    normal_dist.mean(x, y, z),
                    normal_dist.std_dev(x, y, z),
                    stat_tolerance(tolerance, x, y, z)
                );
                affinity.greater_than[x][y][z] = stats[x][y][z] - deviation[0];
                affinity.lower_than[x][y][z] = stats[x][y][z] + deviation[1];
            }
        }
    }
}

fn calc_deviation(value: i16, mean: i16, std_dev: i16, tolerance: f32) -> [i16; 2] {
    let z_score = (value as f32 - mean as f32) / std_dev as f32;
    let usr = z_table::lookup_with(z_score, mean, std_dev);

    let bottom_z_score = z_table::reverse_lookup(usr - tolerance);
    let top_z_score = z_table::reverse_lookup(usr + tolerance);

    let gte = mean + (bottom_z_score * std_dev as f32) as i16;
    let lte = mean + (top_z_score * std_dev as f32) as i16;

    [gte, lte]
}

fn stat_tolerance(tolerance: i32, x: usize, y: usize, z: usize) -> f32 {
    tolerance as f32 / 100
}