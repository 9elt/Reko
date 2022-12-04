use crate::helper::AnimeDetails;
use crate::utils::conversion::common;
use crate::utils::database::connection;
use crate::utils::database::schema::anime;
use crate::utils::database::schema::anime::dsl::*;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = anime)]
pub struct DBAnime {
    id: i32,
    title: String,
    picture: Option<String>,
    mean: Option<i16>,
    airing_date: Option<chrono::NaiveDate>,
    airing_status: Option<i16>,
    num_episodes: Option<i16>,
    rating: Option<i16>,
    genres: Option<Vec<Option<i16>>>,
    related: Option<serde_json::Value>,
}

pub fn get(ids: &Vec<i32>) -> Result<Vec<DBAnime>, diesel::result::Error> {
    let mut complete_result: Vec<DBAnime> = vec![];
    let query_max_size = (ids.len() / 300) + 1;

    // A single query may stack overflow
    // this is a quick fix
    for i in 0..query_max_size {
        let mut query = anime.into_boxed();
        let paging = i * 300;

        if paging == ids.len() {
            break;
        }

        query = query.filter(id.eq(ids[paging]));
        for i in paging..ids.len() {
            if i == paging + 300 {
                break;
            }
            query = query.or_filter(id.eq(ids[i]));
        }

        let result: Result<Vec<DBAnime>, diesel::result::Error> =
            query.load::<DBAnime>(&mut connection::POOL.get().unwrap());

        match result {
            Ok(mut r) => complete_result.append(&mut r),
            Err(e) => return Err(e),
        };
    }

    Ok(complete_result)
}

pub fn insert(entries: Vec<DBAnime>) {
    let inserted = diesel::insert_into(anime)
        .values(&entries)
        .execute(&mut connection::POOL.get().unwrap());

    match inserted {
        Ok(n) => println!("(db) inserted {} anime", n),
        Err(e) => println!("\x1b[31m(db) \x1b[1mERROR!\x1b[0m failed inserting anime (details: {:?})", e),
    };
}

impl DBAnime {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn to_anime_details(&self) -> AnimeDetails {
        AnimeDetails {
            id: self.id,
            title: self.title.to_owned(),
            picture: self.picture.to_owned(),
            airing_date: self.airing_date,
            mean: self.mean,
            airing_status: self.airing_status,
            genres: self.genres.to_owned(),
            num_episodes: self.num_episodes,
            rating: self.rating,
            related: match self.related.to_owned() {
                Some(r) => common::from_serde_value(r),
                None => None
            }
        }
    }

    pub fn new(
        n_id: i32,
        n_title: String,
        n_picture: Option<String>,
        n_mean: Option<i16>,
        n_airing_date: Option<chrono::NaiveDate>,
        n_airing_status: Option<i16>,
        n_num_episodes: Option<i16>,
        n_rating: Option<i16>,
        n_genres: Option<Vec<Option<i16>>>,
        n_related: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: n_id,
            title: n_title,
            picture: n_picture,
            mean: n_mean,
            airing_date: n_airing_date,
            airing_status: n_airing_status,
            num_episodes: n_num_episodes,
            rating: n_rating,
            genres: n_genres,
            related: n_related,
        }
    }
}