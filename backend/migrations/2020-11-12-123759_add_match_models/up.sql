CREATE TABLE matches (
    game_id BIGINT NOT NULL PRIMARY KEY,
    queue_id SMALLINT NOT NULL,
    game_type TEXT NOT NULL,
    game_duration BIGINT NOT NULL,
    platform_id TEXT NOT NULL,
    game_creation BIGINT NOT NULL,
    season_id SMALLINT NOT NULL,
    game_version TEXT NOT NULL,
    map_id SMALLINT NOT NULL,
    game_mode TEXT NOT NULL
);
CREATE TABLE team_stats (
    game_id BIGINT REFERENCES matches (game_id) ON UPDATE CASCADE ON DELETE CASCADE,
    team_id SMALLINT NOT NULL,
    tower_kills INT NOT NULL,
    rift_herald_kills INT NOT NULL,
    first_blood BOOLEAN NOT NULL,
    inhibitor_kills INT NOT NULL,
    first_baron BOOLEAN NOT NULL,
    first_dragon BOOLEAN NOT NULL,
    dominion_victory_score INT NOT NULL,
    dragon_kills INT NOT NULL,
    baron_kills INT NOT NULL,
    first_inhibitor BOOLEAN NOT NULL,
    first_tower BOOLEAN NOT NULL,
    vilemaw_kills INT NOT NULL,
    first_rift_herald BOOLEAN NOT NULL,
    win TEXT,
    PRIMARY KEY(game_id, team_id)
);
CREATE TABLE participants (
    id SERIAL NOT NULL PRIMARY KEY,
    game_id BIGINT NOT NULL REFERENCES matches (game_id) ON UPDATE CASCADE ON DELETE CASCADE,
    participant_id INT NOT NULL,
    summoner_id INT REFERENCES summoners (id),
    champion_id TEXT NOT NULL,
    team_id SMALLINT NOT NULL,
    spell1_id INT NOT NULL,
    spell2_id INT NOT NULL,
    highest_achieved_season_tier TEXT
);
CREATE TABLE participant_stats_general (
    participant_id INT REFERENCES participants (id) ON UPDATE CASCADE ON DELETE CASCADE PRIMARY KEY,
    champ_level INT NOT NULL,
    win BOOLEAN NOT NULL,
    gold_earned INT NOT NULL,
    gold_spent INT NOT NULL,
    item0 INT NOT NULL,
    item1 INT NOT NULL,
    item2 INT NOT NULL,
    item3 INT NOT NULL,
    item4 INT NOT NULL,
    item5 INT NOT NULL,
    item6 INT NOT NULL
);
CREATE TABLE participant_stats_kills (
    participant_id INT REFERENCES participants (id) ON UPDATE CASCADE ON DELETE CASCADE PRIMARY KEY,
    kills INT NOT NULL,
    deaths INT NOT NULL,
    assists INT NOT NULL,
    double_kills INT NOT NULL,
    triple_kills INT NOT NULL,
    quadra_kills INT NOT NULL,
    penta_kills INT NOT NULL,
    unreal_kills INT NOT NULL,
    largest_multi_kill INT NOT NULL,
    largest_killing_spree INT NOT NULL,
    killing_sprees INT NOT NULL,
    longest_time_spent_living INT NOT NULL,
    first_tower_kill BOOLEAN,
    first_tower_assist BOOLEAN,
    first_blood_kill BOOLEAN,
    first_blood_assist BOOLEAN,
    first_inhibitor_kill BOOLEAN,
    first_inhibitor_assist BOOLEAN,
    inhibitor_kills INT,
    turret_kills INT,
    neutral_minions_killed INT NOT NULL,
    neutral_minions_killed_enemy_jungle INT,
    neutral_minions_killed_team_jungle INT,
    total_minions_killed INT NOT NULL
);
CREATE TABLE participant_stats_damage (
    participant_id INT REFERENCES participants (id) ON UPDATE CASCADE ON DELETE CASCADE PRIMARY KEY,
    true_damage_dealt BIGINT NOT NULL,
    true_damage_dealt_to_champions BIGINT NOT NULL,
    true_damage_taken BIGINT NOT NULL,
    physical_damage_dealt BIGINT NOT NULL,
    physical_damage_dealt_to_champions BIGINT NOT NULL,
    physical_damage_taken BIGINT NOT NULL,
    magic_damage_dealt BIGINT NOT NULL,
    magic_damage_dealt_to_champions BIGINT NOT NULL,
    magical_damage_taken BIGINT NOT NULL,
    total_damage_dealt BIGINT NOT NULL,
    total_damage_dealt_to_champions BIGINT NOT NULL,
    total_damage_taken BIGINT NOT NULL,
    damage_dealt_to_turrets BIGINT NOT NULL,
    damage_dealt_to_objectives BIGINT NOT NULL,
    damage_self_mitigated BIGINT NOT NULL,
    largest_critical_strike INT NOT NULL
);
CREATE TABLE participant_stats_scores (
    participant_id INT REFERENCES participants (id) ON UPDATE CASCADE ON DELETE CASCADE PRIMARY KEY,
    combat_player_score INT,
    objective_player_score INT,
    total_player_score INT,
    total_score_rank INT,
    team_objective INT,
    player_score0 INT,
    player_score1 INT,
    player_score2 INT,
    player_score3 INT,
    player_score4 INT,
    player_score5 INT,
    player_score6 INT,
    player_score7 INT,
    player_score8 INT,
    player_score9 INT
);
CREATE TABLE participant_stats_utility (
    participant_id INT REFERENCES participants (id) ON UPDATE CASCADE ON DELETE CASCADE PRIMARY KEY,
    total_units_healed INT NOT NULL,
    total_heal BIGINT NOT NULL,
    total_time_crowd_control_dealt INT NOT NULL,
    time_c_cing_others BIGINT NOT NULL,
    wards_placed INT,
    vision_wards_bought_in_game INT NOT NULL,
    vision_score BIGINT,
    wards_killed INT,
    sight_wards_bought_in_game INT
);
CREATE TABLE match_references (
    game_id BIGINT NOT NULL PRIMARY KEY,
    summoner_id INT NOT NULL REFERENCES summoners (id) ON UPDATE CASCADE ON DELETE CASCADE,
    role TEXT,
    season INT NOT NULL,
    platform_id TEXT NOT NULL,
    champion TEXT NOT NULL,
    queue INT NOT NULL,
    lane TEXT,
    timestamp BIGINT NOT NULL
);