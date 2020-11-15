use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};
use riven::models::match_v4 as R;

use crate::{
    db::Conn,
    models::matches::{
        Match, MatchReference, NewMatch, NewMatchReference, NewParticipant,
        NewParticipantStatsDamage, NewParticipantStatsGeneral, NewParticipantStatsKills,
        NewParticipantStatsScores, NewParticipantStatsUtility, NewTeamStats,
    },
    schema::match_references::dsl::match_references,
    schema::match_references::dsl::{self as mr},
    schema::matches::dsl::matches,
    schema::participant_stats_damage::dsl::participant_stats_damage,
    schema::participant_stats_general::dsl::participant_stats_general,
    schema::participant_stats_kills::dsl::participant_stats_kills,
    schema::participant_stats_scores::dsl::participant_stats_scores,
    schema::participant_stats_utility::dsl::participant_stats_utility,
    schema::participants::dsl::participants,
    schema::team_stats::dsl::team_stats,
};

use super::summoners::get_or_add_partial_summoner;

pub fn get_match_reference(conn: &Conn, game_id: i64) -> Result<MatchReference, Error> {
    match_references
        .filter(mr::game_id.eq(game_id))
        .get_result(conn)
}

pub fn add_match_reference(
    conn: &Conn,
    match_reference: R::MatchReference,
) -> Result<MatchReference, Error> {
    let queue: u16 = match_reference.queue.into();
    let new_match_reference = NewMatchReference {
        game_id: match_reference.game_id,
        role: match_reference.role,
        season: match_reference.season,
        platform_id: match_reference.platform_id,
        champion: match_reference.champion.into(),
        lane: match_reference.lane,
        queue: queue.into(),
        timestamp: match_reference.timestamp,
    };
    insert_into(match_references)
        .values(new_match_reference)
        .get_result(conn)
}

pub fn add_match_details(conn: &Conn, match_details: R::Match) -> Result<Match, Error> {
    let queue: u16 = match_details.queue_id.into();
    let season: u8 = match_details.season_id.into();
    let map: u8 = match_details.map_id.into();
    let new_match = NewMatch {
        game_id: match_details.game_id,
        queue_id: queue as i16,
        game_type: match_details.game_type.into(),
        game_duration: match_details.game_duration,
        platform_id: match_details.platform_id,
        game_creation: match_details.game_creation,
        season_id: season.into(),
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
        add_participant(conn, match_details.game_id, participant, identity);
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
    participant: R::Participant,
    identity: Option<R::ParticipantIdentity>,
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
        champion_id: participant.champion_id.into(),
        team_id: team.into(),
        spell1_id: participant.spell1_id,
        spell2_id: participant.spell2_id,
        highest_achieved_season_tier: participant.highest_achieved_season_tier.map(|t| t.into()),
    };
    insert_into(participants)
        .values(new_participant)
        .execute(conn)
        .unwrap();

    add_participant_stats(conn, game_id, participant.participant_id, participant.stats);
}

fn add_participant_stats(
    conn: &Conn,
    game_id: i64,
    participant_id: i32,
    participant_stats: R::ParticipantStats,
) {
    let new_participant_stats_general = NewParticipantStatsGeneral {
        game_id,
        participant_id,

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
        game_id,
        participant_id,

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

    let new_participant_stats_damage = NewParticipantStatsDamage {
        game_id,
        participant_id,

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
        game_id,
        participant_id,

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
        game_id,
        participant_id,

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
