use std::env;

use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::PooledConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.")
}
