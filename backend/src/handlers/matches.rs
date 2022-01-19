use riven::models::match_v5 as R;
use sqlx::{Error, PgPool, Postgres, Transaction};

use crate::{
    db::matches::{add_match, add_match_team_stats, add_participant_stats},
    models::matches::Match,
};

use super::summoners::get_or_add_partial_summoner;

fn get_season_from_version(version: &str) -> i16 {
    if let Some(patch) = version.split(".").next() {
        patch.parse().unwrap_or(i16::MAX)
    } else {
        i16::MAX
    }
}

pub async fn add_match_details(conn: &PgPool, details: &R::Match) -> Result<Match, Error> {
    let mut tx = conn.begin().await?;

    let info = &details.info;
    let queue = u16::from(info.queue_id) as i16;
    let map: u8 = info.map_id.into();
    let map: i16 = map.into();
    let season_id = get_season_from_version(&info.game_version);

    let game = add_match(&mut tx, info, details, queue, map, season_id).await?;

    for team_stat in &info.teams {
        add_match_team_stats(&mut tx, info.game_id, team_stat).await;
    }

    for participant in &info.participants {
        add_participant(
            &mut tx,
            info.game_id,
            info.game_duration,
            participant,
            queue,
            season_id,
        )
        .await;
    }

    tx.commit().await?;

    Ok(game)
}

async fn add_participant(
    tx: &mut Transaction<'_, Postgres>,
    game_id: i64,
    game_duration: i64,
    participant: &R::Participant,
    queue_id: i16,
    season_id: i16,
) {
    let summoner = get_or_add_partial_summoner(tx, &participant).await.unwrap();

    add_participant_stats(
        tx,
        participant,
        Some(summoner.id),
        game_id,
        game_duration,
        queue_id,
        season_id,
    )
    .await;
}
