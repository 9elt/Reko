use std::collections::HashSet;
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;

use crate::utils::benchmark;

use super::structs::anime::{AnimeDB, AnimeDetails};
use super::structs::list::{DetailedListEntry, ListEntry};
use chrono::Utc;

use super::db::anime;
use super::db::user;
use super::mal::fun::{get_mal_anime, get_mal_list};

/** returns a vector of animes details. If some are **missing from
the database**, the missing ones will be **requested to the
mal api**, if errors occur during any request, the function will
return the animes it has managed to get successfully */
pub async fn get_anime_details(ids: Vec<i32>) -> Vec<AnimeDetails> {
    let mut db_result: Vec<AnimeDB> = match anime::get(ids.to_owned()) {
        Ok(r) => r,
        Err(_) => vec![],
    };

    let mut complete_response: Vec<AnimeDetails> = vec![];

    let r_missing = (ids.len() - db_result.len()) > 0;

    if r_missing {
        print!("missing {} anime\n", ids.len() - db_result.len());

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
            let missing_hash_set: HashSet<_> =
                ids_hash_set.difference(&db_hash_set).cloned().collect();
            missing = Vec::from_iter(missing_hash_set);
        };

        let mut to_insert: Vec<AnimeDB> = vec![];

        let mut dbg_count: usize = 0;
        for m in missing.iter() {
            dbg_count += 1;
            match get_mal_anime(m).await {
                Ok(a) => {
                    println!("{} anime OK", dbg_count);
                    to_insert.push(a.clone());
                    db_result.push(a);
                }
                Err(e) => {
                    println!("{} anime ERROR", dbg_count);
                    println!("{e}");
                    continue;
                }
            };
            thread::sleep(Duration::from_millis(300));
        }

        if to_insert.len() > 0 {
            anime::insert(to_insert);
        };
    };

    for r in db_result {
        complete_response.push(AnimeDetails::from_db(r));
    }
    complete_response
}

pub async fn get_detailed_list(u: &String, reload: bool) -> Result<Vec<DetailedListEntry>, u16> {
    let mut benchmark = benchmark::Time::start("detailed list");

    //let mut base_list: Box<[Box<[i32; 4]>] = vec![];
    let mut base_list: Vec<Vec<i32>> = vec![];

    let database_list = user::get_list(&u);

    benchmark.millis(format!("[{}] database check", u));

    let list_is_missing: bool = match &database_list {
        Ok(_) => false,
        Err(_) => true,
    };

    let update_required = match &list_is_missing {
        false => match database_list {
            Ok(l) => {
                let is_empty: bool = l.list.len() == 0;
                let list_life = Utc::now().naive_local() - l.updated_at;
                base_list = l.list;
                list_life.num_days() > 2 || reload || is_empty
            }
            Err(_) => true,
        },
        true => true,
    };

    if update_required {
        let api_list = get_mal_list(&u).await;
        benchmark.millis(format!("requested [{}] list", u));

        let tmp: Vec<Vec<i32>>;

        match api_list {
            Ok(l) => {
                tmp = l;
            }
            Err(e) => {
                if e == 403 && list_is_missing == false {
                    return Err(user::delete(&u));
                } else {
                    return Err(e);
                }
            }
        }

        match list_is_missing {
            true => user::insert_list(&u, tmp.to_owned()),
            false => user::update_list(&u, tmp.to_owned()),
        }

        base_list = tmp;
    };

    let anime_ids: Vec<i32> = base_list.iter().map(|e| e[0]).collect();
    let mut anime_info = get_anime_details(anime_ids).await;

    benchmark.millis(format!("[{}] anime details", u));

    anime_info.sort_unstable_by(|x, y| y.id.cmp(&x.id));

    base_list.sort_unstable_by(|x, y| y[0].cmp(&x[0]));

    let mut full: Vec<DetailedListEntry> = vec![];

    let mut align: usize = 0;

    for i in 0..base_list.len() {
        if base_list[i][0] == anime_info[i - align].id {
            full.push(DetailedListEntry {
                entry: ListEntry {
                    id: base_list[i][0],
                    status: base_list[i][1],
                    score: base_list[i][2],
                    episodes_watched: base_list[i][0],
                },
                details: anime_info[i].to_owned(),
            });
        } else {
            align += 1;
        }
    }

    benchmark.millis(format!("[{}] merge lists", u));
    Ok(full)
}
