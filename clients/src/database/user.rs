use super::schema::{entries as EntriesTable, entries::dsl as E};
use super::schema::{users as UsersTable, users::dsl as U};
use super::DBClient;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types as sql;
use structs::DetailedListEntry as PublicDetailedListEntry;
use structs::Hash;
use structs::ListEntry as PublicListEntry;
use structs::Stat;
use structs::User as PublicUser;

impl DBClient {
    pub fn get_user(&self, name: String) -> Option<PublicUser> {
        let mut conn = self.connect();

        match U::users
            .filter(U::username.eq(name))
            .first::<User>(&mut conn)
        {
            Ok(user) => Some(user.to_public()),
            Err(_) => None,
        }
    }
    pub fn insert_user(&self, user: &PublicUser, etrs: Vec<PublicListEntry>) -> Option<i32> {
        let mut conn = self.connect();

        let res = diesel::insert_into(U::users)
            .values(UserInsert::from_public(user))
            .execute(&mut conn);

        if res.is_ok() {
            let uid = U::users
                .select(U::id)
                .order_by(U::id.desc())
                .first::<i32>(&mut conn)
                .unwrap();

            let data = etrs
                .iter()
                .map(|e| ListEntryInsert::from_public(uid, e))
                .collect::<Vec<_>>();

            diesel::insert_into(E::entries)
                .values(data)
                .execute(&mut conn)
                .ok();

            Some(uid)
        } else {
            None
        }
    }
    pub fn update_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        diesel::update(U::users)
            .filter(U::id.eq(user.id))
            .set(UserUpdate::from_public(user))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn delete_user(&self, user: &PublicUser) -> bool {
        let mut conn = self.connect();

        diesel::delete(U::users)
            .filter(U::id.eq(user.id))
            .execute(&mut conn)
            .is_ok()
    }
    pub fn get_user_entries(
        &self,
        user: &PublicUser,
        limit: usize,
    ) -> Vec<PublicDetailedListEntry> {
        let mut conn = self.connect();

        let raw: Vec<DetailedListEntry> = match diesel::sql_query(format!(
            "
        SELECT A.id, A.mean, A.stats, E.score
        FROM anime A
        INNER JOIN entries E ON E.anime = A.id
        WHERE E.user = {}
        AND E.watched = 1
        ORDER BY E.updated_at DESC
        LIMIT {};
        ",
            user.id, limit
        ))
        .load::<DetailedListEntry>(&mut conn)
        {
            Ok(entries) => entries,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for e in raw {
            res.push(e.to_public());
        }

        res
    }
    pub fn update_user_entries(&self, user: &PublicUser, entries: Vec<PublicListEntry>) -> bool {
        let mut conn = self.connect();

        let ids = entries.iter().map(|e| e.id).collect::<Vec<_>>();

        let update_ids: Vec<i32> = E::entries
            .select(E::anime)
            .filter(E::user.eq(&user.id))
            .filter(E::anime.eq_any(&ids))
            .get_results::<i32>(&mut conn)
            .unwrap_or(Vec::new());

        let mut missing = Vec::new();

        for entry in entries {
            let insert = ListEntryInsert::from_public(user.id, &entry);

            if update_ids.contains(&entry.id) {
                diesel::update(E::entries)
                    .filter(E::id.eq(entry.id))
                    .set(&insert)
                    .execute(&mut conn)
                    .ok();
            } else {
                missing.push(insert);
            }
        }

        if !missing.is_empty() {
            diesel::insert_into(E::entries)
                .values(missing)
                .on_conflict(diesel::dsl::DuplicatedKeys)
                .do_nothing()
                .execute(&mut conn)
                .is_ok()
        } else {
            true
        }
    }
    pub fn get_old_users(&self) -> Vec<PublicUser> {
        let mut conn = self.connect();

        let raw = match U::users
            .order(U::updated_at.asc())
            .limit(100)
            .get_results::<User>(&mut conn)
        {
            Ok(res) => res,
            Err(_) => return Vec::new(),
        };

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
        }

        res
    }
}

#[derive(Queryable, QueryableByName)]
struct DetailedListEntry {
    #[diesel(sql_type = sql::Integer)]
    id: i32,
    #[diesel(sql_type = sql::VarChar)]
    stats: String,
    #[diesel(sql_type = sql::Nullable<sql::Float>)]
    mean: Option<f32>,
    #[diesel(sql_type = sql::Integer)]
    score: i32,
}

impl DetailedListEntry {
    fn to_public(self) -> PublicDetailedListEntry {
        PublicDetailedListEntry {
            id: self.id,
            stats: serde_json::from_str::<Vec<i32>>(&self.stats)
                .unwrap_or(Vec::new())
                .iter()
                .map(|id| Stat::new(id))
                .collect(),
            score: self.score,
            mean: self.mean.unwrap_or(0.0) as i32,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = EntriesTable)]
struct ListEntry {
    id: i32,
    user: i32,
    anime: Option<i32>,
    score: i32,
    watched: bool,
    updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = EntriesTable)]
struct ListEntryInsert {
    user: i32,
    anime: i32,
    score: i32,
    watched: bool,
    updated_at: NaiveDateTime,
}

impl ListEntryInsert {
    fn from_public(usr_id: i32, entry: &PublicListEntry) -> Self {
        Self {
            user: usr_id,
            anime: entry.id,
            score: entry.score,
            watched: entry.watched,
            updated_at: entry.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = UsersTable)]
struct User {
    id: i32,
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

impl User {
    fn to_public(self) -> PublicUser {
        PublicUser {
            id: self.id,
            username: self.username,
            hash: Hash::BigInt(self.hash),
            updated_at: self.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = UsersTable)]
struct UserInsert {
    username: String,
    hash: u64,
    updated_at: NaiveDateTime,
}

impl UserInsert {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            username: user.username.to_owned(),
            hash: user.hash.to_u64(),
            updated_at: user.updated_at,
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = UsersTable)]
struct UserUpdate {
    hash: u64,
    updated_at: NaiveDateTime,
}

impl UserUpdate {
    fn from_public(user: &PublicUser) -> Self {
        Self {
            hash: user.hash.to_u64(),
            updated_at: user.updated_at,
        }
    }
}
