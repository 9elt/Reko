use std::vec;

use crate::api;

use crate::db;
use crate::helper::models::List;
use crate::helper::models::ListEntry;
use db::models::ListsDB;

use diesel::prelude::*;

pub async fn get(s_user: String, reload: bool) -> Result<Vec<ListEntry>, u16> {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut db::connection::establish();

    let mut complete_response: Vec<ListEntry> = vec![];

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
            complete_response = match res.list {
                Some(l) => l,
                None => return Err(1001),
            };

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
                    return Err(delete(&s_user));
                } else {
                    return Err(e);
                }
            }
        }

        match missing {
            true => insert(ListsDB {
                user_hash: s_user,
                list: Some(list_to_json(&_tmp)),
                updated_at: chrono::Utc::now().naive_local(),
            }),
            false => update(ListsDB {
                user_hash: s_user,
                list: Some(list_to_json(&_tmp)),
                updated_at: chrono::Utc::now().naive_local(),
            }),
        }

        complete_response = _tmp;
    };

    Ok(complete_response)
}

fn insert(new_list: ListsDB) {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut db::connection::establish();

    let inserted = diesel::insert_into(lists)
        .values(&new_list)
        .execute(connection);

    println!("inserted {:?} new list", inserted);
}

fn update(new_list: ListsDB) {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut db::connection::establish();

    let user_h = new_list.user_hash.to_owned();

    let updated = diesel::update(lists.find(user_h))
        .set(new_list)
        .get_result::<ListsDB>(connection)
        .unwrap();

    println!("user list update {:?}", updated);
}

fn delete(user_h: &String) -> u16 {
    use crate::db::schema::lists::dsl::*;
    let connection = &mut db::connection::establish();

    let deleted = diesel::delete(lists.find(user_h))
        .execute(connection)
        .unwrap();

    println!("{:?} user  deleted", deleted);
    403
}

fn list_to_json(list: &Vec<ListEntry>) -> serde_json::Value {
    let j = serde_json::to_string(&list).unwrap();
    serde_json::from_str(&j).unwrap()
}
