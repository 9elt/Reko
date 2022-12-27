use crate::helper::AnimeDetails;
use crate::utils::conversion;
use crate::utils::database::connection;
use crate::utils::database::schema::anime;
use crate::utils::database::schema::anime::dsl::*;
use diesel::prelude::*;

const MAX_QUERY_SIZE: usize = 300;

pub fn get(ids: &Vec<i32>) -> Result<Vec<AnimeDetails>, diesel::result::Error> {
    let mut full_res: Vec<AnimeDetails> = vec![];
    let num_queries = ids.len() / MAX_QUERY_SIZE + 1;
    for i in 0..num_queries {
        let offset = i * MAX_QUERY_SIZE;
        if offset == ids.len() {
            break;
        }
        let target = match offset + MAX_QUERY_SIZE < ids.len() {
            true => offset + MAX_QUERY_SIZE + 1,
            false => ids.len(),
        };

        let mut query = anime.into_boxed().filter(id.eq(ids[offset]));
        for j in offset..target {
            query = query.or_filter(id.eq(ids[j]));
        }

        match query.load::<RawAnime>(&mut connection::POOL.get().unwrap()) {
            Ok(val) => full_res.append(&mut val.iter().map(|e| e.deserialize()).collect()),
            Err(err) => return Err(err),
        };
    }

    Ok(full_res)
}

pub fn insert(entries: Vec<RawAnime>) {
    let res = diesel::insert_into(anime)
        .values(&entries)
        .execute(&mut connection::POOL.get().unwrap());

    match res {
        Ok(num) => println!("(database) \x1b[34m\x1b[1mINFO!\x1b[0m inserted {} anime", num),
        Err(err) => println!("(database) \x1b[31m\x1b[1mERROR!\x1b[0m failed inserting anime (details: {:?})", err),
    };
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = anime)]
pub struct RawAnime {
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

impl RawAnime {
    pub fn deserialize(&self) -> AnimeDetails {
        AnimeDetails::new(
            self.id,
            self.title.to_owned(),
            self.picture.to_owned(),
            self.airing_date,
            self.mean,
            self.airing_status,
            self.genres.to_owned(),
            self.num_episodes,
            self.rating,
            match &self.related {
                Some(r) => conversion::from_serde_value(r.to_owned()),
                None => None,
            },
        )
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
