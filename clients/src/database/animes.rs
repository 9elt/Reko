use super::schema::entries::dsl as entries;
use super::schema::{anime as table_anime, anime::dsl as anime};
use super::DBClient;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use structs::Anime as PublicAnime;

impl DBClient {
    pub fn get_anime(&self, ids: Vec<i32>) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match anime::anime
            .filter(anime::id.eq_any(ids))
            .load::<Anime>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for ani in raw {
            res.push(ani.to_public());
        }

        res
    }
    pub fn insert_anime(&self, data: Vec<PublicAnime>) -> bool {
        let data = data
            .iter()
            .map(|ani| Anime::from_public(ani))
            .collect::<Vec<_>>();

        let mut conn = self.connect();

        diesel::insert_into(anime::anime)
            .values(data)
            .execute(&mut conn)
            .is_ok()
    }
    pub fn update_anime(&self, data: PublicAnime) -> bool {
        let mut conn = self.connect();

        diesel::update(anime::anime)
            .filter(anime::id.eq(data.id))
            .set(AnimeUpdate::from_public(&data))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_airing_anime(&self) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match anime::anime
            .filter(anime::aired.eq(false))
            .load::<Anime>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for ani in raw {
            res.push(ani.to_public());
        }

        res
    }
    pub fn get_missing_anime(&self) -> Vec<i32> {
        let mut conn = self.connect();

        match entries::entries
            .left_join(anime::anime.on(anime::id.eq(entries::anime)))
            .select(entries::anime)
            .filter(entries::anime.is_null())
            .load::<i32>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => Vec::new(),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_anime)]
struct Anime {
    id: i32,
    title: String,
    airing_date: Option<NaiveDateTime>,
    length: Option<i32>,
    mean: Option<f32>,
    rating: Option<String>,
    picture: Option<String>,
    aired: bool,
    stats: String,
    updated_at: NaiveDateTime,
    parent: Option<i32>,
}

impl Anime {
    fn from_public(ani: &PublicAnime) -> Self {
        Self {
            id: ani.id,
            title: ani.title.to_owned(),
            airing_date: ani.airing_date,
            length: ani.length,
            mean: ani.mean,
            rating: ani.rating.to_owned(),
            picture: ani.picture.to_owned(),
            aired: ani.aired,
            stats: serde_json::json!(ani.stats).to_string(),
            updated_at: ani.updated_at,
            parent: ani.parent,
        }
    }
    fn to_public(self) -> PublicAnime {
        PublicAnime {
            id: self.id,
            title: self.title,
            airing_date: self.airing_date,
            length: self.length,
            mean: self.mean,
            rating: self.rating,
            picture: self.picture,
            aired: self.aired,
            stats: serde_json::from_str::<Vec<i32>>(&self.stats).unwrap_or(Vec::new()),
            updated_at: self.updated_at,
            parent: self.parent,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = table_anime)]
struct AnimeUpdate {
    title: String,
    airing_date: Option<NaiveDateTime>,
    length: Option<i32>,
    mean: Option<f32>,
    rating: Option<String>,
    picture: Option<String>,
    aired: bool,
    stats: String,
    updated_at: NaiveDateTime,
    parent: Option<i32>,
}

impl AnimeUpdate {
    fn from_public(ani: &PublicAnime) -> Self {
        Self {
            title: ani.title.to_owned(),
            airing_date: ani.airing_date,
            length: ani.length,
            mean: ani.mean,
            rating: ani.rating.to_owned(),
            picture: ani.picture.to_owned(),
            aired: ani.aired,
            stats: serde_json::json!(ani.stats).to_string(),
            updated_at: ani.updated_at,
            parent: ani.parent,
        }
    }
}
