// @generated automatically by Diesel CLI.

diesel::table! {
    anime (id) {
        id -> Int4,
        title -> Varchar,
        picture -> Nullable<Varchar>,
        mean -> Nullable<Int2>,
        airing_date -> Nullable<Date>,
        airing_status -> Nullable<Int2>,
        num_episodes -> Nullable<Int2>,
        rating -> Nullable<Int2>,
        genres -> Nullable<Array<Nullable<Int2>>>,
        related -> Nullable<Jsonb>,
    }
}

diesel::table! {
    lists (user_hash) {
        user_hash -> Varchar,
        list -> Nullable<Jsonb>,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    lists,
);
