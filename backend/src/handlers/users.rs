use actix_web::web;
use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    auth::auth0::{get_user_info, UserInfo},
    db::Conn,
    db::Pool,
    models::users::NewUser,
    models::users::User,
    schema::users::dsl::*,
};

fn get_user_by_subj(conn: Conn, subject: String) -> Result<User, Error> {
    users.filter(auth0_subject.eq(subject)).get_result(&conn)
}

fn add_user(conn: Conn, user: &UserInfo) -> Result<usize, Error> {
    let new_user = NewUser {
        email: user.email.as_str(),
        auth0_subject: user.sub.as_str(),
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users).values(&new_user).execute(&conn)
}

// todo: check if we want to run this in a task
pub async fn register_subject(db_pool: Pool, token: &str, subject: String) {
    let db_conn = db_pool.clone().get().unwrap();
    let subj_clone = subject.clone();
    if let Err(_) = web::block(move || get_user_by_subj(db_conn, subj_clone)).await {
        let data = get_user_info(token).await.unwrap();
        add_user(db_pool.get().unwrap(), &data).unwrap();
        println!("Added user with email {}", data.email);
    };
    println!("Welcome user with subject {}", subject);
}
