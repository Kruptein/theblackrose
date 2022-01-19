use riven::models::match_v5 as RM;
use sqlx::Error;
use sqlx::PgConnection;

use crate::db::summoners::add_partial_summoner;
use crate::db::summoners::get_summoner_by_puuid;
use crate::models::summoners::Summoner;

pub async fn get_or_add_partial_summoner(
    executor: &mut PgConnection,
    participant: &RM::Participant,
) -> Result<Summoner, Error> {
    let db_summoner = get_summoner_by_puuid(executor, &participant.puuid).await;
    match db_summoner {
        Ok(summoner) => Ok(summoner),
        Err(_) => add_partial_summoner(executor, participant).await,
    }
}
