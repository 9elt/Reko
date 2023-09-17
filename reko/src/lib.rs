mod util;

use clients::database::DBClient;
use clients::myanimelist::MALClient;

use structs::{DetailedListEntry, Hash, Recommendation, RekoError, RekoResult, SimilarUser, User};
use util::*;

const ENTRIES_TO_HASH: usize = 256;
const DAYS_BEFORE_UPDATE: u64 = 3;

#[derive(Clone)]
pub struct Reko {
    db: DBClient,
    mal: MALClient,
}

impl Reko {
    pub fn new() -> Self {
        Reko {
            db: DBClient::new(),
            mal: MALClient::new(),
        }
    }
    pub async fn get_user(
        &self,
        username: &String,
        force_update: bool,
        prevent_update: bool,
    ) -> RekoResult<User> {
        let username = username.to_lowercase();

        match self.db.get_user(username.to_owned()) {
            Some(mut user) => {
                if !prevent_update && force_update || user.updated_at < days_ago(DAYS_BEFORE_UPDATE)
                {
                    let list_update = self.mal.list(username, Some(user.updated_at)).await?;

                    if list_update.len() > 0 {
                        self.db.update_user_entries(&user, list_update);
                        user.hash =
                            self.user_hash(self.db.get_user_entries(&user, ENTRIES_TO_HASH))?;
                    }

                    user.updated_at = now();
                    self.db.update_user(&user);
                }

                Ok(user)
            }
            None => {
                let list = self.mal.list(username.to_owned(), None).await?;

                let mut ids = Vec::new();

                for entry in list.iter() {
                    if entry.watched {
                        ids.push(entry.id);
                        if ids.len() == ENTRIES_TO_HASH {
                            break;
                        }
                    }
                }

                let anime = self.db.get_anime(ids);
                let mut detailed_list = Vec::new();

                for a in anime {
                    let e = list.iter().find(|e| e.id == a.id).unwrap();
                    detailed_list.push(DetailedListEntry::new(a, e));
                }

                let hash = self.user_hash(detailed_list)?;

                let mut user = User {
                    id: -1,
                    username: username.to_owned(),
                    hash,
                    updated_at: now(),
                };

                if let Some(id) = self.db.insert_user(&user, list) {
                    user.id = id;
                    Ok(user)
                } else {
                    Err(RekoError::new(500, "Could not save user"))
                }
            }
        }
    }
    pub fn get_recommendations(&self, user: &User, page: i32) -> RekoResult<Vec<Recommendation>> {
        let res = self.db.get_recommendations(user, db_page(page, 20));
        if res.len() == 0 {
            Err(RekoError::new(404, "No recommendations found"))
        } else {
            Ok(res)
        }
    }
    pub fn get_similar_users(&self, user: &User, page: i32) -> RekoResult<Vec<SimilarUser>> {
        let res = self.db.get_similar_users(user, db_page(page, 100));
        if res.len() == 0 {
            Err(RekoError::new(404, "No similar users found"))
        } else {
            Ok(res)
        }
    }
    fn user_hash(&self, list: Vec<DetailedListEntry>) -> RekoResult<Hash> {
        let mut hash = Hasher::new();

        for entry in list {
            for stat in entry.stats {
                hash.push(stat, if entry.score > 0 { entry.score } else { 1 });
            }
        }

        let res = hash.finalize();

        if res > 0 {
            Ok(Hash::BigInt(res))
        } else {
            Err(RekoError::new(500, "Could not generate User hash"))
        }
    }
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
