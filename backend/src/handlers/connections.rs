use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    db::Conn,
    models::{
        connections::{Connection, NewConnection},
        summoners::Summoner,
        users::User,
    },
    schema::connections::dsl::{connections, user_id},
    schema::summoners::dsl::{name, summoners},
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

pub fn get_connection_names(conn: &Conn, user: User) -> Result<Vec<String>, Error> {
    connections
        .inner_join(summoners)
        .select(name)
        .filter(user_id.eq(user.id))
        .get_results(conn)
}
