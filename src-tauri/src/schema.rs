// @generated automatically by Diesel CLI.

diesel::table! {
    post_tags (id) {
        id -> Integer,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

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
    }
}

diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    post_tags,
    posts,
    tags,
);
