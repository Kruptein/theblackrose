use std::env;

use actix_web::web;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};

use crate::{handlers::users::get_user_by_subj, AppState};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.")
}

// todo: check if we want to run this in a task
pub async fn register_subject(state: &AppState, token: &str, subject: String) {
    web::block(move || get_user_by_subj(state, subject)).await;
    // if let Err(_) = web::block(move || get_user_by_subj(state, subject)).await {
    //     let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    //     let url = format!("{}userinfo", authority);
    //     let client = reqwest::Client::new();
    //     let res = client.get(url.as_str()).bearer_auth(token);
    // };
}
