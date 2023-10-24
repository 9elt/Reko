use super::schema::entries::dsl as E;
use super::schema::{anime as AnimeTable, anime::dsl as A};
use super::DBClient;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use structs::Anime as PublicAnime;
use structs::Stat;

impl DBClient {
    pub fn get_anime(&self, ids: Vec<i32>) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match A::anime.filter(A::id.eq_any(ids)).load::<Anime>(&mut conn) {
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

        diesel::insert_into(A::anime)
            .values(data)
            .execute(&mut conn)
            .is_ok()
    }
    pub fn update_anime(&self, data: PublicAnime) -> bool {
        let mut conn = self.connect();

        diesel::update(A::anime)
            .filter(A::id.eq(data.id))
            .set(AnimeUpdate::from_public(&data))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_airing_anime(&self) -> Vec<PublicAnime> {
        let mut conn = self.connect();

        let raw: Vec<Anime> = match A::anime
            .filter(A::aired.eq(false))
            .limit(100)
            .order(A::updated_at.asc())
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

        match E::entries
            .left_join(A::anime.on(A::id.eq(E::anime)))
            .select(E::anime)
            .filter(A::id.is_null())
            .group_by(E::anime)
            .limit(100)
            .order(E::updated_at.desc())
            .load::<i32>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => Vec::new(),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = AnimeTable)]
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
            stats: serde_json::from_str::<Vec<i32>>(&self.stats)
                .unwrap_or(Vec::new())
                .iter()
                .map(|id| Stat::new(id))
                .collect(),
            updated_at: self.updated_at,
            parent: self.parent,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = AnimeTable)]
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
