use crate::algorithm::analysis::NormalDist;
use crate::algorithm::model::Model;
use crate::utils::conversion::common;

use crate::utils::database::connection;
use crate::utils::database::schema::analysis;
use crate::utils::database::schema::analysis::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = analysis)]
struct RawNormalDist {
    users_count: i32,
    mean: serde_json::Value,
    std_dev: serde_json::Value,
}

impl RawNormalDist {
    fn from(normal_dist: NormalDist) -> Self {
        Self {
            users_count: normal_dist.users_count(),
            mean: common::to_serde_value(&normal_dist.mean().to_vec()),
            std_dev: common::to_serde_value(&normal_dist.std_dev().to_vec()),
        }
    }

    fn deserialize(self) -> NormalDist {
        NormalDist::new(
            self.users_count, 
            Model::<i16>::from_vec(common::from_serde_value(self.mean)), 
            Model::<i16>::from_vec(common::from_serde_value(self.std_dev))
        )
    }
}

pub fn insert(normal_dist: NormalDist) {
    let inserted = diesel::insert_into(analysis)
        .values(RawNormalDist::from(normal_dist))
        .execute(&mut connection::POOL.get().unwrap());

    match inserted {
        Ok(n) => println!("(db) inserted {} normal distribution", n),
        Err(e) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed inserting normal distribution (details: {:?})", e),
    };
}

pub fn get() -> Result<NormalDist, diesel::result::Error> {
    let normal_dist = analysis
        .order_by(users_count.desc())
        .first::<RawNormalDist>(&mut connection::POOL.get().unwrap());

    match normal_dist {
        Ok(d) => Ok(d.deserialize()),
        Err(e) => Err(e),
    }
}