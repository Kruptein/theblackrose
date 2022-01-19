use sqlx::{query, query_as, Error, PgPool};

use crate::models::{connections::Connection, summoners::Summoner, users::User};

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

pub async fn get_connections(conn: &PgPool, user_id: i32) -> Result<Vec<Summoner>, Error> {
    sqlx::query_as!(
        Summoner,
        "SELECT s.* FROM connections c INNER JOIN summoners s ON c.summoner_id = s.id WHERE c.user_id = $1",
        user_id
    )
    .fetch_all(conn)
    .await
}

pub async fn get_connection_names(conn: &PgPool, user_id: i32) -> Result<Vec<String>, Error> {
    let names = query!(
        "
        SELECT s.name
        FROM summoners s
        INNER JOIN connections c
            ON c.summoner_id = s.id
        WHERE c.user_id = $1",
        user_id
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|record| record.name)
    .collect();

    Ok(names)
}

pub async fn get_summoner_ids_by_name(
    conn: &PgPool,
    names: Vec<String>,
) -> Result<Vec<i32>, Error> {
    let ids = query!(r#"SELECT s.id as "id!: i32" FROM summoners s INNER JOIN connections c ON c.summoner_id = s.id WHERE s.name = any($1)"#, &names).fetch_all(conn).await?;

    Ok(ids.into_iter().map(|i| i.id).collect())
}

pub async fn get_summoner_ids_by_connection(
    conn: &PgPool,
    user_id: i32,
) -> Result<Vec<i32>, Error> {
    let ids = query!("SELECT s.id FROM summoners s INNER JOIN connections c ON c.summoner_id = s.id WHERE c.user_id = $1", user_id).fetch_all(conn).await?;

    Ok(ids.into_iter().map(|i| i.id).collect())
}
