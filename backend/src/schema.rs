table! {
    users (id) {
        id -> Int4,
        auth0_subject -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
