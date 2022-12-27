use serde_json::json;

use crate::algorithm::analysis::NormalDist;
use crate::algorithm::model::Model;

use crate::utils::database::connection;
use crate::utils::database::schema::analysis;
use crate::utils::database::schema::analysis::dsl::*;
use diesel::prelude::*;

pub fn insert(normal_dist: NormalDist) {
    let res = diesel::insert_into(analysis)
        .values(RawNormalDist::serialize(normal_dist))
        .execute(&mut connection::POOL.get().unwrap());

    match res {
        Ok(num) => println!("(database) \x1b[34m\x1b[1mINFO!\x1b[0m inserted {} normal distribution", num),
        Err(err) => println!("(database) \x1b[31m\x1b[1mERROR!\x1b[0m failed inserting normal distribution (details: {:?})", err),
    };
}

pub fn get() -> Result<NormalDist, diesel::result::Error> {
    let normal_dist = analysis
        .order_by(users_count.desc())
        .first::<RawNormalDist>(&mut connection::POOL.get().unwrap());

    match normal_dist {
        Ok(val) => Ok(val.deserialize()),
        Err(err) => Err(err),
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = analysis)]
struct RawNormalDist {
    users_count: i32,
    mean: serde_json::Value,
    std_dev: serde_json::Value,
}

impl RawNormalDist {
    fn serialize(normal_dist: NormalDist) -> Self {
        Self {
            users_count: normal_dist.users_count(),
            mean: json!(&normal_dist.mean_model()),
            std_dev: json!(&normal_dist.std_dev_model()),
        }
    }

    fn deserialize(self) -> NormalDist {
        NormalDist::new(
            self.users_count, 
            Model::<i16>::from_json(self.mean), 
            Model::<i16>::from_json(self.std_dev)
        )
    }
}
