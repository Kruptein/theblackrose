use diesel::{insert_into, prelude::*, RunQueryDsl};

use crate::{
    db::Conn,
    models::notifications::NewNotification,
    schema::connections::dsl::{self as c},
    schema::notifications::dsl::{self as n},
    schema::users::dsl::{self as u},
};

pub fn send_connection_notification(conn: &Conn, summoner_id: i32, title: String, message: String) {
    let users: Vec<i32> = c::connections
        .inner_join(u::users)
        .select(u::id)
        .filter(c::summoner_id.eq(summoner_id))
        .get_results(conn)
        .unwrap();
    let mut values: Vec<NewNotification> = Vec::with_capacity(users.len());
    for user_id in users.into_iter() {
        let notification = NewNotification {
            user_id,
            title: title.clone(),
            message: message.clone(),
        };
        values.push(notification);
    }
    insert_into(n::notifications)
        .values(values)
        .execute(conn)
        .unwrap();
}
