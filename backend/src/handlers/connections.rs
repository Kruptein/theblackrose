use sqlx::{Error, PgPool};

use crate::{
    db::{
        connections::get_connection_names,
        matches::{get_game_details, get_match_ids},
    },
    models::{matches::MatchFeedElement, users::User},
    routes::matches::MatchFilter,
};

pub async fn get_connection_matches(
    conn: &PgPool,
    user: User,
    filter: MatchFilter,
) -> Result<Vec<MatchFeedElement>, Error> {
    let user_connections = match filter.get_names() {
        Some(names) => names,
        None => get_connection_names(conn, user.id).await?,
    };
    let after_time = match filter.get_after_time() {
        Some(s) => s.to_owned(),
        None => 0,
    };
    let before_time = match filter.get_before_time() {
        Some(s) => s.to_owned(),
        None => i64::MAX,
    };

    let length: i64 = filter.get_length().into();
    let references = get_match_ids(
        conn,
        user_connections,
        filter.get_queues().unwrap_or_else(|| vec![450]),
        before_time,
        after_time,
        length,
    )
    .await?;

    let mut match_collection: Vec<MatchFeedElement> = vec![];
    for reference in references {
        match get_game_details(reference.game_id, conn).await {
            Ok(data) => match_collection.push(data),
            Err(err) => println!(
                "Could not get game info for {} ({})",
                reference.game_id, err
            ),
        }
    }
    // println!("{}", serde_json::to_string(&match_collection).unwrap());
    Ok(match_collection)
}
