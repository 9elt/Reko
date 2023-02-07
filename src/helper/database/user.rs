use super::{AffinityUsers, DBAffinityUsers, DBUserList, RawListData, UserList, DBUserModel, RawModelData};

use crate::algorithm::model::Model;
use crate::utils::database::connection;
use crate::utils::database::schema::users::dsl::*;

use diesel::dsl::*;
use diesel::{prelude::*, sql_query};
use serde_json::json;

use crate::algorithm::user::affinity::AffinityModel;
use sql_lexer::sanitize_string;

////////////////////////////////////////////////////////////////////////////////
// affinity users
////////////////////////////////////////////////////////////////////////////////

pub fn get_affinity_users(
    affinity: AffinityModel,
    user: &String,
    banned: &Vec<String>,
) -> Result<Vec<AffinityUsers>, diesel::result::Error> {
    let mut query = format!(
        "SELECT user_name, list, model FROM users WHERE user_name != '{}'",
        sanitize_string(user.to_owned())
    );

    for banned_user in banned.iter() {
        query = format!("{} AND user_name != '{}'", query, sanitize_string(banned_user.to_owned()));
    }

    for x in 0..affinity.min.len() {
        for y in 0..affinity.min[x].len() {
            for z in 0..affinity.min[x][y].len() {
                if affinity.min[x][y][z] == 4095 {
                    continue;
                }
                query = format!(
                    "{} AND (model->{}->{}->{})::int >= {} AND (model->{}->{}->{})::int <= {}",
                    query, x, y, z, affinity.min[x][y][z], x, y, z, affinity.max[x][y][z]
                );
            }
        }
    }

    query = format!("{} ORDER BY (model->0->0->0)::int DESC LIMIT 8", query);

    let affinity_users =
        sql_query(query).load::<DBAffinityUsers>(&mut connection::POOL.get().unwrap());

    match affinity_users {
        Ok(u) => Ok(u.into_iter().map(|e| e.deserialize()).collect()),
        Err(e) => Err(e),
    }
}

////////////////////////////////////////////////////////////////////////////////
// user list
////////////////////////////////////////////////////////////////////////////////

pub fn get_list(user: &String) -> Result<DBUserList, diesel::result::Error> {
    let user_list = users
        .select((list, updated_at))
        .filter(user_name.eq(&user))
        .first::<RawListData>(&mut connection::POOL.get().unwrap());

    match user_list {
        Ok(l) => Ok(l.deserialize()),
        Err(e) => Err(e),
    }
}

pub fn insert_list(user: &String, l: UserList) {
    let _res = diesel::insert_into(users)
        .values((
            user_name.eq(&user),
            list.eq(json!(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());
}

pub fn update_list(user: &String, l: UserList) {
    let _res = diesel::update(users.find(&user))
        .set((
            list.eq(json!(&l)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());
}

////////////////////////////////////////////////////////////////////////////////
// user model
////////////////////////////////////////////////////////////////////////////////

pub fn get_model(user: &String) -> Result<DBUserModel, diesel::result::Error> {
    let user_model = users
        .select((model, updated_at))
        .filter(user_name.eq(&user))
        .first::<RawModelData>(&mut connection::POOL.get().unwrap());

    match user_model {
        Ok(m) => Ok(m.deserialize()),
        Err(e) => Err(e),
    }
}

pub fn set_model(user: &String, m: &Model<i16>) {
    let _res = diesel::update(users.find(&user))
        .set((
            model.eq(json!(m)),
            updated_at.eq(chrono::Utc::now().naive_local()),
        ))
        .execute(&mut connection::POOL.get().unwrap());
}

pub fn delete(user: &String) {
    let _res = diesel::delete(users.find(user)).execute(&mut connection::POOL.get().unwrap());
}

////////////////////////////////////////////////////////////////////////////////
// get usernames
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

pub fn get_old_usernames() -> Result<Vec<String>, diesel::result::Error> {
    let usernames = users
        .select(user_name)
        .filter(updated_at.lt(now - 5.days()))
        .limit(10)
        .load::<String>(&mut connection::POOL.get().unwrap());

    match usernames {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}
