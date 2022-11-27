use diesel::prelude::*;

use super::connection;
use crate::data::structs::anime::AnimeDB;

pub fn get(ids: Vec<i32>) -> Result<Vec<AnimeDB>, diesel::result::Error> {
    use super::schema::anime::dsl::*;

    let mut result: Vec<AnimeDB> = vec![];

    let max_size = (ids.len() / 300) + 1;

    for t in 0..max_size {
        let mut query = anime.into_boxed();

        let curr = t * 300;

        query = query.filter(id.eq(ids[curr]));
        for i in curr..ids.len() {
            if i == curr + 300 {
                break;
            };
            query = query.or_filter(id.eq(ids[i]));
        }

        let res: Result<Vec<AnimeDB>, diesel::result::Error> =
            query.load::<AnimeDB>(&mut connection::POOL.get().unwrap());

        match res {
            Ok(mut r) => result.append(&mut r),
            Err(e) => return Err(e),
        };
    }

    Ok(result)
}

pub fn insert(entries: Vec<AnimeDB>) -> Vec<AnimeDB> {
    use super::schema::anime::dsl::*;

    let inserted = diesel::insert_into(anime)
        .values(&entries)
        .execute(&mut connection::POOL.get().unwrap());

    println!("{:?} new anime entries were inserted", inserted);
    entries
}
