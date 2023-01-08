pub mod analysis;
pub mod anime;
pub mod user;

use super::AffinityUsers;

use crate::{utils::conversion, algorithm::model::Model};
use diesel::{prelude::*, sql_types::{VarChar, Jsonb},};
use chrono::Utc;

////////////////////////////////////////////////////////////////////////////////
// affinity users structs
////////////////////////////////////////////////////////////////////////////////

#[derive(QueryableByName, Clone)]
pub struct DBAffinityUsers {
    #[diesel(sql_type = VarChar)]
    user_name: String,
    #[diesel(sql_type = Jsonb)]
    list: serde_json::Value,
    #[diesel(sql_type = Jsonb)]
    model: serde_json::Value,
}

impl DBAffinityUsers {
    pub fn deserialize(&self) -> AffinityUsers {
        AffinityUsers { 
            user_name: self.user_name.to_owned(),
            list: conversion::from_json(self.list.to_owned()),
            model: Model::<i16>::from_json(self.model.to_owned())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// user list structs
////////////////////////////////////////////////////////////////////////////////

pub type UserList = Vec<Vec<i32>>;

pub struct DBUserList {
    list: UserList,
    updated_at: chrono::NaiveDateTime,
}

impl DBUserList {
    pub fn requires_update(&self) -> bool {
        (Utc::now().naive_local() - self.updated_at).num_days() > 0
    }
    pub fn list(self) -> UserList {
        self.list
    }
}

#[derive(Queryable)]
pub struct RawListData {
    list: serde_json::Value,
    updated_at: chrono::NaiveDateTime,
}

impl RawListData {
    pub fn deserialize(self) -> DBUserList {
        DBUserList {
            list: conversion::from_json(self.list),
            updated_at: self.updated_at,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// user model structs
////////////////////////////////////////////////////////////////////////////////

pub type UserModel = Vec<Vec<[i16; 9]>>;

pub struct DBUserModel {
    model: Option<UserModel>,
    updated_at: chrono::NaiveDateTime,
}

impl DBUserModel {
    pub fn requires_update(&self) -> bool {
        (Utc::now().naive_local() - self.updated_at).num_days() > 0
    }

    pub fn model(self) -> Option<UserModel> {
        self.model
    }
}

#[derive(Queryable)]
struct RawModelData {
    model: Option<serde_json::Value>,
    updated_at: chrono::NaiveDateTime,
}

impl RawModelData {
    fn deserialize(self) -> DBUserModel {
        DBUserModel {
            model: match self.model {
                Some(data) => conversion::from_json(data),
                None => None,
            },
            updated_at: self.updated_at,
        }
    }
}
