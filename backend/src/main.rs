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

use std::{collections::HashMap, collections::HashSet, sync::Arc};

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    middleware::Logger,
    web::{self, scope},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use actor::GameFetchActor;
use auth::validation::validator;
use db::{establish_pool, Pool};
use dotenv::dotenv;
use riven::RiotApi;
use routes::{
    connections::{add_connection, get_connections, refresh_connection},
    matches::get_matches,
    records::get_records,
};
use tokio::sync::{Mutex, RwLock};

pub struct AppState {
    db_conn: Pool,
    riot_api: Arc<RiotApi>,
    tokens: RwLock<HashMap<String, i32>>,
    update_task: Addr<GameFetchActor>,
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut tok_runtime = tokio::runtime::Runtime::new().unwrap();
    let local_tasks = tokio::task::LocalSet::new();
    let system_fut = actix_web::rt::System::run_in_tokio("server", &local_tasks);

    local_tasks.block_on(&mut tok_runtime, async {
        tokio::task::spawn_local(system_fut);

        let pool = establish_pool();
        let riot_api = Arc::new(rito::create_riot_api());

        let actor = GameFetchActor::create(|_| GameFetchActor {
            db: pool.clone(),
            riot_api: riot_api.clone(),
            game_processing_lock: Arc::new(Mutex::new(HashSet::new())),
        })
        .clone();

        let web_data = web::Data::new(AppState {
            riot_api: riot_api.clone(),
            db_conn: pool.clone(),
            tokens: RwLock::new(HashMap::new()),
            update_task: actor.clone(),
        });

        println!("Started the black rose backend service!");

        let _ = HttpServer::new(move || {
            let auth = HttpAuthentication::bearer(validator);
            // let a = &actor.clone();

            App::new()
                .app_data(web_data.clone())
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .wrap(Cors::permissive())
                .service(fs::Files::new("/ddragon", "../ddragon").show_files_listing())
                .service(
                    scope("/api/")
                        .wrap(auth)
                        .wrap(Cors::permissive())
                        .service(add_connection)
                        .service(get_connections)
                        .service(get_matches)
                        .service(get_records)
                        .service(refresh_connection),
                )
        })
        .bind("0.0.0.0:9000")
        .unwrap()
        .run()
        .await;
    });

    Ok(())
}
