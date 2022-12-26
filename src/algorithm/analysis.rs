use serde::{Serialize, Deserialize};

use super::model::Model;
use crate::helper;

#[derive(Serialize, Deserialize)]
pub struct NormalDist {
    users_count: i32,
    mean: Model<i16>,
    std_dev: Model<i16>,
}

impl NormalDist {
    pub fn new(users_count: i32, mean: Model<i16>, std_dev: Model<i16>) -> Self {
        Self { users_count, mean, std_dev, }
    }

    pub fn users_count(&self) -> i32 {
        self.users_count
    }

    pub fn mean_model(&self) -> &Model<i16> {
        &self.mean
    }

    pub fn std_dev_model(&self) -> &Model<i16> {
        &self.std_dev
    }

    pub fn mean(&self, x: usize, y: usize, z: usize) -> i16 {
        self.mean[x][y][z]
    }

    pub fn std_dev(&self, x: usize, y: usize, z: usize) -> i16 {
        self.std_dev[x][y][z]
    }
}

pub fn normal_distribution() -> Result<NormalDist, i16> {
    let usernames: Vec<String>;
    match helper::get_all_usernames() {
        Ok(res) => usernames = res,
        Err(_) => return Err(500),
    }

    let mut user_counter: i32 = 0;
    let users_count: i32 = match i32::try_from(usernames.len()) {
        Ok(n) => n,
        Err(_) => 0,
    };

    ////////////////////////////////////////////////////////////////////////////////
    // Sum all values
    ////////////////////////////////////////////////////////////////////////////////

    let mut sum = Model::<i32>::empty();
    for user in usernames.iter() {
        user_counter += 1;
        print!("\r\x1b[1m(values sum)\x1b[34m {}\x1b[0m/{}", user_counter, users_count);

        let user_stats: Model<i16>;

        match helper::get_user_model(user) {
            Ok(db_model) => match db_model.model() {
                Some(m) => user_stats = Model::<i16>::from_vec(m),
                None => continue,
            },
            Err(_) => continue,
        }

        for x in 0..sum.len() {
            for y in 0..sum[x].len() {
                for z in 0..sum[x][y].len() {
                    sum[x][y][z] += user_stats[x][y][z] as i32;
                }
            }
        }
    }

    println!("");
    user_counter = 0;

    ////////////////////////////////////////////////////////////////////////////////
    // Means calculation
    ////////////////////////////////////////////////////////////////////////////////

    let mut mean = Model::<i16>::empty();
    for x in 0..sum.len() {
        for y in 0..sum[x].len() {
            for z in 0..sum[x][y].len() {
                mean[x][y][z] += calc_mean(sum[x][y][z], &users_count);
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Sum all squared deviations
    ////////////////////////////////////////////////////////////////////////////////

    let mut dev_sum = Model::<i64>::empty();
    for user in usernames.iter() {
        user_counter += 1;
        print!("\r\x1b[1m(squared deviation sum)\x1b[34m {}\x1b[0m/{}", user_counter, users_count);

        let user_stats: Model<i16>;

        match helper::get_user_model(user) {
            Ok(db_model) => match db_model.model() {
                Some(m) => user_stats = Model::<i16>::from_vec(m),
                None => continue,
            },
            Err(_) => continue,
        }

        for x in 0..sum.len() {
            for y in 0..sum[x].len() {
                for z in 0..sum[x][y].len() {
                    dev_sum[x][y][z] += squared_dev(user_stats[x][y][z], &mean[x][y][z]);
                }
            }
        }
    }

    println!("");

    ////////////////////////////////////////////////////////////////////////////////
    // Standard deviations calculation
    ////////////////////////////////////////////////////////////////////////////////

    let mut std_dev = Model::<i16>::empty();
    for x in 0..sum.len() {
        for y in 0..sum[x].len() {
            for z in 0..sum[x][y].len() {
                std_dev[x][y][z] += calc_std_dev(dev_sum[x][y][z], &users_count);
            }
        }
    }

    Ok(NormalDist::new(users_count, mean, std_dev))
}

////////////////////////////////////////////////////////////////////////////////
// Math
////////////////////////////////////////////////////////////////////////////////

fn calc_mean(sum: i32, count: &i32) -> i16 {
    if count == &0 {
        return 0;
    }
    match i16::try_from(sum / count) {
        Ok(c) => c,
        Err(_) => 0,
    }
}

fn squared_dev(value: i16, mean: &i16) -> i64 {
    (value as i64 - mean.to_owned() as i64).pow(2)
}

fn calc_std_dev(dev_sum: i64, count: &i32) -> i16 {
    if count == &0 {
        return 0;
    }
    ((dev_sum / count.to_owned() as i64) as f32).sqrt() as i16
}
