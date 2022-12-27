pub mod analysis;
pub mod anime;
pub mod user;

use super::AffinityUsers;

use crate::utils::conversion;
use diesel::{prelude::*, sql_types::{VarChar, Jsonb},};

#[derive(QueryableByName, Clone)]
pub struct DBAffinityUsers {
    #[diesel(sql_type = VarChar)]
    user_name: String,
    #[diesel(sql_type = Jsonb)]
    list: serde_json::Value,
}

impl DBAffinityUsers {
    pub fn deserialize(&self) -> AffinityUsers {
        AffinityUsers { 
            user_name: self.user_name.to_owned(),
            list: conversion::from_serde_value(self.list.to_owned()),
        }
    }
}

