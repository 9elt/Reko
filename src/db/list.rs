use diesel::prelude::*;

use super::models::ListsDB;
use super::connection;

pub fn insert(new_list: ListsDB) {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let inserted = diesel::insert_into(lists)
        .values(&new_list)
        .execute(connection);

    println!("inserted {:?} new list", inserted);
}

pub fn update(new_list: ListsDB) {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let user_h = new_list.user_hash.to_owned();

    let updated = diesel::update(lists.find(user_h))
        .set(new_list)
        .get_result::<ListsDB>(connection)
        .unwrap();

    println!("user list update {:?}", updated);
}

pub fn delete(user_h: &String) -> u16 {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let deleted = diesel::delete(lists.find(user_h))
        .execute(connection)
        .unwrap();

    println!("{:?} user  deleted", deleted);
    403
}