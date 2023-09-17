use chrono::{Days, NaiveDateTime, Utc};
use clients::database::DBClient;
use clients::myanimelist::MALClient;
use dotenvy::dotenv;
use structs::{DetailedListEntry, User};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mal = MALClient::new();
    let db = DBClient::new();

    // if true {
    //     for e in 0..60_000 {
    //         let anime = match mal.anime(e).await {
    //             Ok(anime) => anime,
    //             Err(error) => continue,
    //         };
    //         db.insert_anime(vec![anime]);
    //     }
    //     println!("anime inserted");
    // }

    if true {
        let username1 = String::from("KoreanLunatic").to_lowercase();
        let user1 = get_user(&username1, (false, false), (&db, &mal)).await;

        println!("name {} hash {:02x} ", user1.username, user1.hash);

        let sim = db.get_similar_users(&user1, 0);

        if sim.len() == 0 {
            panic!("No users :/")
        }

        for suser in sim {
            println!("name {} hash {:02x} ", suser.username, suser.hash);

            println!(
                "hamming distance {} | sim. {}%",
                suser.distance,
                100 - suser.distance * 100 / 64
            );
        }

        // db.get_rekos(user.id)
    }
}

const ENTRIES_FOR_HASH: usize = 256;
const DAYS_FOR_UPDATE: u64 = 3;

async fn get_user(
    username: &String,
    (force_update, prevent_update): (bool, bool),
    (db, mal): (&DBClient, &MALClient),
) -> User {
    let username = username.to_lowercase();

    match db.get_user(username.to_owned()) {
        Some(mut user) => {
            if !prevent_update && force_update || user.updated_at < days_ago(DAYS_FOR_UPDATE) {
                let list_update = match mal.list(username, Some(user.updated_at)).await {
                    Ok(list) => list,
                    Err(error) => panic!("{:#?}", error),
                };

                if list_update.len() > 0 {
                    db.update_user_entries(&user, list_update);
                    user.hash = calc_hash(db.get_user_entries(&user, ENTRIES_FOR_HASH));
                }

                user.updated_at = now();
                db.update_user(&user);
            }

            user
        }
        None => {
            let list = match mal.list(username.to_owned(), None).await {
                Ok(list) => list,
                Err(error) => panic!("{:#?}", error),
            };

            let mut ids = Vec::new();

            for entry in list.iter() {
                if entry.watched {
                    ids.push(entry.id);
                    if ids.len() == ENTRIES_FOR_HASH {
                        break;
                    }
                }
            }

            let anime = db.get_anime(ids);
            let mut detailed_list = Vec::new();

            for a in anime {
                let e = list.iter().find(|e| e.id == a.id).unwrap();
                detailed_list.push(DetailedListEntry::new(a, e));
            }

            let user = User {
                id: -1,
                username: username.to_owned(),
                hash: calc_hash(detailed_list),
                updated_at: now(),
            };

            db.insert_user(&user, list);

            user
        }
    }
}

fn calc_hash(detailed_list: Vec<DetailedListEntry>) -> u64 {
    let mut hash = Hasher::new();

    for entry in detailed_list {
        for stat in entry.stats {
            hash.push(stat, if entry.score > 0 { entry.score } else { 1 });
        }
    }

    hash.finalize()
}

fn days_ago(days: u64) -> NaiveDateTime {
    now().checked_sub_days(Days::new(days)).unwrap()
}

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

struct Hasher {
    data: [i32; 92],
}

impl Hasher {
    fn new() -> Self {
        Self { data: [0; 92] }
    }
    fn push(&mut self, stat: i32, value: i32) {
        self.data[stat as usize] += value;
    }
    fn finalize(&mut self) -> u64 {
        let mut hash: u64 = 0;
        for i in 0..64 {
            if self.data[i] > self.data[i + 1] {
                hash += 1 << i;
            }
        }
        hash
    }
}
