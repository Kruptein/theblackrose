use actix_web::web;
use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    db::Conn,
    models::{
        connections::{Connection, NewConnection},
        matches::{
            Match, MatchReference, Participant, ParticipantStatsGeneral, ParticipantStatsKills,
        },
        summoners::Summoner,
        users::User,
    },
    routes::matches::Filter,
    schema::connections::dsl::{connections, summoner_id, user_id},
    schema::match_references::dsl as mr,
    schema::matches::dsl::{self as m, matches},
    schema::participant_stats_general::dsl as psg,
    schema::participant_stats_kills::dsl as psk,
    schema::participants::dsl as p,
    schema::summoners::dsl::{id as s_id, name, profile_icon_id, summoners},
};

pub fn add_connection(conn: &Conn, user: User, summoner: Summoner) -> Result<Connection, Error> {
    let new_connection = NewConnection {
        summoner_id: summoner.id,
        user_id: user.id,
    };
    insert_into(connections)
        .values(new_connection)
        .get_result(conn)
}

pub fn get_unique_connections(conn: &Conn) -> Result<Vec<Connection>, Error> {
    connections.distinct_on(summoner_id).get_results(conn)
}

pub fn get_summoner(conn: &Conn, connection: Connection) -> Result<Summoner, Error> {
    summoners
        .filter(s_id.eq(connection.summoner_id))
        .get_result(conn)
}

pub fn get_connection_short_info(conn: &Conn, user: User) -> Result<Vec<(String, i32)>, Error> {
    connections
        .inner_join(summoners)
        .select((name, profile_icon_id))
        .filter(user_id.eq(user.id))
        .get_results(conn)
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct MatchFeedParticipant {
    participant: Participant,
    summoner: Summoner,
    general: ParticipantStatsGeneral,
    kills: ParticipantStatsKills,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MatchFeedElement {
    match_info: Match,
    participants: Vec<MatchFeedParticipant>,
}

pub fn get_connection_matches(
    conn: &Conn,
    user: User,
    filter: web::Query<Filter>,
) -> Result<Vec<MatchFeedElement>, Error> {
    let user_connections = match filter.0.get_names() {
        Some(names) => names,
        None => summoners
            .inner_join(connections)
            .filter(user_id.eq(user.id))
            .select(name)
            .get_results(conn)?,
    };

    let references: Vec<(MatchReference, Summoner)> = mr::match_references
        .inner_join(summoners)
        .filter(name.eq_any(user_connections))
        .limit(10)
        .distinct_on(mr::timestamp)
        .order_by(mr::timestamp.desc())
        .get_results(conn)?;

    let mut match_collection: Vec<MatchFeedElement> = vec![];
    for reference in references {
        let match_info: Match = matches
            .filter(m::game_id.eq(reference.0.game_id))
            .get_result(conn)?;
        let participants: Vec<(
            Participant,
            ParticipantStatsGeneral,
            ParticipantStatsKills,
            Summoner,
        )> = p::participants
            .inner_join(psg::participant_stats_general)
            .inner_join(psk::participant_stats_kills)
            .inner_join(summoners)
            .filter(p::game_id.eq(reference.0.game_id))
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
        match_collection.push(MatchFeedElement {
            match_info,
            participants,
        });
        // println!("{}", serde_json::to_string(&match_collection).unwrap());
    }
    Ok(match_collection)
}
