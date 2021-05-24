use sqlx::{query, query_as, Error, PgPool};

use crate::{
    models::{
        connections::Connection,
        matches::{MatchFeedElement, MatchReference},
        summoners::Summoner,
        users::User,
    },
    routes::matches::MatchFilter,
};

use super::matches::get_game_info;

pub async fn add_connection(
    conn: &PgPool,
    user: User,
    summoner: Summoner,
) -> Result<Connection, Error> {
    query_as!(
        Connection,
        "INSERT INTO connections (user_id, summoner_id) VALUES ($1, $2) RETURNING *",
        user.id,
        summoner.id
    )
    .fetch_one(conn)
    .await
}

pub async fn get_unique_connections(conn: &PgPool) -> Result<Vec<Connection>, Error> {
    query_as!(
        Connection,
        "SELECT DISTINCT ON(summoner_id) * FROM connections"
    )
    .fetch_all(conn)
    .await
}

pub async fn get_summoner(conn: &PgPool, connection: Connection) -> Result<Summoner, Error> {
    query_as!(
        Summoner,
        "SELECT * FROM summoners WHERE id = $1",
        connection.summoner_id
    )
    .fetch_one(conn)
    .await
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ConnectionShortInfo {
    name: String,
    profile_icon_id: i32,
}

pub async fn get_connection_short_info(
    conn: &PgPool,
    user: User,
) -> Result<Vec<ConnectionShortInfo>, Error> {
    sqlx::query_as!(ConnectionShortInfo, "SELECT name, profile_icon_id FROM connections c INNER JOIN summoners s ON c.summoner_id = s.id WHERE c.user_id = $1", user.id)
        .fetch_all(conn)
        .await
}

pub async fn get_connection_matches(
    conn: &PgPool,
    user: User,
    filter: MatchFilter,
) -> Result<Vec<MatchFeedElement>, Error> {
    let user_connections = match filter.get_names() {
        Some(names) => names.to_owned(),
        None => {
            query!("SELECT s.name FROM summoners s INNER JOIN connections c ON c.summoner_id = s.id WHERE c.user_id = $1", user.id).fetch_all(conn).await?.into_iter().map(|record| record.name).collect()
        }
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
    let references = query_as!(MatchReference, r#"SELECT DISTINCT ON(mr.timestamp) mr.* FROM match_references mr INNER JOIN summoners s ON mr.summoner_id = s.id WHERE s.name = any($1) AND mr.timestamp > $2 AND mr.timestamp < $3 ORDER BY mr.timestamp DESC LIMIT $4"#, &user_connections, after_time, before_time, length).fetch_all(conn).await?;

    let mut match_collection: Vec<MatchFeedElement> = vec![];
    for reference in references {
        match get_game_info(reference.game_id, conn).await {
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
