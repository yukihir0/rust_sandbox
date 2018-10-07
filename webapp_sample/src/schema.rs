table! {
    users (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        email -> Text,
        password_digest -> Text,
        session_digest -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
