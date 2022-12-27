//! the original library is **[z-table](https://github.com/fabianboesiger/z-table/)**
//! made by **Fabian Bösiger**
//! 
//! The library wouldn't complile so I copied it here with some tweaks, such as,
//! removing the infinite loop, and adding non blocking value restriction for cumulative
//! distribution values

mod table;
use table::{TABLE, TABLE_LEN, MAX_Z_SCORE};

////////////////////////////////////////////////////////////////////////////////
// public functions
////////////////////////////////////////////////////////////////////////////////

pub fn cumulative_dist(z_score: f32) -> f32 {
    match z_score >= 0.0 {
        true => TABLE[index_from_z_score(z_score)],
        false => 1.0 - TABLE[index_from_z_score(- z_score)]
    }
}

pub fn z_score(cumulative_dist: f32) -> f32 {
    let cd = restrict(cumulative_dist, 0.0, 1.0);
    match cd >= 0.5 {
        true => z_score_from_index(index_from_cumulative_dist(cd)),
        false => - z_score_from_index(index_from_cumulative_dist(1.0 - cd))
    }
}

////////////////////////////////////////////////////////////////////////////////
// private functions
////////////////////////////////////////////////////////////////////////////////

fn index_from_z_score(z_score: f32) -> usize {
    let index = (z_score / MAX_Z_SCORE * TABLE_LEN as f32) as usize;
    match index < TABLE_LEN {
        true => index,
        false => TABLE_LEN - 1
    }
}

fn z_score_from_index(index: usize) -> f32 {
    index as f32 * MAX_Z_SCORE / TABLE_LEN as f32
}

fn index_from_cumulative_dist(cumulative_dist: f32) -> usize {
    let mut previous = 1.0;
    for index in 0..TABLE_LEN {
        let current = (cumulative_dist - TABLE[index]).abs();
        match previous < current {
            true => return index - 1,
            false => previous = current
        };
    }
    TABLE_LEN - 1
}

fn restrict<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}