pub mod anime;
pub mod user;

use diesel::{prelude::*, sql_types::VarChar};

#[derive(QueryableByName)]
pub struct DBUserNames {
    #[diesel(sql_type = VarChar)]
    user_name: String
}