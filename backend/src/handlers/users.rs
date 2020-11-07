use actix_web::web;
use diesel::{delete, insert_into, result::Error, QueryDsl, RunQueryDsl};

use crate::{
    models::{InputUser, NewUser, User},
    schema::users::dsl::users,
    AppState,
};

pub fn get_all_users(state: web::Data<AppState>) -> Result<Vec<User>, Error> {
    let conn = state.db_conn.get().unwrap();
    users.load::<User>(&conn)
}

pub fn get_user_by_id(state: web::Data<AppState>, user_id: web::Path<i32>) -> Result<User, Error> {
    let conn = state.db_conn.get().unwrap();
    users.find(user_id.0).get_result::<User>(&conn)
}

pub fn add_user(state: web::Data<AppState>, item: web::Json<InputUser>) -> Result<User, Error> {
    let conn = state.db_conn.get().unwrap();
    let new_user = NewUser {
        username: &item.username,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
    };
    insert_into(users)
        .values(&new_user)
        .get_result::<User>(&conn)
}

pub fn delete_user(state: web::Data<AppState>, user_id: web::Path<i32>) -> Result<usize, Error> {
    let conn = state.db_conn.get().unwrap();
    delete(users.find(user_id.0)).execute(&conn)
}
