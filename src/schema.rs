// @generated automatically by Diesel CLI.

diesel::table! {
    post_likes (user_id, post_id) {
        user_id -> Integer,
        post_id -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        created_on -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        profile_image -> Text,
    }
}

diesel::joinable!(post_likes -> posts (post_id));
diesel::joinable!(post_likes -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    post_likes,
    posts,
    users,
);
