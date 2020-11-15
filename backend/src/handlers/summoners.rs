use diesel::{insert_into, prelude::*, result::Error, update, QueryDsl, RunQueryDsl};
use riven::models::match_v4 as RM;
use riven::models::summoner_v4 as RS;

use crate::{
    db::Conn,
    models::summoners::{NewSummoner, Summoner},
    schema::summoners::dsl::{
        account_id, last_match_query_time, name, profile_icon_id, revision_date, summoner_level,
        summoners, update_in_progress,
    },
};

pub fn get_summoner_by_name(conn: &Conn, summoner: &str) -> Result<Summoner, Error> {
    summoners.filter(name.eq(summoner)).get_result(conn)
}

pub fn add_summoner(conn: &Conn, summoner: RS::Summoner) -> Result<Summoner, Error> {
    let new_summoner = NewSummoner {
        account_id: summoner.account_id.as_str(),
        profile_icon_id: summoner.profile_icon_id,
        revision_date: Some(summoner.revision_date),
        name: summoner.name.as_str(),
        summoner_id: Some(summoner.id.as_str()),
        puuid: Some(summoner.puuid.as_str()),
        summoner_level: Some(summoner.summoner_level),
    };
    insert_into(summoners).values(new_summoner).get_result(conn)
}

pub fn update_summoner(conn: &Conn, id: i32, summoner: RS::Summoner) -> Result<usize, Error> {
    update(summoners.find(id))
        .set((
            profile_icon_id.eq(summoner.profile_icon_id),
            name.eq(summoner.name),
            revision_date.eq(summoner.revision_date),
            summoner_level.eq(summoner.summoner_level),
        ))
        .execute(conn)
}

pub fn get_or_add_partial_summoner(conn: &Conn, summoner: RM::Player) -> Result<Summoner, Error> {
    let acc_id = summoner.account_id.clone();
    match summoners.filter(account_id.eq(acc_id)).get_result(conn) {
        Ok(summoner) => Ok(summoner),
        Err(_) => add_partial_summoner(conn, summoner),
    }
}

fn add_partial_summoner(conn: &Conn, summoner: RM::Player) -> Result<Summoner, Error> {
    let new_summoner = NewSummoner {
        account_id: summoner.account_id.as_str(),
        profile_icon_id: summoner.profile_icon,
        name: summoner.summoner_name.as_str(),
        summoner_id: summoner.summoner_id.as_deref(),
        revision_date: None,
        puuid: None,
        summoner_level: None,
    };
    insert_into(summoners).values(new_summoner).get_result(conn)
}

pub fn set_summoner_last_query_time(
    conn: &Conn,
    summoner: i32,
    time: chrono::NaiveDateTime,
) -> Result<usize, Error> {
    update(summoners.find(summoner))
        .set(last_match_query_time.eq(time))
        .execute(conn)
}

pub fn set_summoner_update_state(conn: &Conn, summoner: i32, state: bool) -> Result<usize, Error> {
    update(summoners.find(summoner))
        .set(update_in_progress.eq(state))
        .execute(conn)
}

pub fn set_all_summoners_update_state(conn: &Conn, state: bool) -> Result<usize, Error> {
    update(summoners)
        .set(update_in_progress.eq(state))
        .execute(conn)
}
