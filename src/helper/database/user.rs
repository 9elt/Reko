use time_elapsed;

use super::{AffinityUsers, DBAffinityUsers};

use crate::{utils::conversion, algorithm::model::Model};
use crate::utils::database::connection;
use crate::utils::database::schema::users::dsl::*;

use diesel::{prelude::*, sql_query};
use serde_json::json;
use chrono::Utc;

use crate::algorithm::user::affinity::AffinityModel;

////////////////////////////////////////////////////////////////////////////////
// affinity users
////////////////////////////////////////////////////////////////////////////////

pub fn get_affinity_users(affinity: AffinityModel, user: &String) -> Result<Vec<AffinityUsers>, diesel::result::Error> {
    let time = time_elapsed::start("db affinity users");

    let mut query = format!("
        SELECT user_name, list FROM users
        WHERE user_name != '{}'
    ", user);

    for x in 0..affinity.min.len() {
        for y in 0..affinity.min[x].len() {
            for z in 0..affinity.min[x][y].len() {
                if affinity.min[x][y][z] == 4095 {
                    continue;
                }
                query = format!(
                    "{} AND (model->{}->{}->{})::int >= {} AND (model->{}->{}->{})::int <= {}",
                    query,
                    x, y, z, affinity.min[x][y][z],
                    x, y, z, affinity.max[x][y][z]
                );
            }
        }
    }

    query = format!("{} ORDER BY (model->0->0->0)::int DESC", query);
    query = format!("{} LIMIT 8", query);

    let affinity_users = sql_query(query)
        .load::<DBAffinityUsers>(&mut connection::POOL.get().unwrap());

    time.end();

    match affinity_users {
        Ok(u) => Ok(u.iter().map(|e| e.deserialize()).collect()),
        Err(e) => Err(e)
    }
}

/*
*  USER LIST
*/

pub type UserList = Vec<Vec<i32>>; // to move somewhere else

pub struct DBUserList {
    list: UserList,
    updated_at: chrono::NaiveDateTime,
}

impl DBUserList {
    pub fn requires_update(&self) -> bool {
        let life = Utc::now().naive_local() - self.updated_at;
        life.num_days() > 2
    }
    pub fn list(self) -> UserList {
        self.list
    }
}

#[derive(Queryable)]
struct RawList {
    data: serde_json::Value,
    updated_at: chrono::NaiveDateTime,
}

impl RawList {
    fn deserialize(self) -> DBUserList {
        DBUserList {
            list: conversion::from_json(self.data),
            updated_at: self.updated_at,
        }
    }
}

pub fn get_list(user: &String) -> Result<DBUserList, diesel::result::Error> {
    let user_list = users
        .select((list, updated_at))
        .filter(user_name.eq(&user))
        .first::<RawList>(&mut connection::POOL.get().unwrap());

    match user_list {
        Ok(l) => Ok(l.deserialize()),
        Err(e) => Err(e),
    }
}

pub fn insert_list(user: &String, l: UserList) {
    let inserted = diesel::insert_into(users)
        .values((
            user_name.eq(&user),
            list.eq(json!(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match inserted {
        Ok(_) => println!("(db) inserted [{}] list", user),
        Err(_) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed inserting [{}] list", user),
    };
}

pub fn update_list(user: &String, l: UserList) {
    let updated = diesel::update(users.find(&user))
        .set((
            list.eq(json!(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match updated {
        Ok(_) => println!("(db) updated [{}] list", user),
        Err(_) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed updating [{}] list", user),
    };
}

/*
*  USER MODEL
*/

pub type UserModel = Vec<Vec<[i16; 9]>>; // to move somewhere else

pub struct DBUserModel {
    model: Option<UserModel>,
    updated_at: chrono::NaiveDateTime,
}

impl DBUserModel {
    pub fn requires_update(&self) -> bool {
        let life = Utc::now().naive_local() - self.updated_at;
        life.num_days() > 2
    }
    pub fn model(self) -> Option<UserModel> {
        self.model
    }
}

#[derive(Queryable)]
struct RawModel {
    data: Option<serde_json::Value>,
    updated_at: chrono::NaiveDateTime,
}

impl RawModel {
    fn deserialize(self) -> DBUserModel {
        DBUserModel {
            model: match self.data {
                Some(data) => conversion::from_json(data),
                None => None
            },
            updated_at: self.updated_at,
        }
    }
}

pub fn get_model(user: &String) -> Result<DBUserModel, diesel::result::Error> {
    let user_model = users
        .select((model, updated_at))
        .filter(user_name.eq(&user))
        .first::<RawModel>(&mut connection::POOL.get().unwrap());

    match user_model {
        Ok(m) => Ok(m.deserialize()),
        Err(e) => Err(e),
    }
}

pub fn set_model(user: &String, m: &Model<i16>) {
    let updated = diesel::update(users.find(&user))
        .set((
            model.eq(json!(m)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match updated {
        Ok(_) => println!("(db) updated [{}] model", user),
        Err(_) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed updating [{}] model", user),
    };
}

/*
*  USER
*/

pub fn delete(user: &String) {
    let deleted = diesel::delete(users.find(user)).execute(&mut connection::POOL.get().unwrap());
    match deleted {
        Ok(_) => println!("(db) deleted [{}] user", user),
        Err(_) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed deleting [{}] user", user),
    };
}

////////////////////////////////////////////////////////////////////////////////
// get all usernames
////////////////////////////////////////////////////////////////////////////////

pub fn get_all_usernames() -> Result<Vec<String>, diesel::result::Error> {
    let usernames = users
    .select(user_name)
    .load::<String>(&mut connection::POOL.get().unwrap());

    match usernames {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}