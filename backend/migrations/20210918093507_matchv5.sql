DROP TABLE IF EXISTS __diesel_schema_migrations;
-- STATS
ALTER TABLE team_stats
DROP COLUMN dominion_victory_score,
DROP COLUMN vilemaw_kills,
ALTER COLUMN tower_kills TYPE INT,
ALTER COLUMN rift_herald_kills TYPE INT,
ALTER COLUMN inhibitor_kills TYPE INT,
ALTER COLUMN dragon_kills TYPE INT,
ALTER COLUMN baron_kills TYPE INT;

ALTER TABLE team_stats
ALTER COLUMN win TYPE boolean
USING CASE win WHEN 'Win' THEN TRUE ELSE FALSE END;

-- MATCH
ALTER TABLE matches
ADD COLUMN game_name TEXT,
ADD COLUMN game_start_timestamp BIGINT,
ADD COLUMN game_end_timestamp BIGINT,
ADD COLUMN tournament_code TEXT,
ADD COLUMN data_version TEXT NOT NULL DEFAULT '1',
ADD COLUMN match_id TEXT;

ALTER TABLE matches
ALTER COLUMN data_version DROP DEFAULT;

ALTER TABLE participant_stats_general RENAME TO old_general;
CREATE TABLE participant_general (
    id SERIAL PRIMARY KEY,
    game_id BIGINT NOT NULL REFERENCES matches (game_id) ON UPDATE CASCADE ON DELETE CASCADE,
    summoner_id INT NOT NULL REFERENCES summoners (id) ON UPDATE CASCADE ON DELETE CASCADE,

    champion_id SMALLINT NOT NULL,
    champion_name TEXT NOT NULL,
    game_ended_in_early_surrender BOOL,
    game_ended_in_surrender BOOL,
    individual_position TEXT,
    participant_id SMALLINT NOT NULL,
    team_early_surrendered BOOL,
    team_id SMALLINT NOT NULL,
    team_position TEXT,
    win BOOL NOT NULL
);

INSERT INTO participant_general (game_id, summoner_id, champion_id, champion_name, team_id, win, individual_position, team_position, participant_id)
SELECT p.game_id, s.id, -1, p.champion_id, p.team_id, ts.win, 
CASE
    WHEN mr.lane = 'MID_LANE' AND mr.role = 'SOLO' THEN 'MIDDLE'
    WHEN mr.lane = 'TOP_LANE' AND mr.role = 'SOLO' THEN 'TOP'
    WHEN mr.lane = 'JUNGLE' AND mr.role = 'NONE' THEN 'JUNGLE'
    WHEN mr.lane = 'BOT_LANE' AND mr.role = 'DUO_CARRY' THEN 'BOTTOM'
    WHEN mr.lane = 'BOT_LANE' AND mr.role = 'DUO_SUPPORT' THEN 'UTILITY'
END,
CASE
    WHEN mr.lane = 'MID_LANE' AND mr.role = 'SOLO' THEN 'MIDDLE'
    WHEN mr.lane = 'TOP_LANE' AND mr.role = 'SOLO' THEN 'TOP'
    WHEN mr.lane = 'JUNGLE' AND mr.role = 'NONE' THEN 'JUNGLE'
    WHEN mr.lane = 'BOT_LANE' AND mr.role = 'DUO_CARRY' THEN 'BOTTOM'
    WHEN mr.lane = 'BOT_LANE' AND mr.role = 'DUO_SUPPORT' THEN 'UTILITY'
END,
p.participant_id
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN team_stats ts
ON ts.game_id = p.game_id AND ts.team_id = p.team_id
LEFT OUTER JOIN match_references mr
ON mr.game_id = p.game_id AND mr.summoner_id = s.id;

ALTER TABLE participant_stats_damage RENAME TO old_damage;
CREATE TABLE participant_damage (
    id INT PRIMARY KEY REFERENCES participant_general (id) ON UPDATE CASCADE ON DELETE CASCADE,

    damage_dealt_to_buildings INT,
    damage_dealt_to_objectives INT,
    damage_dealt_to_turrets INT,
    damage_self_mitigated INT,
    largest_critical_strike INT,
    magic_damage_dealt INT,
    magic_damage_dealt_to_champions INT,
    magic_damage_taken INT,
    physical_damage_dealt INT,
    physical_damage_dealt_to_champions INT,
    physical_damage_taken INT,
    total_damage_dealt INT,
    total_damage_dealt_to_champions INT,
    total_damage_shielded_on_teammates INT,
    total_damage_taken INT,
    total_heal INT,
    total_heals_on_teammates INT,
    total_units_healed INT,
    true_damage_dealt INT,
    true_damage_dealt_to_champions INT,
    true_damage_taken INT
);

INSERT INTO participant_damage (
    id, damage_dealt_to_objectives, damage_dealt_to_turrets, damage_self_mitigated, largest_critical_strike, magic_damage_dealt, magic_damage_dealt_to_champions,
    magic_damage_taken, physical_damage_dealt, physical_damage_dealt_to_champions, physical_damage_taken, total_damage_dealt, total_damage_dealt_to_champions, total_damage_taken,
    total_heal, total_units_healed, true_damage_dealt, true_damage_dealt_to_champions, true_damage_taken
)
SELECT pg.id, od.damage_dealt_to_objectives, od.damage_dealt_to_turrets, od.damage_self_mitigated, od.largest_critical_strike, od.magic_damage_dealt, od.magic_damage_dealt_to_champions,
    od.magical_damage_taken, od.physical_damage_dealt, od.physical_damage_dealt_to_champions, od.physical_damage_taken, od.total_damage_dealt, od.total_damage_dealt_to_champions, od.total_damage_taken,
    u.total_heal, u.total_units_healed, od.true_damage_dealt, od.true_damage_dealt_to_champions, od.true_damage_taken
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN old_damage od
ON od.participant_id = p.id
INNER JOIN participant_stats_utility u
ON u.participant_id = p.id
INNER JOIN participant_general pg
ON pg.game_id = p.game_id AND pg.summoner_id = s.id AND pg.participant_id = p.participant_id;

CREATE TABLE participant_items (
    id INT PRIMARY KEY REFERENCES participant_general (id) ON UPDATE CASCADE ON DELETE CASCADE,

    consumables_purchased SMALLINT,
    detector_wards_placed SMALLINT,
    item0 SMALLINT,
    item1 SMALLINT,
    item2 SMALLINT,
    item3 SMALLINT,
    item4 SMALLINT,
    item5 SMALLINT,
    item6 SMALLINT,
    items_purchased SMALLINT,
    sight_wards_bought_in_game SMALLINT,
    vision_wards_bought_in_game SMALLINT,
    wards_placed SMALLINT
);

INSERT INTO participant_items (id, item0, item1, item2, item3, item4, item5, item6, sight_wards_bought_in_game, vision_wards_bought_in_game, wards_placed)
SELECT pg.id, g.item0, g.item1, g.item2, g.item3, g.item4, g.item5, g.item6, u.sight_wards_bought_in_game, u.vision_wards_bought_in_game, u.wards_placed
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN old_general g
ON g.participant_id = p.id
INNER JOIN participant_stats_utility u
ON u.participant_id = p.id
INNER JOIN participant_general pg
ON pg.game_id = p.game_id AND pg.summoner_id = s.id AND pg.participant_id = p.participant_id;

CREATE TABLE participant_kda (
    id INT PRIMARY KEY REFERENCES participant_general (id) ON UPDATE CASCADE ON DELETE CASCADE,

    assists SMALLINT,
    baron_kills SMALLINT,
    deaths SMALLINT,
    double_kills SMALLINT,
    dragon_kills SMALLINT,
    first_blood_assist BOOL,
    first_blood_kill BOOL,
    first_tower_assist BOOL,
    first_tower_kill BOOL,
    inhibitor_kills SMALLINT,
    inhibitor_takedowns SMALLINT,
    inhibitors_lost SMALLINT,
    killing_sprees SMALLINT,
    kills SMALLINT,
    largest_killing_spree SMALLINT,
    largest_multi_kill SMALLINT,
    neutral_minions_killed SMALLINT,
    nexus_kills SMALLINT,
    nexus_takedowns SMALLINT,
    nexus_lost SMALLINT,
    objectives_stolen SMALLINT,
    objectives_stolen_assists SMALLINT,
    penta_kills SMALLINT,
    quadra_kills SMALLINT,
    total_minions_killed SMALLINT,
    triple_kills SMALLINT,
    turret_kills SMALLINT,
    turret_takedowns SMALLINT,
    turrets_lost SMALLINT,
    unreal_kills SMALLINT,
    wards_killed SMALLINT
);

INSERT INTO participant_kda (
    id, assists, deaths, double_kills, first_blood_assist, first_blood_kill, first_tower_assist, first_tower_kill, inhibitor_kills,
    killing_sprees, kills, largest_killing_spree, largest_multi_kill, neutral_minions_killed, penta_kills, quadra_kills, total_minions_killed, triple_kills,
    turret_kills, unreal_kills, wards_killed
)
SELECT pg.id, k.assists, k.deaths, k.double_kills, k.first_blood_assist, k.first_blood_kill, k.first_tower_assist, k.first_tower_kill, k.inhibitor_kills,
    k.killing_sprees, k.kills, k.largest_killing_spree, k.largest_multi_kill, k.neutral_minions_killed, k.penta_kills, k.quadra_kills, k.total_minions_killed, k.triple_kills,
    k.turret_kills, k.unreal_kills, u.wards_killed
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN old_general g
ON g.participant_id = p.id
INNER JOIN participant_stats_kills k
ON k.participant_id = p.id
INNER JOIN participant_stats_utility u
ON u.participant_id = p.id
INNER JOIN participant_general pg
ON pg.game_id = p.game_id AND pg.summoner_id = s.id AND pg.participant_id = p.participant_id;


CREATE TABLE participant_progress (
    id INT PRIMARY KEY REFERENCES participant_general (id) ON UPDATE CASCADE ON DELETE CASCADE,

    bounty_level SMALLINT,
    champ_experience INT,
    champ_level SMALLINT,
    champion_transform SMALLINT,
    gold_earned INT,
    gold_spent INT,
    longest_time_spent_living SMALLINT,
    time_c_cing_others SMALLINT,
    time_played SMALLINT,
    total_time_cc_dealt INT,
    total_time_spent_dead SMALLINT,
    vision_score SMALLINT
);

INSERT INTO participant_progress (id, champ_level, gold_earned, gold_spent, longest_time_spent_living, time_c_cing_others, total_time_cc_dealt, vision_score)
SELECT pg.id, g.champ_level, g.gold_earned, g.gold_spent, k.longest_time_spent_living, u.time_c_cing_others, u.total_time_crowd_control_dealt, u.vision_score
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN old_general g
ON g.participant_id = p.id
INNER JOIN participant_stats_kills k
ON k.participant_id = p.id
INNER JOIN participant_stats_utility u
ON u.participant_id = p.id
INNER JOIN participant_general pg
ON pg.game_id = p.game_id AND pg.summoner_id = s.id AND pg.participant_id = p.participant_id;

CREATE TABLE participant_spells (
    id INT PRIMARY KEY REFERENCES participant_general (id) ON UPDATE CASCADE ON DELETE CASCADE,

    spell1_casts SMALLINT,
    spell2_casts SMALLINT,
    spell3_casts SMALLINT,
    spell4_casts SMALLINT,
    summoner1_casts SMALLINT,
    summoner1_id SMALLINT,
    summoner2_casts SMALLINT,
    summoner2_id SMALLINT
);

INSERT INTO participant_spells (id, summoner1_id, summoner2_id)
SELECT pg.id, p.spell1_id, p.spell2_id
FROM participants p
INNER JOIN summoners s
ON p.summoner_id = s.id
INNER JOIN participant_general pg
ON pg.game_id = p.game_id AND pg.summoner_id = s.id AND pg.participant_id = p.participant_id;

DROP TABLE participant_stats_kills;
DROP TABLE participant_stats_scores;
DROP TABLE participant_stats_utility;
DROP TABLE old_general;
DROP TABLE old_damage;

-- Fix champion ids
UPDATE participant_general SET champion_id = CASE
    WHEN champion_name = 'Aatrox' THEN 266
    WHEN champion_name = 'Ahri' THEN 103
    WHEN champion_name = 'Akali' THEN 84
    WHEN champion_name = 'Akshan' THEN 166
    WHEN champion_name = 'Alistar' THEN 12
    WHEN champion_name = 'Amumu' THEN 32
    WHEN champion_name = 'Anivia' THEN 34
    WHEN champion_name = 'Annie' THEN 1
    WHEN champion_name = 'Aphelios' THEN 523
    WHEN champion_name = 'Ashe' THEN 22
    WHEN champion_name = 'AurelionSol' THEN 136
    WHEN champion_name = 'Azir' THEN 268
    WHEN champion_name = 'Bard' THEN 432
    WHEN champion_name = 'Blitzcrank' THEN 53
    WHEN champion_name = 'Brand' THEN 63
    WHEN champion_name = 'Braum' THEN 201
    WHEN champion_name = 'Caitlyn' THEN 51
    WHEN champion_name = 'Camille' THEN 164
    WHEN champion_name = 'Cassiopeia' THEN 69
    WHEN champion_name = 'Chogath' THEN 31
    WHEN champion_name = 'Corki' THEN 42
    WHEN champion_name = 'Darius' THEN 122
    WHEN champion_name = 'Diana' THEN 131
    WHEN champion_name = 'DrMundo' THEN 36
    WHEN champion_name = 'Draven' THEN 119
    WHEN champion_name = 'Ekko' THEN 245
    WHEN champion_name = 'Elise' THEN 60
    WHEN champion_name = 'Evelynn' THEN 28
    WHEN champion_name = 'Ezreal' THEN 81
    WHEN champion_name = 'FiddleSticks' THEN 9
    WHEN champion_name = 'Fiora' THEN 114
    WHEN champion_name = 'Fizz' THEN 105
    WHEN champion_name = 'Galio' THEN 3
    WHEN champion_name = 'Gangplank' THEN 41
    WHEN champion_name = 'Garen' THEN 86
    WHEN champion_name = 'Gnar' THEN 150
    WHEN champion_name = 'Gragas' THEN 79
    WHEN champion_name = 'Graves' THEN 104
    WHEN champion_name = 'Gwen' THEN 887
    WHEN champion_name = 'Hecarim' THEN 120
    WHEN champion_name = 'Heimerdinger' THEN 74
    WHEN champion_name = 'Illaoi' THEN 420
    WHEN champion_name = 'Irelia' THEN 39
    WHEN champion_name = 'Ivern' THEN 427
    WHEN champion_name = 'Janna' THEN 40
    WHEN champion_name = 'JarvanIV' THEN 59
    WHEN champion_name = 'Jax' THEN 24
    WHEN champion_name = 'Jayce' THEN 126
    WHEN champion_name = 'Jhin' THEN 202
    WHEN champion_name = 'Jinx' THEN 222
    WHEN champion_name = 'Kaisa' THEN 145
    WHEN champion_name = 'Kalista' THEN 429
    WHEN champion_name = 'Karma' THEN 43
    WHEN champion_name = 'Karthus' THEN 30
    WHEN champion_name = 'Kassadin' THEN 38
    WHEN champion_name = 'Katarina' THEN 55
    WHEN champion_name = 'Kayle' THEN 10
    WHEN champion_name = 'Kayn' THEN 141
    WHEN champion_name = 'Kennen' THEN 85
    WHEN champion_name = 'Khazix' THEN 121
    WHEN champion_name = 'Kindred' THEN 203
    WHEN champion_name = 'Kled' THEN 240
    WHEN champion_name = 'KogMaw' THEN 96
    WHEN champion_name = 'Leblanc' THEN 7
    WHEN champion_name = 'LeeSin' THEN 64
    WHEN champion_name = 'Leona' THEN 89
    WHEN champion_name = 'Lillia' THEN 876
    WHEN champion_name = 'Lissandra' THEN 127
    WHEN champion_name = 'Lucian' THEN 236
    WHEN champion_name = 'Lulu' THEN 117
    WHEN champion_name = 'Lux' THEN 99
    WHEN champion_name = 'Malphite' THEN 54
    WHEN champion_name = 'Malzahar' THEN 90
    WHEN champion_name = 'Maokai' THEN 57
    WHEN champion_name = 'MasterYi' THEN 11
    WHEN champion_name = 'MissFortune' THEN 21
    WHEN champion_name = 'MonkeyKing' THEN 62
    WHEN champion_name = 'Mordekaiser' THEN 82
    WHEN champion_name = 'Morgana' THEN 25
    WHEN champion_name = 'Nami' THEN 267
    WHEN champion_name = 'Nasus' THEN 75
    WHEN champion_name = 'Nautilus' THEN 111
    WHEN champion_name = 'Neeko' THEN 518
    WHEN champion_name = 'Nidalee' THEN 76
    WHEN champion_name = 'Nocturne' THEN 56
    WHEN champion_name = 'Nunu' THEN 20
    WHEN champion_name = 'Olaf' THEN 2
    WHEN champion_name = 'Orianna' THEN 61
    WHEN champion_name = 'Ornn' THEN 516
    WHEN champion_name = 'Pantheon' THEN 80
    WHEN champion_name = 'Poppy' THEN 78
    WHEN champion_name = 'Pyke' THEN 555
    WHEN champion_name = 'Qiyana' THEN 246
    WHEN champion_name = 'Quinn' THEN 133
    WHEN champion_name = 'Rakan' THEN 497
    WHEN champion_name = 'Rammus' THEN 33
    WHEN champion_name = 'RekSai' THEN 421
    WHEN champion_name = 'Rell' THEN 526
    WHEN champion_name = 'Renekton' THEN 58
    WHEN champion_name = 'Rengar' THEN 107
    WHEN champion_name = 'Riven' THEN 92
    WHEN champion_name = 'Rumble' THEN 68
    WHEN champion_name = 'Ryze' THEN 13
    WHEN champion_name = 'Samira' THEN 360
    WHEN champion_name = 'Sejuani' THEN 113
    WHEN champion_name = 'Senna' THEN 235
    WHEN champion_name = 'Seraphine' THEN 147
    WHEN champion_name = 'Sett' THEN 875
    WHEN champion_name = 'Shaco' THEN 35
    WHEN champion_name = 'Shen' THEN 98
    WHEN champion_name = 'Shyvana' THEN 102
    WHEN champion_name = 'Singed' THEN 27
    WHEN champion_name = 'Sion' THEN 14
    WHEN champion_name = 'Sivir' THEN 15
    WHEN champion_name = 'Skarner' THEN 72
    WHEN champion_name = 'Sona' THEN 37
    WHEN champion_name = 'Soraka' THEN 16
    WHEN champion_name = 'Swain' THEN 50
    WHEN champion_name = 'Sylas' THEN 517
    WHEN champion_name = 'Syndra' THEN 134
    WHEN champion_name = 'TahmKench' THEN 223
    WHEN champion_name = 'Taliyah' THEN 163
    WHEN champion_name = 'Talon' THEN 91
    WHEN champion_name = 'Taric' THEN 44
    WHEN champion_name = 'Teemo' THEN 17
    WHEN champion_name = 'Thresh' THEN 412
    WHEN champion_name = 'Tristana' THEN 18
    WHEN champion_name = 'Trundle' THEN 48
    WHEN champion_name = 'Tryndamere' THEN 23
    WHEN champion_name = 'TwistedFate' THEN 4
    WHEN champion_name = 'Twitch' THEN 29
    WHEN champion_name = 'Udyr' THEN 77
    WHEN champion_name = 'Urgot' THEN 6
    WHEN champion_name = 'Varus' THEN 110
    WHEN champion_name = 'Vayne' THEN 67
    WHEN champion_name = 'Veigar' THEN 45
    WHEN champion_name = 'Velkoz' THEN 161
    WHEN champion_name = 'Vi' THEN 254
    WHEN champion_name = 'Viego' THEN 234
    WHEN champion_name = 'Viktor' THEN 112
    WHEN champion_name = 'Vladimir' THEN 8
    WHEN champion_name = 'Volibear' THEN 106
    WHEN champion_name = 'Warwick' THEN 19
    WHEN champion_name = 'Xayah' THEN 498
    WHEN champion_name = 'Xerath' THEN 101
    WHEN champion_name = 'XinZhao' THEN 5
    WHEN champion_name = 'Yasuo' THEN 157
    WHEN champion_name = 'Yone' THEN 777
    WHEN champion_name = 'Yorick' THEN 83
    WHEN champion_name = 'Yuumi' THEN 350
    WHEN champion_name = 'Zac' THEN 154
    WHEN champion_name = 'Zed' THEN 238
    WHEN champion_name = 'Ziggs' THEN 115
    WHEN champion_name = 'Zilean' THEN 26
    WHEN champion_name = 'Zoe' THEN 142
    WHEN champion_name = 'Zyra' THEN 143
END;

CREATE TABLE _champions (
    id SMALLINT NOT NULL,
    name TEXT NOT NULL
);

INSERT INTO _champions (id, name) SELECT DISTINCT champion_id, champion_name FROM participant_general ORDER BY champion_id;

ALTER TABLE participant_general
DROP COLUMN champion_name;
DROP TABLE participants;
DROP TABLE match_references;