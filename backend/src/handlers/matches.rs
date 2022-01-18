use std::convert::{TryInto};

use riven::models::match_v5 as R;
use sqlx::{query, query_as, Error, PgPool, Transaction, Postgres};
use strum::IntoEnumIterator;

use crate::models::{
    matches::{
        Match, MatchFeedElement, MatchFeedParticipant,
        ParticipantGeneral, ParticipantItems, ParticipantKda, ParticipantProgress, ParticipantSpells
    },
    records::RecordType,
    summoners::Summoner,
};

use super::{notifications::send_connection_notification, summoners::get_or_add_partial_summoner};

// pub async fn get_match_reference(
//     conn: &PgPool,
//     game_id: i64,
//     summoner_id: i32,
// ) -> Result<MatchReference, Error> {
//     query_as!(
//         MatchReference,
//         "SELECT * FROM match_references WHERE game_id = $1 AND summoner_id = $2",
//         game_id,
//         summoner_id
//     )
//     .fetch_one(conn)
//     .await
// }

pub async fn has_game(
    conn: &PgPool,
    platform_id: &str, game_id: i64
) -> Result<bool, Error> {
    let count = query!(
        r#"SELECT COUNT(*) as "count!" FROM matches WHERE platform_id = $1 AND game_id = $2"#,
        platform_id,
        game_id
    )
    .fetch_one(conn)
    .await?
    .count;
    Ok(count > 0)
}

pub async fn get_game_details(game_id: i64, conn: &PgPool) -> Result<MatchFeedElement, Error> {
    let match_info = get_match_info(conn, game_id).await?;

    let participants = query_as!(MatchFeedParticipant, 
        r#"
        SELECT
            pg as "general!: ParticipantGeneral",
            pi as "items!: ParticipantItems",
            pk as "kda!: ParticipantKda",
            pp as "progress!: ParticipantProgress",
            ps as "spells!: ParticipantSpells",
            s as "summoner: Summoner"
        FROM participant_general pg
        INNER JOIN participant_items pi
            ON pi.id = pg.id
        INNER JOIN participant_kda pk
            ON pk.id = pg.id
        INNER JOIN participant_progress pp
            ON pp.id = pg.id
        INNER JOIN participant_spells ps
            ON ps.id = pg.id
        LEFT OUTER JOIN summoners s
            ON pg.summoner_id = s.id
        WHERE
            pg.game_id = $1
        ORDER BY pg.win"#,
         game_id).fetch_all(conn).await.unwrap();

    Ok(MatchFeedElement {
        match_info,
        participants,
    })
}

// MATCH DB INSERT FUNCTIONS

// pub async fn add_match_reference(
//     conn: &PgPool,
//     info: R::Info,
//     summoner: &Summoner,
// ) -> Result<MatchReference, Error> {
//     let queue: i32 = u16::from(info.queue_id).into();
//     let puuid = summoner.puuid.as_ref().unwrap();
//     let participant = info.participants.into_iter().find(|p| &p.puuid == puuid).unwrap();
//     query_as!(MatchReference, "
//         INSERT INTO match_references (
//             game_id, summoner_id, role, platform_id, champion, lane, queue, timestamp
//         ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
//         info.game_id,
//         summoner.id,
//         participant.role,
//         info.platform_id,
//         participant.champion().unwrap().identifier(),
//         participant.lane,
//         queue,
//         info.game_start_timestamp
//     ).fetch_one(conn).await
// }

pub async fn get_match_info(conn: &PgPool, game_id: i64) -> Result<Match, Error> {
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

pub async fn add_match_details(conn: &PgPool, details: &R::Match) -> Result<Match, Error> {
    let mut tx = conn.begin().await?;

    let info = &details.info;
    let queue: u16 = info.queue_id.into();
    let map: u8 = info.map_id.into();
    let map: i16 = map.into();
    let season_id = get_season_from_version(&info.game_version);

    let game = query_as!(
        Match,
        "INSERT INTO matches (
            game_creation, game_duration, game_id, game_mode, game_name,
            game_start_timestamp, game_end_timestamp, game_type, game_version,
            map_id, platform_id, queue_id, season_id, tournament_code,
            data_version, match_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING *",
        info.game_creation,
        info.game_duration,
        info.game_id,
        info.game_mode.to_string(),
        info.game_name,
        info.game_start_timestamp,
        info.game_end_timestamp,
        info.game_type.to_string(),
        info.game_version,
        map,
        info.platform_id,
        queue as i16,
        season_id,
        info.tournament_code,
        details.metadata.data_version,
        details.metadata.match_id
    )
    .fetch_one(&mut tx)
    .await?;

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

async fn add_match_team_stats(tx: &mut Transaction<'_, Postgres>, game_id: i64, stats: &R::Team) {
    let team: u8 = u16::from(stats.team_id).try_into().unwrap();
    let team: i16 = team.into();
    let objectives = &stats.objectives;

    query!("
        INSERT INTO team_stats (
            game_id, team_id, tower_kills, rift_herald_kills, first_blood, inhibitor_kills, first_baron, first_dragon,
            dragon_kills, baron_kills, first_inhibitor, first_tower, first_rift_herald, win
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
        game_id,
        team.into(),
        objectives.tower.kills,
        objectives.rift_herald.kills,
        objectives.champion.first,
        objectives.inhibitor.kills,
        objectives.baron.first,
        objectives.dragon.first,
        objectives.dragon.kills,
        objectives.baron.kills,
        objectives.inhibitor.first,
        objectives.tower.first,
        objectives.rift_herald.first,
        stats.win
    ).execute(tx).await.unwrap();
}

async fn add_participant(
    tx: &mut Transaction<'_, Postgres>,
    game_id: i64,
    game_duration: i64,
    participant: &R::Participant,
    queue_id: u16,
    season_id: i16,
) {
    let summoner = get_or_add_partial_summoner(tx, &participant)
        .await
        .unwrap();
    
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

struct RecordValue {
    id: i32,
    value: f32,
}

async fn add_participant_stats(
    tx: &mut Transaction<'_, Postgres>,
    stats: &R::Participant,
    summoner_id: Option<i32>,
    game_id: i64,
    game_duration: i64,
    queue_id: u16,
    season_id: i16,
) {
    // let profile_icon: i16 = stats.profile_icon.try_into().unwrap();
    // let summoner_level: i16 = stats.summoner_level.try_into().unwrap();

    // query!("INSERT INTO participant_account (
    //     game_id, summoner_id, participant_id, profile_icon, puuid, riot_id_name, riot_id_tagline, summoner_level, summoner_name)
    //     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    //     game_id, summoner_id, participant_id, profile_icon, stats.puuid, stats.riot_id_name,
    //     stats.riot_id_tagline, summoner_level, stats.summoner_name
    // ).execute(conn).await.unwrap();

    let champion_id: i16 = stats.champion().unwrap().0;
    let participant_id: i16 = stats.participant_id.try_into().unwrap();
    let team_id: i16 = i32::from(u16::from(stats.team_id)).try_into().unwrap();

    let pg = query_as!(ParticipantGeneral,
        "
    INSERT INTO participant_general (
        game_id, summoner_id, champion_id, game_ended_in_early_surrender, game_ended_in_surrender,
        individual_position, participant_id, team_early_surrendered, team_id, team_position, win
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
    RETURNING *
    ",
        game_id,
        summoner_id,
        champion_id,
        stats.game_ended_in_early_surrender,
        stats.game_ended_in_surrender,
        stats.individual_position,
        participant_id,
        stats.team_early_surrendered,
        team_id,
        stats.team_position,
        stats.win,
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    query!("
    INSERT INTO participant_damage (
        id, damage_dealt_to_buildings, damage_dealt_to_objectives, damage_dealt_to_turrets,
        damage_self_mitigated, largest_critical_strike, magic_damage_dealt, magic_damage_dealt_to_champions,
        magic_damage_taken, physical_damage_dealt, physical_damage_dealt_to_champions, physical_damage_taken,
        total_damage_dealt, total_damage_dealt_to_champions, total_damage_shielded_on_teammates, total_damage_taken,
        total_heal, total_heals_on_teammates, total_units_healed, true_damage_dealt, true_damage_dealt_to_champions, true_damage_taken
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)",
    pg.id,
    stats.damage_dealt_to_buildings.unwrap_or(0),
    stats.damage_dealt_to_objectives,
    stats.damage_dealt_to_turrets,
    stats.damage_self_mitigated,
    stats.largest_critical_strike,
    stats.magic_damage_dealt,
    stats.magic_damage_dealt_to_champions,
    stats.magic_damage_taken,
    stats.physical_damage_dealt,
    stats.physical_damage_dealt_to_champions,
    stats.physical_damage_taken,
    stats.total_damage_dealt,
    stats.total_damage_dealt_to_champions,
    stats.total_damage_shielded_on_teammates,
    stats.total_damage_taken,
    stats.total_heal,
    stats.total_heals_on_teammates,
    stats.total_units_healed,
    stats.true_damage_dealt,
    stats.true_damage_dealt_to_champions,
    stats.true_damage_taken
    
).execute(&mut *tx).await.unwrap();

let consumables_purchased: i16 = stats.consumables_purchased.try_into().unwrap();
let detector_wards_placed: i16 = stats.detector_wards_placed.try_into().unwrap();
let item0: i16 = stats.item0.try_into().unwrap();
let item1: i16 = stats.item1.try_into().unwrap();
let item2: i16 = stats.item2.try_into().unwrap();
let item3: i16 = stats.item3.try_into().unwrap();
let item4: i16 = stats.item4.try_into().unwrap();
let item5: i16 = stats.item5.try_into().unwrap();
let item6: i16 = stats.item6.try_into().unwrap();
let items_purchased: i16 = stats.items_purchased.try_into().unwrap();
let sight_wards_bought_in_game: i16 = stats.sight_wards_bought_in_game.try_into().unwrap();
let vision_wards_bought_in_game: i16 = stats.vision_wards_bought_in_game.try_into().unwrap();
let wards_placed: i16 = stats.wards_placed.try_into().unwrap();

query!("
    INSERT INTO participant_items (
        id, consumables_purchased, detector_wards_placed,
        item0, item1, item2, item3, item4, item5, item6,
        items_purchased, sight_wards_bought_in_game, vision_wards_bought_in_game,
        wards_placed
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13, $14
    )",
    pg.id,
    consumables_purchased,
    detector_wards_placed,
    item0,
    item1,
    item2,
    item3,
    item4,
    item5,
    item6,
    items_purchased,
    sight_wards_bought_in_game,
    vision_wards_bought_in_game,
    wards_placed,
    ).execute(&mut *tx).await.unwrap();

    let assists: i16 = stats.assists.try_into().unwrap();
    let deaths: i16 = stats.deaths.try_into().unwrap();
    let double_kills: i16 = stats.double_kills.try_into().unwrap();
    let dragon_kills: i16 = stats.dragon_kills.try_into().unwrap();
    let inhibitor_kills: i16 = stats.inhibitor_kills.try_into().unwrap();
    let inhibitor_takedowns: Option<i16> = stats.inhibitor_takedowns.map(TryInto::try_into).transpose().unwrap();
    let inhibitors_lost: Option<i16> = stats.inhibitors_lost.map(TryInto::try_into).transpose().unwrap();
    let killing_sprees: i16 = stats.killing_sprees.try_into().unwrap();
    let kills: i16 = stats.kills.try_into().unwrap();
    let largest_killing_spree: i16 = stats.largest_killing_spree.try_into().unwrap();
    let largest_multi_kill: i16 = stats.largest_multi_kill.try_into().unwrap();
    let neutral_minions_killed: i16 = stats.neutral_minions_killed.try_into().unwrap();
    let nexus_kills: i16 = stats.nexus_kills.try_into().unwrap();
    let nexus_lost: Option<i16> = stats.nexus_lost.map(TryInto::try_into).transpose().unwrap();
    let nexus_takedowns: Option<i16> = stats.nexus_takedowns.map(TryInto::try_into).transpose().unwrap();
    let objectives_stolen: i16 = stats.objectives_stolen.try_into().unwrap();
    let objectives_stolen_assists: i16 = stats.objectives_stolen_assists.try_into().unwrap();
    let penta_kills: i16 = stats.penta_kills.try_into().unwrap();
    let quadra_kills: i16 = stats.quadra_kills.try_into().unwrap();
    let total_minions_killed: i16 = stats.total_minions_killed.try_into().unwrap();
    let triple_kills: i16 = stats.triple_kills.try_into().unwrap();
    let turret_kills: i16 = stats.turret_kills.try_into().unwrap();
    let turrets_lost: Option<i16> = stats.turrets_lost.map(TryInto::try_into).transpose().unwrap();
    let turret_takedowns: Option<i16> = stats.turret_takedowns.map(TryInto::try_into).transpose().unwrap();
    let unreal_kills: i16 = stats.unreal_kills.try_into().unwrap();
    let wards_killed: i16 = stats.wards_killed.try_into().unwrap();

    query!("
    INSERT INTO participant_kda (
        id, assists, deaths, double_kills, dragon_kills, first_blood_assist, first_blood_kill,
        first_tower_assist, first_tower_kill, inhibitor_kills, inhibitor_takedowns, inhibitors_lost,
        killing_sprees, kills, largest_killing_spree, largest_multi_kill, neutral_minions_killed, nexus_kills,
        nexus_takedowns, nexus_lost, objectives_stolen, objectives_stolen_assists, penta_kills, quadra_kills,
        total_minions_killed, triple_kills, turret_kills, turret_takedowns, turrets_lost, unreal_kills, wards_killed
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
        $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,
        $31
    )",
    pg.id,
    assists,
    deaths,
    double_kills,
    dragon_kills,
    stats.first_blood_assist,
    stats.first_blood_kill,
    stats.first_tower_assist,
    stats.first_tower_kill,
    inhibitor_kills,
    inhibitor_takedowns,
    inhibitors_lost,
    killing_sprees,
    kills,
    largest_killing_spree,
    largest_multi_kill,
    neutral_minions_killed,
    nexus_kills,
    nexus_takedowns,
    nexus_lost,
    objectives_stolen,
    objectives_stolen_assists,
    penta_kills,
    quadra_kills,
    total_minions_killed,
    triple_kills,
    turret_kills,
    turret_takedowns,
    turrets_lost,
    unreal_kills,
    wards_killed
    ).execute(&mut *tx).await.unwrap();

    let bounty_level: i16 = stats.bounty_level.try_into().unwrap();
    let champ_level: i16 = stats.champ_level.try_into().unwrap();
    let champion_transform: i16 = stats.champion_transform.try_into().unwrap();

    let longest_time_spent_living: i16 = stats.longest_time_spent_living.try_into().unwrap();
    let time_c_cing_others: i16 = stats.time_c_cing_others.try_into().unwrap();
    let time_played: i16 = stats.time_played.try_into().unwrap();

    let total_time_spent_dead: i16 = stats.total_time_spent_dead.try_into().unwrap();
    let vision_score: i16 = stats.vision_score.try_into().unwrap();

    query!("
    INSERT INTO participant_progress (
        id, bounty_level, champ_experience, champ_level, champion_transform,
        gold_earned, gold_spent, longest_time_spent_living, time_c_cing_others, time_played,
        total_time_cc_dealt, total_time_spent_dead, vision_score
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
        $11, $12, $13
    )",
    pg.id,
    bounty_level,
    stats.champ_experience,
    champ_level,
    champion_transform,
    stats.gold_earned,
    stats.gold_spent,
    longest_time_spent_living,
    time_c_cing_others,
    time_played,
    stats.total_time_cc_dealt,
    total_time_spent_dead,
    vision_score
    ).execute(&mut *tx).await.unwrap();

    let spell1_casts: i16 = stats.spell1_casts.try_into().unwrap();
    let spell2_casts: i16 = stats.spell2_casts.try_into().unwrap();
    let spell3_casts: i16 = stats.spell3_casts.try_into().unwrap();
    let spell4_casts: i16 = stats.spell4_casts.try_into().unwrap();
    let summoner1_casts: i16 = stats.summoner1_casts.try_into().unwrap();
    let summoner1_id: i16 = stats.summoner1_id.try_into().unwrap_or(-1);
    let summoner2_casts: i16 = stats.summoner2_casts.try_into().unwrap();
    let summoner2_id: i16 = stats.summoner2_id.try_into().unwrap_or(-1);

    query!("
    INSERT INTO participant_spells (
        id, spell1_casts, spell2_casts, spell3_casts, spell4_casts,
        summoner1_casts, summoner1_id, summoner2_casts, summoner2_id
    ) VALUES (
        $1, $2, $3, $4, $5, $6, $7, $8, $9
    )",
    pg.id,
    spell1_casts,
    spell2_casts,
    spell3_casts,
    spell4_casts,
    summoner1_casts,
    summoner1_id,
    summoner2_casts,
    summoner2_id
    ).execute(&mut *tx).await.unwrap();

    



    if let Some(summoner_id) = summoner_id {
        for record_type in RecordType::iter() {
            let game_value = record_type.get_value(&stats, game_duration);
            let record_int = record_type.clone() as i16;
            let record = query_as!(RecordValue, "SELECT r.id, r.value FROM records r INNER JOIN matches m USING (game_id) WHERE r.record_type = $1 AND r.summoner_id = $2 AND m.queue_id = $3 AND m.season_id = $4",
                record_int,
                summoner_id,
                queue_id as i16,
                season_id).fetch_optional(&mut *tx).await;

            match record {
                Ok(Some(record_value)) => {
                    if record_value.value < game_value {
                        query!(
                            "UPDATE records SET value = $1, game_id = $2 WHERE id = $3",
                            game_value,
                            game_id,
                            record_value.id
                        )
                        .execute(&mut *tx)
                        .await
                        .unwrap();

                        let is_all_time_record: Result<i64, _> = query!(r#"
                            SELECT COUNT(*) as "count!"
                            FROM records r
                            INNER JOIN matches m USING (game_id)
                            WHERE
                                r.record_type = $1 AND
                                r.summoner_id = $2 AND
                                m.queue_id = $3 AND
                                r.value > $4
                            "#,
                            record_int,
                            summoner_id,
                            queue_id as i16,
                            game_value
                        ).fetch_one(&mut *tx).await.map(|record| record.count);

                        let summoner_name: String =
                            query!("SELECT name FROM summoners WHERE id = $1", summoner_id)
                                .fetch_one(&mut *tx)
                                .await
                                .unwrap()
                                .name;

                        if is_all_time_record.unwrap_or(0) == 0 {
                            send_connection_notification(
                                tx,
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
                                tx,
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
                    query!("
                        INSERT INTO records (game_id, summoner_id, record_type, value) VALUES ($1, $2, $3, $4)",
                        game_id,
                        summoner_id,
                        record_int,
                        game_value
                    ).execute(&mut *tx).await.unwrap();
                }
                Err(_) => {}
            };
        }
    }
}
