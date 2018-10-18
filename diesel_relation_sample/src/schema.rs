table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        user_id -> Integer,
    }
}

table! {
    posts_tags (id) {
        id -> Integer,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    tags (id) {
        id -> Integer,
        label -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(posts -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    tags,
    users,
);
