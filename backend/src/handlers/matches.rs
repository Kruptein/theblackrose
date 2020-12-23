use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};
use riven::models::match_v4 as R;
use strum::IntoEnumIterator;

use crate::{
    db::Conn,
    models::{
        matches::{
            Match, MatchFeedElement, MatchFeedParticipant, MatchReference, NewMatch,
            NewMatchReference, NewParticipant, NewParticipantStatsDamage,
            NewParticipantStatsGeneral, NewParticipantStatsKills, NewParticipantStatsScores,
            NewParticipantStatsUtility, NewTeamStats, Participant, ParticipantStatsGeneral,
            ParticipantStatsKills,
        },
        records::{NewRecord, RecordType},
        summoners::Summoner,
    },
    schema::match_references::dsl::{self as mr, match_references},
    schema::matches::dsl::{self as m, matches},
    schema::participant_stats_damage::dsl::participant_stats_damage,
    schema::participant_stats_general::dsl::{self as psg, participant_stats_general},
    schema::participant_stats_kills::dsl::{self as psk, participant_stats_kills},
    schema::participant_stats_scores::dsl::participant_stats_scores,
    schema::participant_stats_utility::dsl::participant_stats_utility,
    schema::participants::dsl::{self as p},
    schema::records::dsl::{self as r},
    schema::summoners::dsl::{self as s},
    schema::team_stats::dsl::team_stats,
};

use super::{notifications::send_connection_notification, summoners::get_or_add_partial_summoner};

pub fn get_match_reference(
    conn: &Conn,
    game_id: i64,
    summoner_id: i32,
) -> Result<MatchReference, Error> {
    match_references
        .filter(mr::game_id.eq(game_id))
        .filter(mr::summoner_id.eq(summoner_id))
        .get_result(conn)
}

pub fn get_game_info(game_id: i64, conn: &Conn) -> Result<MatchFeedElement, Error> {
    let match_info: Match = matches.filter(m::game_id.eq(game_id)).get_result(conn)?;
    let participants: Vec<(
        Participant,
        ParticipantStatsGeneral,
        ParticipantStatsKills,
        Summoner,
    )> = p::participants
        .inner_join(psg::participant_stats_general)
        .inner_join(psk::participant_stats_kills)
        .inner_join(s::summoners)
        .filter(p::game_id.eq(game_id))
        .order_by(psg::win)
        .get_results(conn)?;
    let participants: Vec<MatchFeedParticipant> = participants
        .into_iter()
        .map(|p| MatchFeedParticipant {
            participant: p.0,
            general: p.1,
            kills: p.2,
            summoner: p.3,
        })
        .collect();
    Ok(MatchFeedElement {
        match_info,
        participants,
    })
}

// MATCH DB INSERT FUNCTIONS

pub fn add_match_reference(
    conn: &Conn,
    match_reference: R::MatchReference,
    summoner_id: i32,
) -> Result<MatchReference, Error> {
    let queue: u16 = match_reference.queue.into();
    let new_match_reference = NewMatchReference {
        game_id: match_reference.game_id,
        summoner_id,
        role: match_reference.role,
        platform_id: match_reference.platform_id,
        champion: match_reference.champion.identifier(),
        lane: match_reference.lane,
        queue: queue.into(),
        timestamp: match_reference.timestamp,
    };
    insert_into(match_references)
        .values(new_match_reference)
        .get_result(conn)
}

pub fn get_match_details(conn: &Conn, game_id: i64) -> Result<Match, Error> {
    matches.filter(m::game_id.eq(game_id)).get_result(conn)
}

fn get_season_from_version(version: &str) -> i16 {
    if let Some(patch) = version.split(".").next() {
        patch.parse().unwrap_or(i16::MAX)
    } else {
        i16::MAX
    }
}

pub fn add_match_details(conn: &Conn, match_details: R::Match) -> Result<Match, Error> {
    let queue: u16 = match_details.queue_id.into();
    let map: u8 = match_details.map_id.into();
    let season_id = get_season_from_version(&match_details.game_version);
    let new_match = NewMatch {
        game_id: match_details.game_id,
        queue_id: queue as i16,
        game_type: match_details.game_type.into(),
        game_duration: match_details.game_duration,
        platform_id: match_details.platform_id,
        game_creation: match_details.game_creation,
        season_id,
        game_version: match_details.game_version,
        map_id: map.into(),
        game_mode: match_details.game_mode.into(),
    };

    let game = insert_into(matches).values(new_match).get_result(conn);

    for team_stat in match_details.teams {
        add_match_team_stats(conn, match_details.game_id, team_stat);
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
        );
    }

    game
}

fn add_match_team_stats(conn: &Conn, game_id: i64, stats: R::TeamStats) {
    let team: u8 = stats.team_id.into();
    let new_team_stats = NewTeamStats {
        game_id,
        team_id: team.into(),
        tower_kills: stats.tower_kills,
        rift_herald_kills: stats.rift_herald_kills,
        first_blood: stats.first_blood,
        inhibitor_kills: stats.inhibitor_kills,
        first_baron: stats.first_baron,
        first_dragon: stats.first_dragon,
        dominion_victory_score: stats.dominion_victory_score,
        dragon_kills: stats.dragon_kills,
        baron_kills: stats.baron_kills,
        first_inhibitor: stats.first_inhibitor,
        first_tower: stats.first_tower,
        vilemaw_kills: stats.vilemaw_kills,
        first_rift_herald: stats.first_rift_herald,
        win: stats.win,
    };
    insert_into(team_stats)
        .values(new_team_stats)
        .execute(conn)
        .unwrap();
}

fn add_participant(
    conn: &Conn,
    game_id: i64,
    game_duration: i64,
    participant: R::Participant,
    identity: Option<R::ParticipantIdentity>,
    queue_id: u16,
    season_id: i16,
) {
    let team: u8 = participant.team_id.into();
    let summoner_id = match identity {
        Some(identity) => Some(
            get_or_add_partial_summoner(conn, identity.player)
                .unwrap()
                .id,
        ),
        None => None,
    };
    let new_participant = NewParticipant {
        game_id,
        participant_id: participant.participant_id,
        summoner_id,
        champion_id: participant.champion_id.identifier(),
        team_id: team.into(),
        spell1_id: participant.spell1_id,
        spell2_id: participant.spell2_id,
        highest_achieved_season_tier: participant.highest_achieved_season_tier.map(|t| t.into()),
    };
    let new_participant = insert_into(p::participants)
        .values(new_participant)
        .get_result(conn)
        .unwrap();

    add_participant_stats(
        conn,
        &new_participant,
        participant.stats,
        summoner_id,
        game_id,
        game_duration,
        queue_id,
        season_id,
    );
}

fn add_participant_stats(
    conn: &Conn,
    participant: &Participant,
    participant_stats: R::ParticipantStats,
    summoner_id: Option<i32>,
    game_id: i64,
    game_duration: i64,
    queue_id: u16,
    season_id: i16,
) {
    let new_participant_stats_general = NewParticipantStatsGeneral {
        participant_id: participant.id,

        champ_level: participant_stats.champ_level,
        win: participant_stats.win,

        gold_earned: participant_stats.gold_earned,
        gold_spent: participant_stats.gold_spent,

        item0: participant_stats.item0,
        item1: participant_stats.item1,
        item2: participant_stats.item2,
        item3: participant_stats.item3,
        item4: participant_stats.item4,
        item5: participant_stats.item5,
        item6: participant_stats.item6,
    };
    insert_into(participant_stats_general)
        .values(new_participant_stats_general)
        .execute(conn)
        .unwrap();

    let new_participant_stats_kills = NewParticipantStatsKills {
        participant_id: participant.id,

        kills: participant_stats.kills,
        deaths: participant_stats.deaths,
        assists: participant_stats.assists,

        double_kills: participant_stats.double_kills,
        triple_kills: participant_stats.triple_kills,
        quadra_kills: participant_stats.quadra_kills,
        penta_kills: participant_stats.penta_kills,
        unreal_kills: participant_stats.unreal_kills,
        largest_multi_kill: participant_stats.largest_multi_kill,
        largest_killing_spree: participant_stats.largest_killing_spree,
        killing_sprees: participant_stats.killing_sprees,

        longest_time_spent_living: participant_stats.longest_time_spent_living,

        first_tower_kill: participant_stats.first_tower_kill,
        first_tower_assist: participant_stats.first_tower_assist,
        first_blood_kill: participant_stats.first_blood_kill,
        first_blood_assist: participant_stats.first_blood_assist,
        first_inhibitor_kill: participant_stats.first_inhibitor_kill,
        first_inhibitor_assist: participant_stats.first_inhibitor_assist,

        inhibitor_kills: participant_stats.inhibitor_kills,
        turret_kills: participant_stats.turret_kills,

        neutral_minions_killed: participant_stats.neutral_minions_killed,
        neutral_minions_killed_enemy_jungle: participant_stats.neutral_minions_killed_enemy_jungle,
        neutral_minions_killed_team_jungle: participant_stats.neutral_minions_killed_team_jungle,
        total_minions_killed: participant_stats.total_minions_killed,
    };
    insert_into(participant_stats_kills)
        .values(new_participant_stats_kills)
        .execute(conn)
        .unwrap();

    if let Some(summoner_id) = summoner_id {
        for record_type in RecordType::iter() {
            let game_value = record_type.get_value(&participant_stats, game_duration);
            let record_int = record_type.clone() as i16;
            let record: Result<Option<(i32, f32)>, Error> = r::records
                .inner_join(matches)
                .select((r::id, r::value))
                .filter(r::record_type.eq(record_int))
                .filter(r::summoner_id.eq(summoner_id))
                .filter(m::queue_id.eq(queue_id as i16))
                .filter(m::season_id.eq(season_id))
                .get_result(conn)
                .optional();

            match record {
                Ok(Some((id, value))) => {
                    if value < game_value {
                        diesel::update(r::records.filter(r::id.eq(id)))
                            .set((r::value.eq(game_value), r::game_id.eq(game_id)))
                            .execute(conn)
                            .unwrap();

                        let is_all_time_record: Result<i64, Error> = r::records
                            .inner_join(matches)
                            .filter(r::record_type.eq(record_int))
                            .filter(r::summoner_id.eq(summoner_id))
                            .filter(m::queue_id.eq(queue_id as i16))
                            .filter(r::value.gt(game_value))
                            .count()
                            .get_result(conn);

                        let summoner_name: String = s::summoners
                            .select(s::name)
                            .filter(s::id.eq(summoner_id))
                            .get_result(conn)
                            .unwrap();

                        if is_all_time_record.unwrap_or(0) > 0 {
                            send_connection_notification(
                                conn,
                                summoner_id,
                                format!("New all time record for {}", summoner_name),
                                format!(
                                    "{} improved their \"{}\" record from {} to {}!",
                                    summoner_name, record_type, value, game_value
                                ),
                            );
                        } else {
                            send_connection_notification(
                                conn,
                                summoner_id,
                                format!("New season record for {}", summoner_name),
                                format!(
                                    "{} improved their \"{}\" record from {} to {}!",
                                    summoner_name, record_type, value, game_value
                                ),
                            );
                        }
                    }
                }
                Ok(None) => {
                    let record = NewRecord {
                        game_id,
                        summoner_id,
                        record_type: record_int,
                        value: game_value,
                    };
                    insert_into(r::records)
                        .values(record)
                        .execute(conn)
                        .unwrap();
                }
                Err(_) => {}
            };
        }
    }

    let new_participant_stats_damage = NewParticipantStatsDamage {
        participant_id: participant.id,

        true_damage_dealt: participant_stats.true_damage_dealt,
        true_damage_dealt_to_champions: participant_stats.true_damage_dealt_to_champions,
        true_damage_taken: participant_stats.true_damage_taken,

        physical_damage_dealt: participant_stats.physical_damage_dealt,
        physical_damage_dealt_to_champions: participant_stats.physical_damage_dealt_to_champions,
        physical_damage_taken: participant_stats.physical_damage_taken,

        magic_damage_dealt: participant_stats.magic_damage_dealt,
        magic_damage_dealt_to_champions: participant_stats.magic_damage_dealt_to_champions,
        magical_damage_taken: participant_stats.magical_damage_taken,

        total_damage_dealt: participant_stats.total_damage_dealt,
        total_damage_dealt_to_champions: participant_stats.total_damage_dealt_to_champions,
        total_damage_taken: participant_stats.total_damage_taken,

        damage_dealt_to_turrets: participant_stats.damage_dealt_to_turrets,
        damage_dealt_to_objectives: participant_stats.damage_dealt_to_objectives,

        damage_self_mitigated: participant_stats.damage_self_mitigated,
        largest_critical_strike: participant_stats.largest_critical_strike,
    };
    insert_into(participant_stats_damage)
        .values(new_participant_stats_damage)
        .execute(conn)
        .unwrap();

    let new_participant_stats_scores = NewParticipantStatsScores {
        participant_id: participant.id,

        combat_player_score: participant_stats.combat_player_score,
        objective_player_score: participant_stats.objective_player_score,
        total_player_score: participant_stats.total_player_score,

        total_score_rank: participant_stats.total_score_rank,

        team_objective: participant_stats.team_objective,

        player_score0: participant_stats.player_score0,
        player_score1: participant_stats.player_score1,
        player_score2: participant_stats.player_score2,
        player_score3: participant_stats.player_score3,
        player_score4: participant_stats.player_score4,
        player_score5: participant_stats.player_score5,
        player_score6: participant_stats.player_score6,
        player_score7: participant_stats.player_score7,
        player_score8: participant_stats.player_score8,
        player_score9: participant_stats.player_score9,
    };
    insert_into(participant_stats_scores)
        .values(new_participant_stats_scores)
        .execute(conn)
        .unwrap();

    let new_participant_stats_utility = NewParticipantStatsUtility {
        participant_id: participant.id,

        total_units_healed: participant_stats.total_units_healed,
        total_heal: participant_stats.total_heal,

        total_time_crowd_control_dealt: participant_stats.total_time_crowd_control_dealt,
        time_c_cing_others: participant_stats.time_c_cing_others,

        wards_placed: participant_stats.wards_placed,
        vision_wards_bought_in_game: participant_stats.vision_wards_bought_in_game,
        vision_score: participant_stats.vision_score,
        wards_killed: participant_stats.wards_killed,
        sight_wards_bought_in_game: participant_stats.sight_wards_bought_in_game,
    };
    insert_into(participant_stats_utility)
        .values(new_participant_stats_utility)
        .execute(conn)
        .unwrap();
}
