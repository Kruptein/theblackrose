use riven::models::match_v4 as RM;
use riven::models::summoner_v4 as RS;
use sqlx::{query, query_as, Error, PgPool};

use crate::models::summoners::Summoner;

pub async fn get_summoner_by_name(conn: &PgPool, summoner: &str) -> Result<Summoner, Error> {
    query_as!(
        Summoner,
        "SELECT * FROM summoners WHERE name = $1",
        summoner
    )
    .fetch_one(conn)
    .await
}

pub async fn add_summoner(conn: &PgPool, summoner: RS::Summoner) -> Result<Summoner, Error> {
    query_as!(Summoner, "INSERT INTO summoners (account_id, profile_icon_id, revision_date, name, summoner_id, puuid, summoner_level) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    summoner.account_id.as_str(),
    summoner.profile_icon_id,
    Some(summoner.revision_date),
    summoner.name.as_str(),
    Some(summoner.id.as_str()),
    Some(summoner.puuid.as_str()),
    Some(summoner.summoner_level)).fetch_one(conn).await
}

pub async fn update_summoner(conn: &PgPool, id: i32, summoner: RS::Summoner) -> Result<u64, Error> {
    query!(
        "UPDATE summoners SET profile_icon_id = $1, name = $2, revision_date = $3, summoner_level = $4, puuid = $5 WHERE id = $6",
        summoner.profile_icon_id,
        summoner.name,
        summoner.revision_date,
        summoner.summoner_level,
        summoner.puuid,
        id,
    )
    .execute(conn).await.map(|result| result.rows_affected())
}

pub async fn get_or_add_partial_summoner(
    conn: &PgPool,
    summoner: RM::Player,
) -> Result<Summoner, Error> {
    let acc_id = summoner.account_id.clone();
    let db_summoner = query_as!(
        Summoner,
        "SELECT * FROM summoners WHERE account_id = $1",
        acc_id
    )
    .fetch_one(conn)
    .await;
    match db_summoner {
        Ok(summoner) => Ok(summoner),
        Err(_) => add_partial_summoner(conn, summoner).await,
    }
}

async fn add_partial_summoner(conn: &PgPool, summoner: RM::Player) -> Result<Summoner, Error> {
    query_as!(Summoner, "INSERT INTO summoners (account_id, profile_icon_id, name, summoner_id) VALUES ($1, $2, $3, $4) RETURNING *", summoner.account_id.as_str(), summoner.profile_icon, summoner.summoner_name.as_str(), summoner.summoner_id.as_deref()).fetch_one(conn).await
}

pub async fn set_summoner_last_query_time(
    conn: &PgPool,
    summoner: i32,
    time: chrono::NaiveDateTime,
) -> Result<u64, Error> {
    query!(
        "UPDATE summoners SET last_match_query_time = $1 WHERE id = $2",
        time,
        summoner
    )
    .execute(conn)
    .await
    .map(|result| result.rows_affected())
}

pub async fn set_summoner_update_state(
    conn: &PgPool,
    summoner: i32,
    state: bool,
) -> Result<u64, Error> {
    query!(
        "UPDATE summoners SET update_in_progress = $1 WHERE id = $2",
        state,
        summoner
    )
    .execute(conn)
    .await
    .map(|result| result.rows_affected())
}

pub async fn set_all_summoners_update_state(conn: &PgPool, state: bool) -> Result<u64, Error> {
    query!("UPDATE summoners SET update_in_progress = $1", state)
        .execute(conn)
        .await
        .map(|result| result.rows_affected())
}
