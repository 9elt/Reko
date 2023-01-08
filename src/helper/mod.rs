mod database;
pub mod mal_api;

use time_elapsed;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::thread;
use std::time::Duration;

use crate::algorithm::analysis::NormalDist;
use crate::algorithm::{model::Model, user::affinity::AffinityModel};

use crate::helper::database::DBUserList;

use serde::{Deserialize, Serialize};

use self::database::anime::RawAnime;

#[derive(Serialize, Deserialize)]
pub struct AffinityUsers {
    pub user_name: String,
    pub list: Vec<Vec<i32>>,
    pub model: Model<i16>,
}

////////////////////////////////////////////////////////////////////////////////
// Analysis
////////////////////////////////////////////////////////////////////////////////

pub fn get_normal_dist() -> Result<NormalDist, diesel::result::Error> {
    database::analysis::get()
}

pub fn save_normal_dist(normal_dist: NormalDist) {
    database::analysis::insert(normal_dist)
}

////////////////////////////////////////////////////////////////////////////////
// User
////////////////////////////////////////////////////////////////////////////////

pub fn get_user_list(user: &String) -> Result<DBUserList, diesel::result::Error> {
    database::user::get_list(&user)
}

pub fn get_all_usernames() -> Result<Vec<String>, diesel::result::Error> {
    database::user::get_all_usernames()
}

pub fn get_old_usernames() -> Result<Vec<String>, diesel::result::Error> {
    database::user::get_old_usernames()
}

pub fn get_affinity_users(
    affinity_model: AffinityModel,
    user: &String,
    banned: &Vec<String>
) -> Result<Vec<AffinityUsers>, diesel::result::Error> {
    database::user::get_affinity_users(affinity_model, user, banned)
}

pub fn save_user_model(user: &String, model: &Model<i16>) {
    database::user::set_model(user, model);
}

pub fn get_user_model(user: &String) -> Result<database::DBUserModel, diesel::result::Error> {
    database::user::get_model(user)
}

////////////////////////////////////////////////////////////////////////////////
// Anime
////////////////////////////////////////////////////////////////////////////////

pub fn get_airing_anime() -> Result<Vec<i32>, diesel::result::Error> {
    database::anime::get_airing_anime()
}

pub fn update_anime(anime: RawAnime) -> Result<(), ()> {
    database::anime::update(anime)
}

////////////////////////////////////////////////////////////////////////////////
// Custom
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListEntry {
    id: i32,
    status: i32,
    score: i32,
    episodes_watched: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedListEntry {
    id: i32,
    status: i32,
    score: i32,
    episodes_watched: i32,
    title: String,

    picture: Option<String>,
    airing_date: Option<chrono::NaiveDate>,
    mean: Option<i16>,
    airing_status: Option<i16>,
    genres: Option<Vec<Option<i16>>>,
    num_episodes: Option<i16>,
    rating: Option<i16>,
    related: Option<Vec<RelatedAnime>>,
}

impl DetailedListEntry {
    pub fn _id(&self) -> i32 {
        self.id
    }
    pub fn status(&self) -> i32 {
        self.status
    }
    pub fn score(&self) -> i32 {
        self.score
    }
    pub fn _episodes_watched(&self) -> i32 {
        self.episodes_watched
    }
    pub fn _title(&self) -> &String {
        &self.title
    }
    pub fn _picture(&self) -> &Option<String> {
        &self.picture
    }
    pub fn airing_date(&self) -> Option<chrono::NaiveDate> {
        self.airing_date
    }
    pub fn mean(&self) -> Option<i16> {
        self.mean
    }
    pub fn _airing_status(&self) -> Option<i16> {
        self.airing_status
    }
    pub fn genres(&self) -> &Option<Vec<Option<i16>>> {
        &self.genres
    }
    pub fn num_episodes(&self) -> Option<i16> {
        self.num_episodes
    }
    pub fn rating(&self) -> Option<i16> {
        self.rating
    }
    pub fn _related(&self) -> &Option<Vec<RelatedAnime>> {
        &self.related
    }
}

pub async fn get_detailed_list(
    u: &String,
    reload: bool,
    prevent_update: bool,
) -> Result<Vec<DetailedListEntry>, u16> {
    let mut time = time_elapsed::start("list");

    let mut base_list: Vec<Vec<i32>> = vec![];
    let database_list = database::user::get_list(&u);

    time.log(format!("[{}] database check", u)).timestamp();

    let mut list_is_missing: bool = false;
    let update_required: bool;

    match database_list {
        Ok(l) => {
            update_required = l.requires_update() || reload;
            base_list = l.list();
        }
        Err(_) => {
            list_is_missing = true;
            update_required = true;
        }
    }

    if list_is_missing && prevent_update {
        return Err(500);
    }

    if update_required && !prevent_update {
        let api_list = mal_api::list::get(&u).await;

        time.log(format!("requested [{}] list", u)).timestamp();

        let tmp: Vec<Vec<i32>>;

        match api_list {
            Ok(l) => {
                tmp = l.to_vec();
            }
            Err(e) => {
                if (e == 403 || e == 404) && !list_is_missing {
                    database::user::delete(&u);
                }
                return Err(e);
            }
        }

        match list_is_missing {
            true => database::user::insert_list(&u, tmp.to_owned()),
            false => database::user::update_list(&u, tmp.to_owned()),
        }

        base_list = tmp;
    }

    let anime_ids: Vec<i32> = base_list.iter().map(|e| e[0]).collect();
    let mut anime_info = get_anime_details(anime_ids).await;

    time.log(format!("[{}] anime details", u)).timestamp();

    anime_info.sort_unstable_by(|x, y| y.id.cmp(&x.id));
    base_list.sort_unstable_by(|x, y| y[0].cmp(&x[0]));

    let mut full: Vec<DetailedListEntry> = vec![];
    let mut align: usize = 0;
    for i in 0..base_list.len() {
        if base_list[i][0] == anime_info[i - align].id {
            full.push(DetailedListEntry {
                id: base_list[i][0],
                status: base_list[i][1],
                score: base_list[i][2],
                episodes_watched: base_list[i][0],
                title: anime_info[i].title.to_owned(),
                picture: anime_info[i].picture.to_owned(),
                airing_date: anime_info[i].airing_date,
                mean: anime_info[i].mean,
                airing_status: anime_info[i].airing_status,
                genres: anime_info[i].genres.to_owned(),
                num_episodes: anime_info[i].num_episodes,
                rating: anime_info[i].rating,
                related: anime_info[i].related.to_owned(),
            });
        } else {
            align += 1;
        }
    }

    time.log(format!("[{}] extend list", u));

    time.end();

    Ok(full)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelatedAnime {
    pub id: u32,
    pub relation: i16,
}

impl RelatedAnime {
    pub fn new(id: u32, relation: i16) -> Self {
        Self {
            id,
            relation,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnimeDetails {
    pub id: i32,
    pub title: String,
    pub picture: Option<String>,
    pub airing_date: Option<chrono::NaiveDate>,
    pub mean: Option<i16>,
    pub airing_status: Option<i16>,
    pub genres: Option<Vec<Option<i16>>>,
    pub num_episodes: Option<i16>,
    pub rating: Option<i16>,
    pub related: Option<Vec<RelatedAnime>>,
}

impl AnimeDetails {
    pub fn new(
        id: i32,
        title: String,
        picture: Option<String>,
        airing_date: Option<chrono::NaiveDate>,
        mean: Option<i16>,
        airing_status: Option<i16>,
        genres: Option<Vec<Option<i16>>>,
        num_episodes: Option<i16>,
        rating: Option<i16>,
        related: Option<Vec<RelatedAnime>>,
    ) -> Self {
        Self {
            id,
            title,
            picture,
            airing_date,
            mean,
            airing_status,
            genres,
            num_episodes,
            rating,
            related,
        }
    }
}

pub async fn get_anime_details(ids: Vec<i32>) -> Vec<AnimeDetails> {
    let mut result = match database::anime::get(&ids) {
        Ok(val) => val,
        Err(_) => vec![],
    };

    if ids.len() - result.len() > 0 {
        let missing: Vec<i32> = match result.len() == 0 {
            true => ids,
            false => {
                let database_ids: Vec<i32> = result.iter().map(|e| e.id).collect();
                let database_ids_hashset: HashSet<i32> = HashSet::from_iter(database_ids);
                let missing_hashset: HashSet<i32> = HashSet::from_iter(ids)
                    .difference(&database_ids_hashset).cloned().collect();

                Vec::from_iter(missing_hashset)
            }
        };

        let mut to_insert = vec![];

        for id in missing.iter() {
            match mal_api::anime::get(id).await {
                Ok(res) => {
                    to_insert.push(res.serialize());
                    result.push(res.deserialize());
                }
                Err(_) => continue
            };
            thread::sleep(Duration::from_millis(300));
        }

        if to_insert.len() > 0 {
            database::anime::insert(to_insert);
        };
    };

    result
}
