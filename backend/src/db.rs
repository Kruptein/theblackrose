use std::env;

use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, Error};

pub async fn establish_pool() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(PgPoolOptions::new().connect(database_url.as_str()).await?)
}
