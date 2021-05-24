use riven::models::match_v4 as R;
use sqlx::{query, query_as, Error, PgPool};
use strum::IntoEnumIterator;

use crate::models::{
    matches::{
        Match, MatchFeedElement, MatchFeedParticipant, MatchReference, Participant,
        ParticipantStatsGeneral, ParticipantStatsKills,
    },
    records::RecordType,
    summoners::Summoner,
};

use super::{notifications::send_connection_notification, summoners::get_or_add_partial_summoner};

pub async fn get_match_reference(
    conn: &PgPool,
    game_id: i64,
    summoner_id: i32,
) -> Result<MatchReference, Error> {
    query_as!(
        MatchReference,
        "SELECT * FROM match_references WHERE game_id = $1 AND summoner_id = $2",
        game_id,
        summoner_id
    )
    .fetch_one(conn)
    .await
}

pub async fn get_game_info(game_id: i64, conn: &PgPool) -> Result<MatchFeedElement, Error> {
    let match_info = get_match_details(conn, game_id).await?;

    let participants = query_as!(MatchFeedParticipant, r#"SELECT p as "participant!: Participant", psg as "general!: ParticipantStatsGeneral", psk as "kills!: ParticipantStatsKills", s as "summoner: Summoner" FROM participants p INNER JOIN participant_stats_general psg ON psg.participant_id = p.id INNER JOIN participant_stats_kills psk ON psk.participant_id = p.id LEFT OUTER JOIN summoners s ON p.summoner_id = s.id WHERE p.game_id = $1 ORDER BY psg.win"#, game_id).fetch_all(conn).await.unwrap();

    Ok(MatchFeedElement {
        match_info,
        participants,
    })
}

// MATCH DB INSERT FUNCTIONS

pub async fn add_match_reference(
    conn: &PgPool,
    match_reference: R::MatchReference,
    summoner_id: i32,
) -> Result<MatchReference, Error> {
    let queue: u16 = match_reference.queue.into();
    let queue: i32 = queue.into();
    query_as!(MatchReference, "INSERT INTO match_references (game_id, summoner_id, role, platform_id, champion, lane, queue, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
match_reference.game_id, summoner_id, match_reference.role, match_reference.platform_id, match_reference.champion.identifier(), match_reference.lane, queue, match_reference.timestamp).fetch_one(conn).await
}

pub async fn get_match_details(conn: &PgPool, game_id: i64) -> Result<Match, Error> {
    query_as!(Match, "SELECT * FROM matches WHERE game_id = $1", game_id)
        .fetch_one(conn)
        .await
}

fn get_season_from_version(version: &str) -> i16 {
    if let Some(patch) = version.split(".").next() {
        patch.parse().unwrap_or(i16::MAX)
    } else {
        i16::MAX
    }
}

pub async fn add_match_details(conn: &PgPool, match_details: R::Match) -> Result<Match, Error> {
    let queue: u16 = match_details.queue_id.into();
    let map: u8 = match_details.map_id.into();
    let map: i16 = map.into();
    let season_id = get_season_from_version(&match_details.game_version);
    let game_type: String = match_details.game_type.to_string();
    let game_mode: String = match_details.game_type.to_string();

    let game = query_as!(Match, "INSERT INTO matches (game_id, queue_id, game_type, game_duration, platform_id, game_creation, season_id, game_version, map_id, game_mode) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
match_details.game_id, queue as i16, game_type, match_details.game_duration, match_details.platform_id, match_details.game_creation, season_id, match_details.game_version, map, game_mode).fetch_one(conn).await;

    for team_stat in match_details.teams {
        add_match_team_stats(conn, match_details.game_id, team_stat).await;
    }

    let mut md = match_details.participant_identities.into_iter();
    for participant in match_details.participants {
        let identity = md.find(|p| p.participant_id == participant.participant_id);
        add_participant(
            conn,
            match_details.game_id,
            match_details.game_duration,
            participant,
            identity,
            queue,
            season_id,
        )
        .await;
    }

    game
}

async fn add_match_team_stats(conn: &PgPool, game_id: i64, stats: R::TeamStats) {
    let team: u8 = stats.team_id.into();
    let team: i16 = team.into();

    query!("INSERT INTO team_stats (game_id, team_id, tower_kills, rift_herald_kills, first_blood, inhibitor_kills, first_baron, first_dragon, dominion_victory_score, dragon_kills,baron_kills, first_inhibitor, first_tower, vilemaw_kills, first_rift_herald, win) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)", game_id, team.into(),
    stats.tower_kills,
    stats.rift_herald_kills,
    stats.first_blood,
    stats.inhibitor_kills,
    stats.first_baron,
    stats.first_dragon,
    stats.dominion_victory_score,
    stats.dragon_kills,
    stats.baron_kills,
    stats.first_inhibitor,
    stats.first_tower,
    stats.vilemaw_kills,
    stats.first_rift_herald,
    stats.win).execute(conn).await.unwrap();
}

async fn add_participant(
    conn: &PgPool,
    game_id: i64,
    game_duration: i64,
    participant: R::Participant,
    identity: Option<R::ParticipantIdentity>,
    queue_id: u16,
    season_id: i16,
) {
    let team: u8 = participant.team_id.into();
    let team: i16 = team.into();
    let highest_tier: Option<&str> = participant.highest_achieved_season_tier.map(|t| t.into());
    let summoner_id = match identity {
        Some(identity) => Some(
            get_or_add_partial_summoner(conn, identity.player)
                .await
                .unwrap()
                .id,
        ),
        None => None,
    };
    let new_participant = query_as!(Participant, "INSERT INTO participants (game_id, participant_id, summoner_id, champion_id, team_id, spell1_id, spell2_id, highest_achieved_season_tier) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
game_id, participant.participant_id, summoner_id, participant.champion_id.identifier(), team, participant.spell1_id, participant.spell2_id, highest_tier).fetch_one(conn).await.unwrap();

    add_participant_stats(
        conn,
        &new_participant,
        participant.stats,
        summoner_id,
        game_id,
        game_duration,
        queue_id,
        season_id,
    )
    .await;
}

struct RecordValue {
    id: i32,
    value: f32,
}

async fn add_participant_stats(
    conn: &PgPool,
    participant: &Participant,
    participant_stats: R::ParticipantStats,
    summoner_id: Option<i32>,
    game_id: i64,
    game_duration: i64,
    queue_id: u16,
    season_id: i16,
) {
    query!("INSERT INTO participant_stats_general (participant_id, champ_level, win, gold_earned, gold_spent, item0, item1, item2, item3, item4, item5, item6) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",participant.id,

    participant_stats.champ_level,
    participant_stats.win,

    participant_stats.gold_earned,
    participant_stats.gold_spent,

    participant_stats.item0,
    participant_stats.item1,
    participant_stats.item2,
    participant_stats.item3,
    participant_stats.item4,
    participant_stats.item5,
    participant_stats.item6).execute(conn).await.unwrap();

    query!("INSERT INTO participant_stats_kills (participant_id, kills, deaths, assists, double_kills, triple_kills, quadra_kills, penta_kills, unreal_kills, largest_multi_kill, largest_killing_spree, killing_sprees, longest_time_spent_living, first_tower_kill, first_tower_assist, first_blood_kill, first_blood_assist, first_inhibitor_kill, first_inhibitor_assist, inhibitor_kills, turret_kills, neutral_minions_killed, neutral_minions_killed_enemy_jungle, neutral_minions_killed_team_jungle, total_minions_killed) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)", participant.id,

    participant_stats.kills,
    participant_stats.deaths,
    participant_stats.assists,

    participant_stats.double_kills,
    participant_stats.triple_kills,
    participant_stats.quadra_kills,
    participant_stats.penta_kills,
    participant_stats.unreal_kills,
    participant_stats.largest_multi_kill,
    participant_stats.largest_killing_spree,
    participant_stats.killing_sprees,

    participant_stats.longest_time_spent_living,

    participant_stats.first_tower_kill,
    participant_stats.first_tower_assist,
    participant_stats.first_blood_kill,
    participant_stats.first_blood_assist,
    participant_stats.first_inhibitor_kill,
    participant_stats.first_inhibitor_assist,

    participant_stats.inhibitor_kills,
    participant_stats.turret_kills,

    participant_stats.neutral_minions_killed,
    participant_stats.neutral_minions_killed_enemy_jungle,
    participant_stats.neutral_minions_killed_team_jungle,
    participant_stats.total_minions_killed).execute(conn).await.unwrap();

    if let Some(summoner_id) = summoner_id {
        for record_type in RecordType::iter() {
            let game_value = record_type.get_value(&participant_stats, game_duration);
            let record_int = record_type.clone() as i16;
            let record = query_as!(RecordValue, "SELECT r.id, r.value FROM records r INNER JOIN matches m USING (game_id) WHERE r.record_type = $1 AND r.summoner_id = $2 AND m.queue_id = $3 AND m.season_id = $4",
                record_int,
                summoner_id,
                queue_id as i16,
                season_id).fetch_optional(conn).await;

            match record {
                Ok(Some(record_value)) => {
                    if record_value.value < game_value {
                        query!(
                            "UPDATE records SET value = $1, game_id = $2 WHERE id = $3",
                            game_value,
                            game_id,
                            record_value.id
                        )
                        .execute(conn)
                        .await
                        .unwrap();

                        let is_all_time_record: Result<i64, _> = query!(r#"SELECT COUNT(*) as "count!" FROM records r INNER JOIN matches m USING (game_id) WHERE r.record_type = $1 AND r.summoner_id = $2 AND m.queue_id = $3 AND r.value > $4"#, record_int, summoner_id, queue_id as i16, game_value).fetch_one(conn).await.map(|record| record.count);

                        let summoner_name: String =
                            query!("SELECT name FROM summoners WHERE id = $1", summoner_id)
                                .fetch_one(conn)
                                .await
                                .unwrap()
                                .name;

                        if is_all_time_record.unwrap_or(0) == 0 {
                            send_connection_notification(
                                conn,
                                summoner_id,
                                format!("New all time record for {}", summoner_name),
                                format!(
                                    "{} improved their \"{}\" record from {} to {}!",
                                    summoner_name, record_type, record_value.value, game_value
                                ),
                            )
                            .await;
                        } else {
                            send_connection_notification(
                                conn,
                                summoner_id,
                                format!("New season record for {}", summoner_name),
                                format!(
                                    "{} improved their \"{}\" record from {} to {}!",
                                    summoner_name, record_type, record_value.value, game_value
                                ),
                            )
                            .await;
                        }
                    }
                }
                Ok(None) => {
                    query!("INSERT INTO records (game_id, summoner_id, record_type, value) VALUES ($1, $2, $3, $4)", game_id, summoner_id, record_int, game_value).execute(conn).await.unwrap();
                }
                Err(_) => {}
            };
        }
    }

    query!("INSERT INTO participant_stats_damage (participant_id, true_damage_dealt, true_damage_dealt_to_champions, true_damage_taken, physical_damage_dealt, physical_damage_dealt_to_champions, physical_damage_taken, magic_damage_dealt, magic_damage_dealt_to_champions, magical_damage_taken, total_damage_dealt, total_damage_dealt_to_champions, total_damage_taken, damage_dealt_to_turrets, damage_dealt_to_objectives, damage_self_mitigated, largest_critical_strike) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)",participant.id,
    participant_stats.true_damage_dealt,
    participant_stats.true_damage_dealt_to_champions,
    participant_stats.true_damage_taken,
    participant_stats.physical_damage_dealt,
    participant_stats.physical_damage_dealt_to_champions,
    participant_stats.physical_damage_taken,
    participant_stats.magic_damage_dealt,
    participant_stats.magic_damage_dealt_to_champions,
    participant_stats.magical_damage_taken,
    participant_stats.total_damage_dealt,
    participant_stats.total_damage_dealt_to_champions,
    participant_stats.total_damage_taken,
    participant_stats.damage_dealt_to_turrets,
    participant_stats.damage_dealt_to_objectives,
    participant_stats.damage_self_mitigated,
    participant_stats.largest_critical_strike).execute(conn).await.unwrap();

    query!("INSERT INTO participant_stats_scores (participant_id, combat_player_score, 
        objective_player_score, total_player_score, total_score_rank, team_objective, player_score0, player_score1, player_score2, player_score3, player_score4, player_score5, player_score6, player_score7, player_score8, player_score9) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",participant.id,
    participant_stats.combat_player_score,
    participant_stats.objective_player_score,
    participant_stats.total_player_score,
    participant_stats.total_score_rank,
    participant_stats.team_objective,
    participant_stats.player_score0,
    participant_stats.player_score1,
    participant_stats.player_score2,
    participant_stats.player_score3,
    participant_stats.player_score4,
    participant_stats.player_score5,
    participant_stats.player_score6,
    participant_stats.player_score7,
    participant_stats.player_score8,
    participant_stats.player_score9,
).execute(conn).await.unwrap();

    query!("INSERT INTO participant_stats_utility (participant_id, total_units_healed, total_heal, total_time_crowd_control_dealt, time_c_cing_others, wards_placed, vision_wards_bought_in_game, vision_score, wards_killed, sight_wards_bought_in_game) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
    participant.id,
participant_stats.total_units_healed,
                    participant_stats.total_heal,
participant_stats.total_time_crowd_control_dealt,
participant_stats.time_c_cing_others,
participant_stats.wards_placed,
participant_stats.vision_wards_bought_in_game,
participant_stats.vision_score,
participant_stats.wards_killed,
participant_stats.sight_wards_bought_in_game).execute(conn).await.unwrap();
}
