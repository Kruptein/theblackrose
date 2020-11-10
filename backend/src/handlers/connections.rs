use diesel::{insert_into, result::Error, RunQueryDsl};

use crate::{
    db::Conn,
    models::{
        connections::{Connection, NewConnection},
        summoners::Summoner,
        users::User,
    },
    schema::connections::dsl::*,
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
