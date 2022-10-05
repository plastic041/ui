// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        post_id -> Integer,
    }
}

diesel::joinable!(tags -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    tags,
);
