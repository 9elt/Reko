use super::DBClient;
use diesel::prelude::*;
use diesel::sql_types as sql;
use structs::Hash;
use structs::Pagination;
use structs::SimilarUser as PublicSimilarUser;
use structs::User as PublicUser;
use util::{pub_page, similarity, HASH_SHIFT, MAX_PAGE_SIMILAR_USERS};

const SIMILAR_PAGE_SIZE: i32 = 32;
const SIMILAR_PAGE_TAKE: i32 = SIMILAR_PAGE_SIZE + 1;

impl DBClient {
    pub fn get_similar_users(
        &self,
        user: &PublicUser,
        page: u8,
    ) -> (Vec<PublicSimilarUser>, Pagination) {
        let mut pagination = Pagination::new(pub_page(page));

        let mut conn = self.connect();

        let id = user.id;
        let hash = user.hash.to_u64();
        let offset = page as i32 * SIMILAR_PAGE_SIZE;

        let mut raw = match diesel::sql_query(format!(
            "
        SELECT username, hash, (
            BIT_COUNT({hash} ^ hash) +
            BIT_COUNT(({hash} >> {HASH_SHIFT}) ^ (hash >> {HASH_SHIFT}))
        ) distance
        FROM users
        WHERE id != '{id}'
        ORDER BY distance ASC
        LIMIT {SIMILAR_PAGE_TAKE} OFFSET {offset};
        "
        ))
        .load::<SimilarUser>(&mut conn)
        {
            Ok(res) => res,
            Err(err) => {
                println!("err {:#?}", err);
                return (Vec::new(), pagination);
            }
        };

        let is_next_page = raw.len() == SIMILAR_PAGE_TAKE as usize;
        if is_next_page {
            raw.pop();
        }

        let mut res = Vec::with_capacity(raw.len());
        for u in raw {
            res.push(u.to_public());
        }

        if is_next_page && pub_page(page) < MAX_PAGE_SIMILAR_USERS {
            pagination.next = Some(pub_page(page) + 1);
        }

        (res, pagination)
    }
}

#[derive(QueryableByName)]
struct SimilarUser {
    #[diesel(sql_type = sql::VarChar)]
    username: String,
    #[diesel(sql_type = sql::Unsigned<sql::Bigint>)]
    hash: u64,
    #[diesel(sql_type = sql::Integer)]
    distance: i32,
}

impl SimilarUser {
    fn to_public(self) -> PublicSimilarUser {
        PublicSimilarUser {
            username: self.username,
            hash: Hash::BigInt(self.hash),
            similarity: similarity(self.distance),
        }
    }
}
