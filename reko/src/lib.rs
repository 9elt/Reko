mod hash;

use clients::database::DBClient;
use clients::myanimelist::MALClient;
use hash::Hasher;
use structs::{
    DetailedListEntry, Hash, Pagination, Recommendation, RekoError,
    RekoResult, SimilarUser, User, UserRecommendation,
};
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
                if !prevent_update && (force_update || user.updated_at < days_ago(DAYS_BEFORE_UPDATE))
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

                if list.len() == 0 {
                    return Err(RekoError::new(422, "EmptyUserList", "User list is empty"));
                }

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
                    Err(RekoError::new(500, "FailedSave", "Could not save user"))
                }
            }
        }
    }
    pub fn get_recommendations(
        &self,
        user: &User,
        page: i32,
    ) -> RekoResult<(Vec<Recommendation>, Pagination)> {
        let (result, pagination) = self
            .db
            .get_recommendations(user, db_page(page, MAX_PAGE_RECOMMENDATIONS));

        if result.len() == 0 {
            Err(RekoError::new(404, "NoData", "No recommendations found"))
        } else {
            Ok((result, pagination))
        }
    }
    pub fn get_recommendations_from(
        &self,
        user: &User,
        other: &User,
        page: i32,
    ) -> RekoResult<(Vec<UserRecommendation>, Pagination)> {
        let (result, pagination) =
            self.db
                .get_recommendations_from(user, &other, db_page(page, MAX_PAGE_RECOMMENDATIONS));

        if result.len() == 0 {
            Err(RekoError::new(404, "NoData", "No recommendations found"))
        } else {
            Ok((result, pagination))
        }
    }
    pub fn get_similar_users(
        &self,
        user: &User,
        page: i32,
    ) -> RekoResult<(Vec<SimilarUser>, Pagination)> {
        let (result, pagination) = self
            .db
            .get_similar_users(user, db_page(page, MAX_PAGE_SIMILAR_USERS));
        if result.len() == 0 {
            Err(RekoError::new(404, "NoData", "No similar users found"))
        } else {
            Ok((result, pagination))
        }
    }
    pub fn compare_users(&self, user: &User, other: &User) -> SimilarUser {
        let user_hash = user.hash.to_u64();
        let other_hash = other.hash.to_u64();

        let full_hd = (user_hash ^ other_hash).count_ones();
        let part_hd = ((user_hash & HASH_MASK) ^ (other_hash & HASH_MASK)).count_ones();

        SimilarUser {
            username: other.username.to_owned(),
            hash: other.hash.to_owned(),
            similarity: similarity((full_hd + part_hd) as i32),
        }
    }
    pub async fn update_old_users<F: Fn(u32, u32)>(&self, progress: F) {
        let users = self.db.get_old_users();

        let tot = users.len() as u32;
        let mut curr: u32 = 1;

        for user in users {
            progress(curr, tot);
            self.get_user(&user.username, true, false).await.ok();
            curr += 1;
        }
    }
    pub async fn update_airing_anime<F: Fn(u32, u32)>(&self, progress: F) {
        let airing = self.db.get_airing_anime();

        let tot = airing.len() as u32;
        let mut curr: u32 = 1;

        for anime in airing {
            progress(curr, tot);
            if let Ok(anime) = self.mal.anime(anime.id).await {
                self.db.update_anime(anime);
            }
            curr += 1;
        }
    }
    pub async fn request_missing_anime<F: Fn(u32, u32)>(&self, progress: F) {
        let missing = self.db.get_missing_anime();

        let tot = missing.len() as u32;
        let mut curr: u32 = 1;

        for id in missing {
            progress(curr, tot);
            if let Ok(anime) = self.mal.anime(id).await {
                self.db.insert_anime(vec![anime]);
            }
            curr += 1;
        }
    }
    fn user_hash(&self, list: Vec<DetailedListEntry>) -> RekoResult<Hash> {
        let mut hash = Hasher::new();

        for entry in list {
            for stat in entry.stats {
                hash.push(stat, entry.score + 1);
            }
        }

        let res = hash.finalize();

        if res > 0 {
            Ok(Hash::BigInt(res))
        } else {
            Err(RekoError::new(
                500,
                "HashFailed",
                "Could not generate User hash",
            ))
        }
    }
}
