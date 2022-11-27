use diesel::prelude::*;

use crate::data::structs::anime::AnimeDB;
use super::connection;

pub fn get(ids: Vec<i32>) -> Result<Vec<AnimeDB>, diesel::result::Error> {
    use super::schema::anime::dsl::*;

    let mut query = anime.into_boxed();

    query = query.filter(id.eq(ids[0]));
    for i in 1..ids.len() {
        query = query.or_filter(id.eq(ids[i]));
    }

    query.load::<AnimeDB>(&mut connection::POOL.get().unwrap())
}

pub fn insert(entries: Vec<AnimeDB>) -> Vec<AnimeDB> {
    use super::schema::anime::dsl::*;

    let inserted = diesel::insert_into(anime)
        .values(&entries)
        .execute(&mut connection::POOL.get().unwrap());

    println!("{:?} new anime entries were inserted", inserted);
    entries
}