mod actor;
mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod rito;
mod routes;
mod schema;
mod utils;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use std::collections::HashMap;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use actor::GameFetchActor;
use auth::validation::validator;
use db::{establish_pool, Pool};
use dotenv::dotenv;
use riven::RiotApi;
use routes::connections::{add_connection, get_connections};
use tokio::sync::RwLock;

pub struct AppState {
    db_conn: Pool,
    riot_api: RiotApi,
    tokens: RwLock<HashMap<String, i32>>,
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut tok_runtime = tokio::runtime::Runtime::new().unwrap();
    let local_tasks = tokio::task::LocalSet::new();
    let system_fut = actix_web::rt::System::run_in_tokio("server", &local_tasks);

    local_tasks.block_on(&mut tok_runtime, async {
        tokio::task::spawn_local(system_fut);

        let pool = establish_pool();

        GameFetchActor::create(|_| GameFetchActor { db: pool.clone() });

        println!("Started the black rose backend service!");

        let _ = HttpServer::new(move || {
            let auth = HttpAuthentication::bearer(validator);

            App::new()
                .data(AppState {
                    riot_api: rito::create_riot_api(),
                    db_conn: pool.clone(),
                    tokens: RwLock::new(HashMap::new()),
                })
                .wrap(auth)
                .wrap(Cors::permissive())
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(add_connection)
                .service(get_connections)
        })
        .bind("0.0.0.0:9000")
        .unwrap()
        .run()
        .await;
    });

    Ok(())
}
