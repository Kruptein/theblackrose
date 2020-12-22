use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    db::Conn,
    models::{
        connections::{Connection, NewConnection},
        matches::{MatchFeedElement, MatchReference},
        summoners::Summoner,
        users::User,
    },
    routes::matches::MatchFilter,
    schema::connections::dsl::{connections, summoner_id, user_id},
    schema::match_references::dsl as mr,
    schema::summoners::dsl::{id as s_id, name, profile_icon_id, summoners},
};

use super::matches::get_game_info;

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

pub fn get_connection_matches(
    conn: &Conn,
    user: User,
    filter: MatchFilter,
) -> Result<Vec<MatchFeedElement>, Error> {
    let user_connections = match filter.get_names() {
        Some(names) => names.to_owned(),
        None => summoners
            .inner_join(connections)
            .filter(user_id.eq(user.id))
            .select(name)
            .get_results(conn)?,
    };
    let start_time = match filter.get_start_time() {
        Some(s) => s.to_owned(),
        None => i64::MAX,
    };

    let references: Vec<(MatchReference, Summoner)> = mr::match_references
        .inner_join(summoners)
        .filter(name.eq_any(user_connections))
        .filter(mr::timestamp.lt(start_time))
        .limit(10)
        .distinct_on(mr::timestamp)
        .order_by(mr::timestamp.desc())
        .get_results(conn)?;

    let mut match_collection: Vec<MatchFeedElement> = vec![];
    for reference in references {
        match_collection.push(get_game_info(reference.0.game_id, conn)?);
    }
    // println!("{}", serde_json::to_string(&match_collection).unwrap());
    Ok(match_collection)
}
