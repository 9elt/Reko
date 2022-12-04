use crate::utils::conversion::common;
use crate::utils::database::connection;
use crate::utils::database::schema::users::dsl::*;
use diesel::prelude::*;
use chrono::Utc;

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
            list: common::from_serde_value(self.data),
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
            list.eq(common::to_serde_value(&l)),
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
            list.eq(common::to_serde_value(&l)),
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

pub type UserModel = Vec<Vec<[i32; 9]>>; // to move somewhere else

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
                Some(data) => common::from_serde_value(data),
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

pub fn set_model(user: &String, m: UserModel) {
    let updated = diesel::update(users.find(&user))
        .set((
            model.eq(common::to_serde_value(&m)),
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