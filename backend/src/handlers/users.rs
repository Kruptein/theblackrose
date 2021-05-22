use sqlx::{Error, PgPool};

use crate::{auth::auth0::UserInfo, models::users::User};

pub async fn get_user_by_subj(conn: &PgPool, subject: &str) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE auth0_subject = $1",
        subject
    )
    .fetch_one(conn)
    .await
}

pub async fn get_user_by_id(conn: &PgPool, user: i32) -> Result<User, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user)
        .fetch_one(conn)
        .await
}

pub async fn add_user(conn: &PgPool, user: &UserInfo) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (email, auth0_subject, created_at) VALUES ($1, $2, $3) RETURNING *",
        user.email.as_str(),
        user.sub.as_str(),
        chrono::Local::now().naive_local()
    )
    .fetch_one(conn)
    .await
}
