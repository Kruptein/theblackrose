mod actors;
mod auth;
mod db;
mod errors;
mod handlers;
mod models;
mod rito;
mod routes;
mod utils;

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
use actors::gamefetcher::GameFetchActor;
use auth::validation::validator;
use db::establish_pool;
use dotenv::dotenv;
use handlers::stats::ChampionWinrate;
use riven::RiotApi;
use routes::{
    connections::{add_connection, get_connections, refresh_connection},
    matches::get_matches,
    notifications::{get_notifications, remove_notification},
    records::get_records,
};
use sqlx::PgPool;
use tokio::sync::{Mutex, RwLock};

use crate::{
    actors::statscollector::StatsCollectorActor,
    routes::{live::get_live_info, stats::get_winrates, summoners::get_summoner},
};

pub struct AppState {
    db_conn: PgPool,
    riot_api: Arc<RiotApi>,
    tokens: RwLock<HashMap<String, i32>>,
    update_task: Addr<GameFetchActor>,
    stats_collector: Addr<StatsCollectorActor>,
    winrate_map: Arc<RwLock<HashMap<i16, HashMap<String, HashMap<i16, ChampionWinrate>>>>>,
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    actix_web::rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        let pool = establish_pool().await.unwrap();
        // tokio::task::spawn_local(system_fut);

        let riot_api = Arc::new(rito::create_riot_api());

        let game_fetcher = GameFetchActor::create(|_| GameFetchActor {
            db: pool.clone(),
            riot_api: riot_api.clone(),
            game_processing_lock: Arc::new(Mutex::new(HashSet::new())),
        })
        .clone();

        let winrate_map = Arc::new(RwLock::new(HashMap::new()));

        let stats_collector = StatsCollectorActor::create(|_| StatsCollectorActor {
            db: pool.clone(),
            modified_summoners: Arc::new(Mutex::new(HashSet::new())),
            winrate_map: winrate_map.clone(),
        });

        let web_data = web::Data::new(AppState {
            riot_api: riot_api.clone(),
            db_conn: pool.clone(),
            tokens: RwLock::new(HashMap::new()),
            update_task: game_fetcher.clone(),
            stats_collector,
            winrate_map,
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
                    scope("/api")
                        .wrap(auth)
                        .wrap(Cors::permissive())
                        .service(add_connection)
                        .service(get_connections)
                        .service(get_matches)
                        .service(get_records)
                        .service(get_summoner)
                        .service(get_notifications)
                        .service(remove_notification)
                        .service(refresh_connection)
                        .service(get_live_info)
                        .service(get_winrates),
                )
        })
        .bind("0.0.0.0:9000")
        .unwrap()
        .run()
        .await;
    });

    Ok(())
}
