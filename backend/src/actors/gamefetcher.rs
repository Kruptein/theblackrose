use std::{collections::HashSet, env, sync::Arc, time::Duration};

use actix::{prelude::*, spawn};
use chrono::TimeZone;
use riven::{consts::RegionalRoute, models::match_v5::Match, RiotApi};
use sqlx::{Error, PgPool};
use tokio::sync::Mutex;

use crate::{
    handlers::connections::get_summoner,
    handlers::connections::get_unique_connections,
    handlers::matches as m,
    handlers::{
        matches::{get_match_info, has_game},
        summoners as s,
    },
    models::connections::Connection,
    models::summoners::Summoner,
    rito::summoners::update_summoner,
    utils::{millis_to_chrono, SlidingWindow},
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectionUpdateMessage {
    pub connection: Connection,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SummonerUpdateMessage {
    pub summoner: Summoner,
}

pub struct GameFetchActor {
    pub db: PgPool,
    pub riot_api: Arc<RiotApi>,
    pub game_processing_lock: Arc<Mutex<HashSet<i64>>>,
}

impl Actor for GameFetchActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started actor");
        // First make sure we reset any summoner update state that might not have properly be restored on shutdown
        let db = self.db.clone();
        spawn(async { unlock_all_summoners(db).await });

        // Check all connections
        let skip: bool = env::var("SKIP_START_LOOP")
            .unwrap_or("false".to_owned())
            .parse()
            .unwrap_or(false);
        if !skip {
            update_closure(self, ctx);
        }
        ctx.run_interval(Duration::from_secs(1 * 60 * 60), update_closure);
    }
}

async fn unlock_all_summoners(db: PgPool) {
    s::set_all_summoners_update_state(&db, false).await.unwrap();
}

fn update_closure(actor: &mut GameFetchActor, _: &mut Context<GameFetchActor>) {
    let db = actor.db.clone();
    let riot_api = actor.riot_api.clone();
    let game_processing_lock = actor.game_processing_lock.clone();
    spawn(async { update_connections(db, riot_api, game_processing_lock).await });
}

impl Handler<ConnectionUpdateMessage> for GameFetchActor {
    type Result = ();

    fn handle(&mut self, msg: ConnectionUpdateMessage, _: &mut Context<GameFetchActor>) {
        let db = self.db.clone();
        let api = self.riot_api.clone();
        let game_processing_lock = self.game_processing_lock.clone();
        spawn(async { update_connection(db, api, game_processing_lock, msg.connection).await });
    }
}

impl Handler<SummonerUpdateMessage> for GameFetchActor {
    type Result = ();

    fn handle(&mut self, msg: SummonerUpdateMessage, _: &mut Context<GameFetchActor>) {
        let db = self.db.clone();
        let api = self.riot_api.clone();
        let game_processing_lock = self.game_processing_lock.clone();
        spawn(async {
            update_summoner_with_lock(db, msg.summoner, api, game_processing_lock).await
        });
    }
}

async fn update_connections(
    db: PgPool,
    api: Arc<RiotApi>,
    game_processing_lock: Arc<Mutex<HashSet<i64>>>,
) {
    match get_unique_connections(&db).await {
        Ok(connections) => {
            for connection in connections {
                update_connection(
                    db.clone(),
                    api.clone(),
                    game_processing_lock.clone(),
                    connection,
                )
                .await;
            }
        }
        Err(_) => println!("Could not retrieve account ids from database!"),
    };
}

async fn update_connection(
    db: PgPool,
    api: Arc<RiotApi>,
    game_processing_lock: Arc<Mutex<HashSet<i64>>>,
    connection: Connection,
) {
    let summoner = get_summoner(&db, connection).await.unwrap();
    match summoner.update_in_progress {
        true => (),
        false => {
            update_summoner_with_lock(db, summoner, api, game_processing_lock).await;
        }
    }
}

async fn update_summoner_with_lock(
    db: PgPool,
    summoner: Summoner,
    api: Arc<RiotApi>,
    game_processing_lock: Arc<Mutex<HashSet<i64>>>,
) {
    let summoner_id = summoner.id;
    set_summoner_state(&db, summoner_id, true).await;
    update_summoner(&api, &db, &summoner).await.unwrap();
    update_matches_for_summoner(&db, &api, &game_processing_lock, summoner).await;
    set_summoner_state(&db, summoner_id, false).await;
}

async fn set_summoner_state(db: &PgPool, summoner: i32, state: bool) {
    s::set_summoner_update_state(db, summoner, state)
        .await
        .unwrap();
}

async fn update_matches_for_summoner(
    db: &PgPool,
    api: &RiotApi,
    game_processing_lock: &Arc<Mutex<HashSet<i64>>>,
    summoner: Summoner,
) {
    let begin_time = chrono::Utc.ymd(2000, 1, 1).and_hms(1, 1, 1).naive_utc();
    let begin_time = summoner.last_match_query_time.unwrap_or(begin_time);
    let mut begin_index = 0;
    let mut games_added = 0;

    let mut last_game_time: Option<chrono::NaiveDateTime> = None;

    let mut sliding_window = SlidingWindow::new();

    println!("Starting match update loop for {}", summoner.name);

    'outer: loop {
        match api
            .match_v5()
            .get_match_ids_by_puuid(
                RegionalRoute::EUROPE,
                summoner.puuid.as_ref().unwrap().as_str(),
                Some(100),
                None,
                None,
                None,
                Some(begin_index),
                None,
            )
            .await
        {
            Ok(games) => {
                for game in games.iter() {
                    let game_split = game.split("_").collect::<Vec<&str>>();
                    let platform_id = game_split.get(0).unwrap().to_owned();
                    let game_id: i64 = game_split.get(1).unwrap().parse().unwrap();

                    if let Ok(true) = has_game(db, platform_id, game_id).await {
                        let info = get_match_info(db, game_id).await;
                        let game_time = millis_to_chrono(info.unwrap().game_creation);
                        if last_game_time.is_none() {
                            last_game_time = Some(game_time);
                        }
                        if game_time.le(&begin_time) {
                            break 'outer;
                        }
                        continue;
                    }

                    match api
                        .match_v5()
                        .get_match(RegionalRoute::EUROPE, game.as_str())
                        .await
                    {
                        Ok(Some(details)) => {
                            let game_time = millis_to_chrono(details.info.game_creation);

                            if game_time.le(&begin_time) {
                                break 'outer;
                            }

                            if last_game_time.is_none() {
                                last_game_time = Some(game_time);
                            }

                            let mut lock = game_processing_lock.lock().await;
                            let has_game = lock.contains(&details.info.game_id);
                            if !has_game {
                                lock.insert(details.info.game_id);
                            }
                            drop(lock);

                            if !has_game {
                                if add_game_details(db, &details, &mut sliding_window).await {
                                    games_added += 1;
                                }
                            }
                        }
                        Ok(None) => {
                            sliding_window.clear();
                            println!("MATCH NOT FOUND ({:?})", game)
                        }
                        Err(e) => {
                            sliding_window.clear();
                            println!("MATCH RETRIEVAL FAILED ({:?})", e);
                            break;
                        }
                    }
                }
                if games.len() < 100 {
                    break;
                }
            }
            Err(e) => {
                sliding_window.clear();
                println!("MATCHLIST RETRIEVAL FAILED ({:?})", e);
                break;
            }
        }
        begin_index += 100;
    }
    if last_game_time.is_some() && begin_time.lt(&last_game_time.unwrap()) {
        let summoner_id = summoner.id.clone();

        s::set_summoner_last_query_time(db, summoner_id, last_game_time.unwrap())
            .await
            .unwrap();
    }
    sliding_window.clear();
    println!(
        "Completed match update loop for {} [{} games added]",
        summoner.name, games_added
    );
}

async fn add_game_details(
    db: &PgPool,
    details: &Match,
    sliding_window: &mut SlidingWindow,
) -> bool {
    let game_id = details.info.game_id;
    match m::get_match_info(db, game_id).await {
        Ok(_) => false,
        Err(Error::RowNotFound) => {
            let game_time = millis_to_chrono(details.info.game_creation);
            sliding_window.push(format!("Adding game {} [{:?}]", game_id, game_time));
            m::add_match_details(db, details).await.unwrap();
            sliding_window.replace(format!("Added game {} [{:?}]", game_id, game_time));
            true
        }
        Err(e) => {
            sliding_window.clear();
            println!("ADD GAME DETAILS ERROR: {:?}", e);
            false
        }
    }
}
