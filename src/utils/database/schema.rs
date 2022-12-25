// @generated automatically by Diesel CLI.

diesel::table! {
    analysis (users_count) {
        users_count -> Int4,
        mean -> Jsonb,
        std_dev -> Jsonb,
    }
}

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
    users (user_name) {
        user_name -> Varchar,
        list -> Jsonb,
        model -> Nullable<Jsonb>,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    analysis,
    anime,
    users,
);
