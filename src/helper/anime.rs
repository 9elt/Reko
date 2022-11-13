use crate::api;

use crate::db;
use crate::helper::models::AnimeDetails;
use db::models::AnimeDB;

use diesel::prelude::*;

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

    let mut db_result = query
        .load::<AnimeDB>(connection)
        .expect("failed to load anime details");

    let mut complete_response: Vec<AnimeDetails> = vec![];
    let r_missing = (ids.len() - db_result.len()) > 0;

    if r_missing {
        let r_empty: bool = db_result.len() == 0;

        let mut missing: Vec<i32> = vec![];

        if r_empty {
            missing = ids;
        } else {
            let mut align: usize = 0;
            for i in 0..ids.len() {
                if (i - align) < db_result.len() {
                    if ids[i] != db_result[i - align].id {
                        align = align + 1;
                        missing.push(ids[i].to_owned());
                    };
                } else {
                    missing.push(ids[i].to_owned());
                };
            }
        };

        let mut to_insert: Vec<AnimeDB> = vec![];

        for m in missing.iter() {
            println!("requested new anime");
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
