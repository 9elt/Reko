use crate::api;

use crate::db;
use crate::helper::models::AnimeDetails;
use db::models::AnimeDB;

use diesel::prelude::*;
use std::{thread, time::Duration};
use std::collections::HashSet;
use std::iter::FromIterator;

/** returns a vector of animes details. If some are **missing from
the database**, the missing ones will be **requested to the
mal api**, if errors occur during any request, the function will
return the animes it has managed to get successfully */
pub async fn get(ids: Vec<i32>) -> Vec<AnimeDetails> {
    use crate::db::schema::anime::dsl::*;
    let connection = &mut db::connection::establish();

    let mut query = anime.into_boxed();

    query = query.filter(id.eq(ids[0]));
    for i in 1..ids.len() {
        query = query.or_filter(id.eq(ids[i]));
    }

    let mut db_result: Vec<AnimeDB> = query
        .load::<AnimeDB>(connection)
        .expect("failed to load anime details");

    let mut complete_response: Vec<AnimeDetails> = vec![];
    let r_missing = (ids.len() - db_result.len()) > 0;

    if r_missing {
        let r_empty: bool = db_result.len() == 0;

        let missing: Vec<i32>;

        if r_empty {
            missing = ids.to_owned();
        } else {
            let mut db_ids: Vec<i32> = vec![];
            for db_entr in db_result.iter() {
                db_ids.push(db_entr.id);
            }
            let ids_hash_set: HashSet<i32> = HashSet::from_iter(ids);
            let db_hash_set: HashSet<i32> = HashSet::from_iter(db_ids);
            let missing_hash_set: HashSet<_> = ids_hash_set.difference(&db_hash_set).cloned().collect();
            missing = Vec::from_iter(missing_hash_set);
        };

        let mut to_insert: Vec<AnimeDB> = vec![];

        let mut dbg_count: usize = 0;
        for m in missing.iter() {
            dbg_count += 1;
            println!("requested new anime N {}", dbg_count);
            match api::anime::get(m).await {
                Ok(a) => {
                    to_insert.push(a.clone());
                    db_result.push(a);
                }
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            };
            thread::sleep(Duration::from_millis(300));
        }

        if to_insert.len() > 0 {
            insert(to_insert);
        };
    };

    for r in db_result {
        complete_response.push(AnimeDetails::from_db(r));
    }
    complete_response
}

fn insert(entries: Vec<AnimeDB>) -> Vec<AnimeDB> {
    use crate::db::schema::anime::dsl::*;
    let connection = &mut db::connection::establish();

    let inserted = diesel::insert_into(anime)
        .values(&entries)
        .execute(connection);

    println!("inserted {:?} new anime entries", inserted);
    entries
}
