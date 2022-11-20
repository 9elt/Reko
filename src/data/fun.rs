use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::Instant;
use std::{thread, time::Duration};

use super::structs::anime::{AnimeDB, AnimeDetails};
use super::structs::list::{DetailedListEntry, List, ListEntry, ListsDB};

use super::cast::generic::to_serde_value;

use super::db::fun::{
    delete_list, get_db_anime, get_db_list, insert_anime, insert_list, update_list,
};
use super::mal::fun::{get_mal_anime, get_mal_list};

/** returns a vector of animes details. If some are **missing from
the database**, the missing ones will be **requested to the
mal api**, if errors occur during any request, the function will
return the animes it has managed to get successfully */
pub async fn get_anime_details(ids: Vec<i32>) -> Vec<AnimeDetails> {
    let mut db_result: Vec<AnimeDB> = match get_db_anime(ids.to_owned()) {
        Ok(r) => r,
        Err(_) => vec![],
    };

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
            let missing_hash_set: HashSet<_> =
                ids_hash_set.difference(&db_hash_set).cloned().collect();
            missing = Vec::from_iter(missing_hash_set);
        };

        let mut to_insert: Vec<AnimeDB> = vec![];

        let mut dbg_count: usize = 0;
        for m in missing.iter() {
            dbg_count += 1;
            println!("requested new anime N {}", dbg_count);
            match get_mal_anime(m).await {
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
            insert_anime(to_insert);
        };
    };

    for r in db_result {
        complete_response.push(AnimeDetails::from_db(r));
    }
    complete_response
}

pub async fn get_detailed_list(
    s_user: String,
    reload: bool,
) -> Result<Vec<DetailedListEntry>, u16> {
    let start = Instant::now();

    let mut user_list: Vec<ListEntry> = vec![];

    let db_check = get_db_list(&s_user);
    println!("get_detailed_list > db checked in {} μs", start.elapsed().as_micros());

    let missing = match db_check {
        Ok(_) => false,
        Err(_) => true,
    };

    let update_required = match db_check {
        Ok(l) => {
            let dur = chrono::Utc::now().naive_local() - l.updated_at;

            let res = List::from_db(l);
            user_list = res.list;

            dur.num_days() > 3 || reload
        }
        Err(_) => true,
    };

    if update_required {
        let api_list = get_mal_list(&s_user).await;
        println!("requested new list");

        let tmp: Vec<ListEntry>;

        match api_list {
            Ok(l) => {
                tmp = l;
            }
            Err(e) => {
                if e == 403 && missing == false {
                    return Err(delete_list(&s_user));
                } else {
                    return Err(e);
                }
            }
        }

        match missing {
            true => insert_list(ListsDB {
                user_hash: s_user,
                list: to_serde_value::<Vec<ListEntry>>(&tmp),
                updated_at: chrono::Utc::now().naive_local(),
            }),
            false => update_list(ListsDB {
                user_hash: s_user,
                list: to_serde_value::<Vec<ListEntry>>(&tmp),
                updated_at: chrono::Utc::now().naive_local(),
            }),
        }

        user_list = tmp;
    };

    let mut anime_ids = vec![];
    for e in user_list.iter() {
        anime_ids.push(e.id);
    }

    let anime_info = get_anime_details(anime_ids).await;
    println!("get_detailed_list > anime details retrieved in {} μs", start.elapsed().as_micros());

    //ugly... way too ugly
    let mut full_list: Vec<DetailedListEntry> = vec![];
    for i in 0..user_list.len() {
        let id = user_list[i].id;
        for j in 0..anime_info.len() {
            if anime_info[j].id == id {
                full_list.push(DetailedListEntry {
                    entry: user_list[i].clone(),
                    details: anime_info[j].clone(),
                })
            };
        }
    }

    println!("get_detailed_list > detailed list done in {} μs", start.elapsed().as_micros());
    Ok(full_list)
}
