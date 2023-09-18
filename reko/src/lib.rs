mod util;

use clients::database::DBClient;
use clients::myanimelist::MALClient;
use structs::{
    CompareResponseWrapper, DetailedListEntry, Hash, RecommendationResponseWrapper, RekoError,
    RekoResult, SimilarResponseWrapper, SimilarUser, User,
};
use util::*;

const ENTRIES_TO_HASH: usize = 256;
const DAYS_BEFORE_UPDATE: u64 = 3;
const MAX_PAGE_SIMILAR_USERS: u8 = 40;
const MAX_PAGE_RECOMMENDATIONS: u8 = 20;

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
                    let list_update = match self.mal.list(username, Some(user.updated_at)).await {
                        Ok(res) => res,
                        Err(e) => {
                            if e.code == 403 || e.code == 404 {
                                self.db.delete_user(&user);
                            }
                            return Err(e);
                        }
                    };

                    if list_update.len() > 0 {
                        self.db.update_user_entries(&user, list_update);

                        let list = self.db.get_user_entries(&user, ENTRIES_TO_HASH);
                        user.hash = self.user_hash(list)?;
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
    pub fn get_recommendations(
        &self,
        user: &User,
        page: i32,
    ) -> RekoResult<RecommendationResponseWrapper> {
        let res = self
            .db
            .get_recommendations(user, db_page(page, MAX_PAGE_RECOMMENDATIONS));
        if res.len() == 0 {
            Err(RekoError::new(404, "No recommendations found"))
        } else {
            Ok(RecommendationResponseWrapper::new(user, res))
        }
    }
    pub fn get_similar_users(&self, user: &User, page: i32) -> RekoResult<SimilarResponseWrapper> {
        let res = self
            .db
            .get_similar_users(user, db_page(page, MAX_PAGE_SIMILAR_USERS));
        if res.len() == 0 {
            Err(RekoError::new(404, "No similar users found"))
        } else {
            Ok(SimilarResponseWrapper::new(user, res))
        }
    }
    pub fn compare_users(&self, user: &User, other: &User) -> RekoResult<CompareResponseWrapper> {
        let hd_64 = (user.hash.to_u64() ^ other.hash.to_u64()).count_ones() as i32;
        let hd_16 = ((user.hash.to_u64() >> 48) ^ (other.hash.to_u64() >> 48)).count_ones() as i32;

        Ok(CompareResponseWrapper::new(
            user,
            SimilarUser {
                username: other.username.to_owned(),
                hash: other.hash.to_owned(),
                similarity: 100 - ((hd_64 + hd_16) * 100 / 80),
            },
        ))
    }
    pub async fn update_old_users(&self) {
        let users = self.db.get_old_users();

        for user in users {
            self.get_user(&user.username, true, false).await.ok();
            sleep(150);
        }
    }
    pub async fn update_airing_anime(&self) {
        let airing = self.db.get_airing_anime();

        for anime in airing {
            if let Ok(anime) = self.mal.anime(anime.id).await {
                self.db.update_anime(anime);
                sleep(350);
            }
        }
    }
    pub async fn request_missing_anime(&self) {
        let missing = self.db.get_missing_anime();

        for id in missing {
            if let Ok(anime) = self.mal.anime(id).await {
                self.db.insert_anime(vec![anime]);
                sleep(350);
            }
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
                hash += 1 << (63 - i);
            }
        }
        hash
    }
}
