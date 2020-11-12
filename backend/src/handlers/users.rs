use diesel::{insert_into, prelude::*, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    auth::auth0::UserInfo, db::Conn, models::users::NewUser, models::users::User,
    schema::users::dsl::*,
};

pub fn get_user_by_subj(conn: Conn, subject: &str) -> Result<User, Error> {
    users.filter(auth0_subject.eq(subject)).get_result(&conn)
}

pub fn get_user_by_id(conn: &Conn, user: i32) -> Result<User, Error> {
    users.filter(id.eq(user)).get_result(conn)
}

pub fn add_user(conn: Conn, user: &UserInfo) -> Result<User, Error> {
    let new_user = NewUser {
        email: user.email.as_str(),
        auth0_subject: user.sub.as_str(),
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users).values(&new_user).get_result(&conn)
}
