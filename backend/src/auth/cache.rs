use std::collections::HashMap;

use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::db::users::{add_user, get_user_by_subj};

use super::auth0::get_user_info;

// todo: check if we want to run this in a task
pub async fn update_token_cache(
    db_pool: &PgPool,
    tokens: &RwLock<HashMap<String, i32>>,
    token: &str,
    subject: &str,
) {
    let has_subject = {
        let subjects = tokens.read().await;
        subjects.get(subject).map(i32::to_owned)
    };
    match has_subject {
        Some(_) => (),
        None => {
            add_token_to_cache(
                tokens,
                token,
                get_or_create_subject(db_pool, token, subject).await,
            )
            .await
        }
    }
}

async fn add_token_to_cache(tokens: &RwLock<HashMap<String, i32>>, token: &str, user: i32) {
    let mut tokens = tokens.write().await;
    tokens.insert(token.to_owned(), user);
}

async fn get_or_create_subject(db_pool: &PgPool, token: &str, subject: &str) -> i32 {
    match load_subject_from_db(db_pool, subject).await {
        Some(user) => user,
        None => add_user_to_db(db_pool, token).await,
    }
}

async fn load_subject_from_db(db_pool: &PgPool, subject: &str) -> Option<i32> {
    let subj_clone = subject.to_owned();
    match get_user_by_subj(db_pool, subj_clone.as_str()).await {
        Ok(user) => Some(user.id),
        Err(_) => None,
    }
}

async fn add_user_to_db(db_pool: &PgPool, token: &str) -> i32 {
    let data = get_user_info(token).await.unwrap();
    add_user(db_pool, &data).await.unwrap().id
}
