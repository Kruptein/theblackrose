use sqlx::{query, query_as, Error, PgPool, Postgres, Transaction};

use crate::models::notifications::Notification;

pub async fn send_connection_notification(
    tx: &mut Transaction<'_, Postgres>,
    summoner_id: i32,
    title: String,
    message: String,
) {
    for user_id in query!(
        "
        SELECT u.id
        FROM users u
        INNER JOIN connections c ON c.user_id = u.id
        WHERE c.summoner_id = $1",
        summoner_id
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap()
    .into_iter()
    .map(|record| record.id)
    {
        query!(
            "INSERT INTO notifications (user_id, title, message) VALUES ($1, $2, $3)",
            user_id,
            title.clone(),
            message.clone(),
        )
        .execute(&mut *tx)
        .await
        .unwrap();
    }
}

pub async fn get_notifications(conn: &PgPool, user_id: i32) -> Result<Vec<Notification>, Error> {
    query_as!(
        Notification,
        "SELECT * FROM notifications WHERE user_id = $1",
        user_id
    )
    .fetch_all(conn)
    .await
}

pub async fn owns_notification(
    conn: &PgPool,
    notification_id: i32,
    user_id: i32,
) -> Result<bool, Error> {
    let count = query!(
        r#"SELECT COUNT(*) as "count!" FROM notifications WHERE id = $1 AND user_id = $2"#,
        notification_id,
        user_id
    )
    .fetch_one(conn)
    .await?
    .count;
    Ok(count > 0)
}

pub async fn remove_notification(conn: &PgPool, notification_id: i32) -> Result<u64, Error> {
    query!("DELETE FROM notifications WHERE id = $1", notification_id)
        .execute(conn)
        .await
        .map(|result| result.rows_affected())
}
