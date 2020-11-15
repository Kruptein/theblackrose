use crate::schema::*;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct MatchReference {
    pub game_id: i64,
    pub role: Option<String>,
    pub season: i32,
    pub platform_id: String,
    pub champion: String,
    pub queue: i32,
    pub lane: Option<String>,
    pub timestamp: i64,
}

#[derive(Insertable, Debug)]
#[table_name = "match_references"]
pub struct NewMatchReference<'a> {
    pub game_id: i64,
    pub role: Option<String>,
    pub season: i32,
    pub platform_id: String,
    pub champion: &'a str,
    pub queue: i32,
    pub lane: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Match {
    pub game_id: i64,
    pub queue_id: i16,
    pub game_type: String,
    pub game_duration: i64,
    pub platform_id: String,
    pub game_creation: i64,
    pub season_id: i16,
    pub game_version: String,
    pub map_id: i16,
    pub game_mode: String,
}

#[derive(Insertable, Debug)]
#[table_name = "matches"]
pub struct NewMatch<'a> {
    pub game_id: i64,
    pub queue_id: i16,
    pub game_type: &'a str,
    pub game_duration: i64,
    pub platform_id: String,
    pub game_creation: i64,
    pub season_id: i16,
    pub game_version: String,
    pub map_id: i16,
    pub game_mode: &'a str,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct TeamStats {
    pub game_id: i64,
    pub team_id: i16,
    pub tower_kills: i32,
    pub rift_herald_kills: i32,
    pub first_blood: bool,
    pub inhibitor_kills: i32,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub baron_kills: i32,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub vilemaw_kills: i32,
    pub first_rift_herald: bool,
    pub win: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "team_stats"]
pub struct NewTeamStats {
    pub game_id: i64,
    pub team_id: i16,
    pub tower_kills: i32,
    pub rift_herald_kills: i32,
    pub first_blood: bool,
    pub inhibitor_kills: i32,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub baron_kills: i32,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub vilemaw_kills: i32,
    pub first_rift_herald: bool,
    pub win: Option<String>,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Participant {
    pub game_id: i64,
    pub participant_id: i32,
    pub summoner_id: Option<i32>,
    pub champion_id: String,
    pub team_id: i16,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub highest_achieved_season_tier: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "participants"]
pub struct NewParticipant<'a> {
    pub game_id: i64,
    pub participant_id: i32,
    pub summoner_id: Option<i32>,
    pub champion_id: &'a str,
    pub team_id: i16,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub highest_achieved_season_tier: Option<&'a str>,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct ParticipantStatsGeneral {
    pub game_id: i64,
    pub participant_id: i32,

    pub champ_level: i32,
    pub win: bool,

    pub gold_earned: i32,
    pub gold_spent: i32,

    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "participant_stats_general"]
pub struct NewParticipantStatsGeneral {
    pub game_id: i64,
    pub participant_id: i32,

    pub champ_level: i32,
    pub win: bool,

    pub gold_earned: i32,
    pub gold_spent: i32,

    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct ParticipantStatsKills {
    pub game_id: i64,
    pub participant_id: i32,

    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,

    pub double_kills: i32,
    pub triple_kills: i32,
    pub quadra_kills: i32,
    pub penta_kills: i32,
    pub unreal_kills: i32,
    pub largest_multi_kill: i32,
    pub largest_killing_spree: i32,
    pub killing_sprees: i32,

    pub longest_time_spent_living: i32,

    pub first_tower_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_blood_kill: Option<bool>,
    pub first_blood_assist: Option<bool>,
    pub first_inhibitor_kill: Option<bool>,
    pub first_inhibitor_assist: Option<bool>,

    pub inhibitor_kills: Option<i32>,
    pub turret_kills: Option<i32>,

    pub neutral_minions_killed: i32,
    pub neutral_minions_killed_enemy_jungle: Option<i32>,
    pub neutral_minions_killed_team_jungle: Option<i32>,
    pub total_minions_killed: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "participant_stats_kills"]
pub struct NewParticipantStatsKills {
    pub game_id: i64,
    pub participant_id: i32,

    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,

    pub double_kills: i32,
    pub triple_kills: i32,
    pub quadra_kills: i32,
    pub penta_kills: i32,
    pub unreal_kills: i32,
    pub largest_multi_kill: i32,
    pub largest_killing_spree: i32,
    pub killing_sprees: i32,

    pub longest_time_spent_living: i32,

    pub first_tower_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_blood_kill: Option<bool>,
    pub first_blood_assist: Option<bool>,
    pub first_inhibitor_kill: Option<bool>,
    pub first_inhibitor_assist: Option<bool>,

    pub inhibitor_kills: Option<i32>,
    pub turret_kills: Option<i32>,

    pub neutral_minions_killed: i32,
    pub neutral_minions_killed_enemy_jungle: Option<i32>,
    pub neutral_minions_killed_team_jungle: Option<i32>,
    pub total_minions_killed: i32,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct ParticipantStatsDamage {
    pub game_id: i64,
    pub participant_id: i32,

    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,

    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,

    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magical_damage_taken: i64,

    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_taken: i64,

    pub damage_dealt_to_turrets: i64,
    pub damage_dealt_to_objectives: i64,

    pub damage_self_mitigated: i64,
    pub largest_critical_strike: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "participant_stats_damage"]
pub struct NewParticipantStatsDamage {
    pub game_id: i64,
    pub participant_id: i32,

    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,

    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,

    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magical_damage_taken: i64,

    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_taken: i64,

    pub damage_dealt_to_turrets: i64,
    pub damage_dealt_to_objectives: i64,

    pub damage_self_mitigated: i64,
    pub largest_critical_strike: i32,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct ParticipantStatsScores {
    pub game_id: i64,
    pub participant_id: i32,

    pub combat_player_score: Option<i32>,
    pub objective_player_score: Option<i32>,
    pub total_player_score: Option<i32>,

    pub total_score_rank: Option<i32>,

    pub team_objective: Option<i32>,

    pub player_score0: Option<i32>,
    pub player_score1: Option<i32>,
    pub player_score2: Option<i32>,
    pub player_score3: Option<i32>,
    pub player_score4: Option<i32>,
    pub player_score5: Option<i32>,
    pub player_score6: Option<i32>,
    pub player_score7: Option<i32>,
    pub player_score8: Option<i32>,
    pub player_score9: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "participant_stats_scores"]
pub struct NewParticipantStatsScores {
    pub game_id: i64,
    pub participant_id: i32,

    pub combat_player_score: Option<i32>,
    pub objective_player_score: Option<i32>,
    pub total_player_score: Option<i32>,

    pub total_score_rank: Option<i32>,

    pub team_objective: Option<i32>,

    pub player_score0: Option<i32>,
    pub player_score1: Option<i32>,
    pub player_score2: Option<i32>,
    pub player_score3: Option<i32>,
    pub player_score4: Option<i32>,
    pub player_score5: Option<i32>,
    pub player_score6: Option<i32>,
    pub player_score7: Option<i32>,
    pub player_score8: Option<i32>,
    pub player_score9: Option<i32>,
}

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct ParticipantStatsUtility {
    pub game_id: i64,
    pub participant_id: i32,

    pub total_units_healed: i32,
    pub total_heal: i64,

    pub total_time_crowd_control_dealt: i32,
    pub time_c_cing_others: i64,

    pub wards_placed: Option<i32>,
    pub vision_wards_bought_in_game: i32,
    pub vision_score: Option<i64>,
    pub wards_killed: Option<i32>,
    pub sight_wards_bought_in_game: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "participant_stats_utility"]
pub struct NewParticipantStatsUtility {
    pub game_id: i64,
    pub participant_id: i32,

    pub total_units_healed: i32,
    pub total_heal: i64,

    pub total_time_crowd_control_dealt: i32,
    pub time_c_cing_others: i64,

    pub wards_placed: Option<i32>,
    pub vision_wards_bought_in_game: i32,
    pub vision_score: Option<i64>,
    pub wards_killed: Option<i32>,
    pub sight_wards_bought_in_game: Option<i32>,
}
