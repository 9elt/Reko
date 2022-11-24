use diesel::prelude::*;

use crate::data::structs::list::{ListsDB};
use crate::data::structs::anime::AnimeDB;
use super::connection;

//  anime

pub fn get_db_anime(ids: Vec<i32>) -> Result<Vec<AnimeDB>, diesel::result::Error> {
    use super::schema::anime::dsl::*;
    let connection = &mut connection::establish();

    let mut query = anime.into_boxed();

    query = query.filter(id.eq(ids[0]));
    for i in 1..ids.len() {
        query = query.or_filter(id.eq(ids[i]));
    }

    query.load::<AnimeDB>(connection)
}

pub fn insert_anime(entries: Vec<AnimeDB>) -> Vec<AnimeDB> {
    use super::schema::anime::dsl::*;
    let connection = &mut connection::establish();

    let inserted = diesel::insert_into(anime)
        .values(&entries)
        .execute(connection);

    println!("inserted {:?} new anime entries", inserted);
    entries
}

// lists

pub fn get_db_list(user: &String) -> Result<ListsDB, diesel::result::Error> {
    use super::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    lists
        .filter(user_hash.eq(&user))
        .first::<ListsDB>(connection)

}

pub fn insert_list(new_list: ListsDB) {
    use super::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let _inserted = diesel::insert_into(lists)
        .values(&new_list)
        .execute(connection);

    println!("inserted new list");
}

pub fn update_list(new_list: ListsDB) {
    use super::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let user_h = new_list.user_hash.to_owned();

    let _updated = diesel::update(lists.find(user_h))
        .set(new_list)
        .get_result::<ListsDB>(connection)
        .unwrap();

    println!("user list update");
}

pub fn delete_list(user_h: &String) -> u16 {
    use super::schema::lists::dsl::*;
    let connection = &mut connection::establish();

    let _deleted = diesel::delete(lists.find(user_h))
        .execute(connection)
        .unwrap();

    println!("user deleted");
    403
}