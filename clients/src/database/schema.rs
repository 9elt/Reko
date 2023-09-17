// @generated automatically by Diesel CLI.

diesel::table! {
    anime (id) {
        id -> Integer,
        #[max_length = 256]
        title -> Varchar,
        airing_date -> Nullable<Timestamp>,
        length -> Nullable<Integer>,
        mean -> Nullable<Float>,
        #[max_length = 8]
        rating -> Nullable<Varchar>,
        #[max_length = 64]
        picture -> Nullable<Varchar>,
        aired -> Bool,
        stats -> Longtext,
        updated_at -> Timestamp,
        parent -> Nullable<Integer>,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        score -> Integer,
        watched -> Bool,
        updated_at -> Timestamp,
        anime -> Integer,
        user -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 64]
        username -> Varchar,
        hash -> Unsigned<Bigint>,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(entries -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    entries,
    users,
);
