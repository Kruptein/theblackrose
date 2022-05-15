use riven::{
    consts::{Champion, Team},
    models::spectator_v4::CurrentGameInfo,
};
use sqlx::{Error, PgPool};

use crate::db::stats::get_winrate_for_champion;

#[derive(Debug, Serialize)]
pub struct ParticipantLiveInfo {
    summoner: String,
    champion: Champion,
    team: Team,
    wins: u16,
    total: u16,
}

pub async fn get_live_info(
    conn: &PgPool,
    info: CurrentGameInfo,
) -> Result<Vec<ParticipantLiveInfo>, Error> {
    let mut participant_info = vec![];
    for participant in info.participants {
        let winrate = get_winrate_for_champion(
            conn,
            participant.champion_id,
            &participant.summoner_id,
            info.game_queue_config_id,
        )
        .await?;
        participant_info.push(ParticipantLiveInfo {
            summoner: participant.summoner_name,
            champion: participant.champion_id,
            team: participant.team_id,
            wins: winrate.wins,
            total: winrate.total,
        })
    }
    Ok(participant_info)
}
