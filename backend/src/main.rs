mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod rito;
mod routes;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use std::collections::HashMap;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::validation::validator;
use db::{establish_pool, Pool};
use dotenv::dotenv;
use riven::RiotApi;
use tokio::sync::RwLock;

pub struct AppState<'a> {
    db_conn: Pool,
    riot_api: RiotApi,
    subjects: RwLock<HashMap<&'a str, i32>>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    let pool = establish_pool();

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(AppState {
                riot_api: rito::create_riot_api(),
                db_conn: pool.clone(),
                subjects: RwLock::new(HashMap::new()),
            })
            .wrap(auth)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await?;
    sys.await?;
    Ok(server)
}
