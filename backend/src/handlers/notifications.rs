use diesel::{delete, insert_into, prelude::*, result::Error, RunQueryDsl};

use crate::{
    db::Conn,
    models::notifications::{NewNotification, Notification},
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

pub fn get_notifications(conn: &Conn, user_id: i32) -> Result<Vec<Notification>, Error> {
    n::notifications
        .filter(n::user_id.eq(user_id))
        .get_results(conn)
}

pub fn owns_notification(conn: &Conn, notification_id: i32, user_id: i32) -> Result<bool, Error> {
    let count: i64 = n::notifications
        .filter(n::id.eq(notification_id))
        .filter(n::user_id.eq(user_id))
        .count()
        .get_result(conn)?;
    Ok(count > 0)
}

pub fn remove_notification(conn: &Conn, notification_id: i32) -> Result<usize, Error> {
    delete(n::notifications.filter(n::id.eq(notification_id))).execute(conn)
}
