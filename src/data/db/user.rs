use super::connection;
use super::schema::users::dsl::*;
use crate::data::cast::common;
use diesel::prelude::*;

//  list

#[derive(Queryable)]
struct UserListRaw {
    list: serde_json::Value,
    updated_at: chrono::NaiveDateTime,
}

impl UserListRaw {
    fn deserialize(self) -> UserList {
        UserList {
            list: common::from_serde_value(self.list),
            updated_at: self.updated_at,
        }
    }
}

pub struct UserList {
    pub list: Vec<[i32; 4]>,
    pub updated_at: chrono::NaiveDateTime,
}

pub fn get_list(user: &String) -> Result<UserList, diesel::result::Error> {
    let user_list = users
        .select((list, updated_at))
        .filter(user_name.eq(&user))
        .first::<UserListRaw>(&mut connection::POOL.get().unwrap());

    match user_list {
        Ok(l) => Ok(l.deserialize()),
        Err(e) => Err(e),
    }
}

pub fn insert_list(user: &String, l: Vec<[i32; 4]>) {
    let inserted = diesel::insert_into(users)
        .values((
            user_name.eq(&user),
            list.eq(common::to_serde_value(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match inserted {
        Ok(_) => println!("[{}] list inserted", user),
        Err(_) => println!("failed insert for [{}] list", user),
    };
}

pub fn update_list(user: &String, l: Vec<[i32; 4]>) {
    let updated = diesel::update(users.find(&user))
        .set((
            list.eq(common::to_serde_value(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match updated {
        Ok(_) => println!("[{}] list updated", user),
        Err(_) => println!("failed update for [{}] list", user),
    };
}

//  model

pub fn get_model(user: &String) -> Result<Option<Vec<Vec<[i32; 9]>>>, diesel::result::Error> {
    let user_model = users
        .select(model)
        .filter(user_name.eq(&user))
        .first::<Option<serde_json::Value>>(&mut connection::POOL.get().unwrap());

    match user_model {
        Ok(m) => Ok(match m {
            Some(m) => Some(common::from_serde_value(m)),
            None => None,
        }),
        Err(e) => Err(e),
    }
}

pub fn set_model(user: &String, m: Vec<Vec<[i32; 9]>>) {
    let updated = diesel::update(users.find(&user))
        .set((
            model.eq(common::to_serde_value(&m)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());

    match updated {
        Ok(_) => println!("[{}] model updated", user),
        Err(_) => println!("failed update for [{}] model", user),
    };
}

//  user

pub fn delete(user: &String) -> u16 {
    let deleted = diesel::delete(users.find(user)).execute(&mut connection::POOL.get().unwrap());

    match deleted {
        Ok(_) => println!("user [{}] deleted", user),
        Err(_) => println!("failed delete for user [{}]", user),
    };

    403
}
