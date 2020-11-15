table! {
    connections (user_id, summoner_id) {
        user_id -> Int4,
        summoner_id -> Int4,
    }
}

table! {
    match_references (game_id) {
        game_id -> Int8,
        role -> Nullable<Text>,
        season -> Int4,
        platform_id -> Text,
        champion -> Text,
        queue -> Int4,
        lane -> Nullable<Text>,
        timestamp -> Int8,
    }
}

table! {
    matches (game_id) {
        game_id -> Int8,
        queue_id -> Int2,
        game_type -> Text,
        game_duration -> Int8,
        platform_id -> Text,
        game_creation -> Int8,
        season_id -> Int2,
        game_version -> Text,
        map_id -> Int2,
        game_mode -> Text,
    }
}

table! {
    participant_stats_damage (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        true_damage_dealt -> Int8,
        true_damage_dealt_to_champions -> Int8,
        true_damage_taken -> Int8,
        physical_damage_dealt -> Int8,
        physical_damage_dealt_to_champions -> Int8,
        physical_damage_taken -> Int8,
        magic_damage_dealt -> Int8,
        magic_damage_dealt_to_champions -> Int8,
        magical_damage_taken -> Int8,
        total_damage_dealt -> Int8,
        total_damage_dealt_to_champions -> Int8,
        total_damage_taken -> Int8,
        damage_dealt_to_turrets -> Int8,
        damage_dealt_to_objectives -> Int8,
        damage_self_mitigated -> Int8,
        largest_critical_strike -> Int4,
    }
}

table! {
    participant_stats_general (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        champ_level -> Int4,
        win -> Bool,
        gold_earned -> Int4,
        gold_spent -> Int4,
        item0 -> Int4,
        item1 -> Int4,
        item2 -> Int4,
        item3 -> Int4,
        item4 -> Int4,
        item5 -> Int4,
        item6 -> Int4,
    }
}

table! {
    participant_stats_kills (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        kills -> Int4,
        deaths -> Int4,
        assists -> Int4,
        double_kills -> Int4,
        triple_kills -> Int4,
        quadra_kills -> Int4,
        penta_kills -> Int4,
        unreal_kills -> Int4,
        largest_multi_kill -> Int4,
        largest_killing_spree -> Int4,
        killing_sprees -> Int4,
        longest_time_spent_living -> Int4,
        first_tower_kill -> Nullable<Bool>,
        first_tower_assist -> Nullable<Bool>,
        first_blood_kill -> Nullable<Bool>,
        first_blood_assist -> Nullable<Bool>,
        first_inhibitor_kill -> Nullable<Bool>,
        first_inhibitor_assist -> Nullable<Bool>,
        inhibitor_kills -> Nullable<Int4>,
        turret_kills -> Nullable<Int4>,
        neutral_minions_killed -> Int4,
        neutral_minions_killed_enemy_jungle -> Nullable<Int4>,
        neutral_minions_killed_team_jungle -> Nullable<Int4>,
        total_minions_killed -> Int4,
    }
}

table! {
    participant_stats_scores (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        combat_player_score -> Nullable<Int4>,
        objective_player_score -> Nullable<Int4>,
        total_player_score -> Nullable<Int4>,
        total_score_rank -> Nullable<Int4>,
        team_objective -> Nullable<Int4>,
        player_score0 -> Nullable<Int4>,
        player_score1 -> Nullable<Int4>,
        player_score2 -> Nullable<Int4>,
        player_score3 -> Nullable<Int4>,
        player_score4 -> Nullable<Int4>,
        player_score5 -> Nullable<Int4>,
        player_score6 -> Nullable<Int4>,
        player_score7 -> Nullable<Int4>,
        player_score8 -> Nullable<Int4>,
        player_score9 -> Nullable<Int4>,
    }
}

table! {
    participant_stats_utility (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        total_units_healed -> Int4,
        total_heal -> Int8,
        total_time_crowd_control_dealt -> Int4,
        time_c_cing_others -> Int8,
        wards_placed -> Nullable<Int4>,
        vision_wards_bought_in_game -> Int4,
        vision_score -> Nullable<Int8>,
        wards_killed -> Nullable<Int4>,
        sight_wards_bought_in_game -> Nullable<Int4>,
    }
}

table! {
    participants (game_id, participant_id) {
        game_id -> Int8,
        participant_id -> Int4,
        summoner_id -> Nullable<Int4>,
        champion_id -> Text,
        team_id -> Int2,
        spell1_id -> Int4,
        spell2_id -> Int4,
        highest_achieved_season_tier -> Nullable<Text>,
    }
}

table! {
    summoners (id) {
        id -> Int4,
        account_id -> Text,
        profile_icon_id -> Int4,
        revision_date -> Int8,
        name -> Text,

table! {
    team_stats (game_id, team_id) {
        game_id -> Int8,
        team_id -> Int2,
        tower_kills -> Int4,
        rift_herald_kills -> Int4,
        first_blood -> Bool,
        inhibitor_kills -> Int4,
        first_baron -> Bool,
        first_dragon -> Bool,
        dominion_victory_score -> Int4,
        dragon_kills -> Int4,
        baron_kills -> Int4,
        first_inhibitor -> Bool,
        first_tower -> Bool,
        vilemaw_kills -> Int4,
        first_rift_herald -> Bool,
        win -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        auth0_subject -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

joinable!(connections -> summoners (summoner_id));
joinable!(connections -> users (user_id));
joinable!(participant_stats_damage -> matches (game_id));
joinable!(participant_stats_general -> matches (game_id));
joinable!(participant_stats_kills -> matches (game_id));
joinable!(participant_stats_scores -> matches (game_id));
joinable!(participant_stats_utility -> matches (game_id));
joinable!(participants -> matches (game_id));
joinable!(participants -> summoners (summoner_id));
joinable!(team_stats -> matches (game_id));

allow_tables_to_appear_in_same_query!(
    connections,
    match_references,
    matches,
    participant_stats_damage,
    participant_stats_general,
    participant_stats_kills,
    participant_stats_scores,
    participant_stats_utility,
    participants,
    summoners,
    team_stats,
    users,
);
