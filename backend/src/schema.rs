table! {
    summoners (id) {
        id -> Int4,
        account_id -> Text,
        profile_icon_id -> Int4,
        revision_date -> Int8,
        name -> Text,
        summoner_id -> Text,
        puuid -> Text,
        summoner_level -> Int8,
    }
}

table! {
    users (id) {
        id -> Int4,
        auth0_subject -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(connections -> summoners (summoner_id));
joinable!(connections -> users (user_id));

allow_tables_to_appear_in_same_query!(
    summoners,
    users,
);
