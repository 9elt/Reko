use diesel::prelude::*;
use std::vec;

use crate::api;
use crate::db;
//use crate::helper::models::AnimeDetails;
use db::models::ListsDB;

use super::models::FullList;
use super::models::List;
use super::models::ListEntry;

use super::cast::to_json_type;

pub async fn get_detailed(s_user: String, reload: bool) -> Result<Vec<FullList>, u16> {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut db::connection::establish();

    let mut user_list: Vec<ListEntry> = vec![];

    let db_check = match lists
        .filter(user_hash.eq(&s_user))
        .first::<ListsDB>(connection)
    {
        Ok(res) => Some(res),
        Err(_) => None,
    };

    let missing = match db_check {
        Some(_) => false,
        None => true,
    };

    let update_required = match db_check {
        Some(l) => {
            let dur = chrono::Utc::now().naive_local() - l.updated_at;

            let res = List::from_db(l);
            user_list = res.list;

            dur.num_days() > 3 || reload
        }
        None => true,
    };

    if update_required {
        let api_list = api::list::get(&s_user).await;
        println!("requested new list");

        let mut _tmp: Vec<ListEntry> = vec![];

        match api_list {
            Ok(l) => {
                _tmp = l;
            }
            Err(e) => {
                if e == 403 && missing == false {
                    return Err(db::list::delete(&s_user));
                } else {
                    return Err(e);
                }
            }
        }

        match missing {
            true => db::list::insert(ListsDB {
                user_hash: s_user,
                list: to_json_type::<Vec<ListEntry>>(&_tmp),
                updated_at: chrono::Utc::now().naive_local(),
            }),
            false => db::list::update(ListsDB {
                user_hash: s_user,
                list: to_json_type::<Vec<ListEntry>>(&_tmp),
                updated_at: chrono::Utc::now().naive_local(),
            }),
        }

        user_list = _tmp;
    };

    let mut anime_ids = vec![];
    for e in user_list.iter() {
        anime_ids.push(e.id);
    }

    let anime_info = super::anime::get(anime_ids).await;

    //ugly... way too ugly
    let mut full_list: Vec<FullList> = vec![];
    for i in 0..user_list.len() {
        let id = user_list[i].id;
        for j in 0..anime_info.len() {
            if anime_info[j].id == id {
                full_list.push(FullList {
                    entry: user_list[i].clone(),
                    details: anime_info[j].clone(),
                })
            };
        }
    }

    Ok(full_list)
}
