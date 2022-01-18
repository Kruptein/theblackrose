use riven::models::match_v5 as RM;
use riven::models::summoner_v4 as RS;
use sqlx::Postgres;
use sqlx::Transaction;
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
        Some(summoner.summoner_level)
    ).fetch_one(conn).await
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
    tx: &mut Transaction<'_, Postgres>,
    participant: &RM::Participant,
) -> Result<Summoner, Error> {
    let db_summoner = query_as!(
        Summoner,
        "SELECT * FROM summoners WHERE puuid = $1",
        participant.puuid
    )
    .fetch_one(&mut *tx)
    .await;
    match db_summoner {
        Ok(summoner) => Ok(summoner),
        Err(_) => add_partial_summoner(tx, participant).await,
    }
}

async fn add_partial_summoner(
    tx: &mut Transaction<'_, Postgres>,
    participant: &RM::Participant,
) -> Result<Summoner, Error> {
    query_as!(
        Summoner,
        "INSERT INTO summoners (puuid, profile_icon_id, name, summoner_id)
        VALUES ($1, $2, $3, $4) RETURNING *",
        participant.puuid,
        participant.profile_icon,
        participant.summoner_name,
        participant.summoner_id
    )
    .fetch_one(tx)
    .await
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

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QuickStats {
    total_played: i64,
    season_played: i64,
    total_win: i64,
    season_win: i64,
    total_kills: i64,
    season_kills: i64,
    total_deaths: i64,
    season_deaths: i64,
    total_assists: i64,
    season_assists: i64,
}

pub async fn get_summoner_quick_stats(
    conn: &PgPool,
    summoner: &Summoner,
) -> Result<QuickStats, Error> {
    query_as!(
        QuickStats,
        r#"
        SELECT
            COUNT(*) as "total_played!",
            COUNT(*) filter (WHERE m.game_start_timestamp >= $2) as "season_played!",
            COUNT(*) filter (WHERE win) as "total_win!",
            COUNT(*) filter (WHERE win AND m.game_start_timestamp >= $2) as "season_win!",
            SUM(kills) as "total_kills!",
            COALESCE(SUM(kills) filter (WHERE m.game_start_timestamp >= $2), 0) as "season_kills!",
            SUM(deaths) as "total_deaths!",
            COALESCE(SUM(deaths) filter (WHERE m.game_start_timestamp >= $2), 0) as "season_deaths!",
            SUM(assists) as "total_assists!",
            COALESCE(SUM(assists) filter (WHERE m.game_start_timestamp >= $2), 0) as "season_assists!"
        FROM participant_general pg
        INNER JOIN participant_kda pk ON pk.id = pg.id
        INNER JOIN matches m ON m.game_id = pg.game_id
        WHERE
            pg.summoner_id = $1
        "#,
        summoner.id,
        1641524400000 // season 11 start day
    )
    .fetch_one(conn)
    .await
}
