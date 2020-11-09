use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};
use riven::models::summoner_v4::Summoner as RiotSummoner;

use crate::{
    db::Conn,
    models::summoners::{NewSummoner, Summoner},
    schema::summoners::dsl::*,
};

pub fn get_summoner_by_name(conn: &Conn, summoner: &str) -> Result<Summoner, Error> {
    summoners.filter(name.eq(summoner)).get_result(conn)
}

pub fn add_summoner(conn: &Conn, summoner: RiotSummoner) -> Result<Summoner, Error> {
    let new_summoner = NewSummoner {
        account_id: summoner.account_id.as_str(),
        profile_icon_id: summoner.profile_icon_id,
        revision_date: summoner.revision_date,
        name: summoner.name.as_str(),
        summoner_id: summoner.id.as_str(),
        puuid: summoner.puuid.as_str(),
        summoner_level: summoner.summoner_level,
    };
    insert_into(summoners).values(new_summoner).get_result(conn)
}
