table! {
    users (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        email -> Text,
        password_digest -> Text,
        created_at -> Timestamp,
    }
}
