use std::time::Duration;

use actix::{prelude::*, spawn};
use actix_web::web;
use chrono::TimeZone;
use diesel::Connection as DieselConnection;
use riven::{consts::Region, models::match_v4::Match, models::match_v4::MatchReference, RiotApi};

use crate::{
    db::Pool,
    errors::UpdateError,
    handlers::connections::get_summoner,
    handlers::connections::get_unique_connections,
    handlers::matches::add_match_details,
    handlers::matches::{add_match_reference, get_match_reference},
    handlers::summoners::set_all_summoners_update_state,
    handlers::summoners::set_summoner_update_state,
    handlers::summoners::{self as s, set_summoner_last_query_time},
    models::connections::Connection,
    models::summoners::Summoner,
    rito,
    utils::millis_to_chrono,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectionUpdateMessage {
    pub connection: Connection,
}

pub struct GameFetchActor {
    pub db: Pool,
}

impl Actor for GameFetchActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started actor");
        // First make sure we reset any summoner update state that might not have properly be restored on shutdown
        let db = self.db.clone();
        spawn(async { unlock_all_summoners(db).await });
        // Check all connections
        update_closure(self, ctx);
        ctx.run_interval(Duration::from_secs(1 * 60 * 60), update_closure);
    }
}

async fn unlock_all_summoners(db: Pool) {
    let conn = db.get().unwrap();
    web::block(move || set_all_summoners_update_state(&conn, false))
        .await
        .unwrap();
}

fn update_closure(actor: &mut GameFetchActor, _: &mut Context<GameFetchActor>) {
    let db = actor.db.clone();
    spawn(async { update_connections(db).await })
}

impl Handler<ConnectionUpdateMessage> for GameFetchActor {
    type Result = ();

    fn handle(&mut self, msg: ConnectionUpdateMessage, _: &mut Context<GameFetchActor>) {
        println!("Got message");
        let d = self.db.clone();
        spawn(async { update_connection(d, &rito::create_riot_api(), msg.connection).await });
    }
}

async fn update_connections(db: Pool) {
    let api = rito::create_riot_api();
    match get_unique_connections(&db.get().unwrap()) {
        Ok(connections) => {
            let db = &db.clone();
            for connection in connections {
                update_connection(db.clone(), &api, connection).await;
            }
        }
        Err(_) => println!("Could not retrieve account ids from database!"),
    };
}

async fn update_connection(db: Pool, api: &RiotApi, connection: Connection) {
    let conn = db.get().unwrap();
    let summoner = web::block(move || get_summoner(&conn, connection))
        .await
        .unwrap();
    match summoner.update_in_progress {
        true => (),
        false => {
            let summoner_id = summoner.id;
            set_summoner_state(&db, summoner_id, true).await;
            let update_state: Result<usize, UpdateError> = try {
                update_summoner(&db, &api, &summoner).await?;
                update_matches_for_summoner(&db, api, summoner).await?
            };
            set_summoner_state(&db, summoner_id, false).await;
            update_state.unwrap();
        }
    }
}

async fn set_summoner_state(db: &Pool, summoner: i32, state: bool) {
    let conn = db.get().unwrap();
    web::block(move || set_summoner_update_state(&conn, summoner, state))
        .await
        .unwrap();
}

async fn update_summoner(
    db: &Pool,
    api: &RiotApi,
    summoner: &Summoner,
) -> Result<usize, UpdateError> {
    let summoner_id = summoner.id;
    match api
        .summoner_v4()
        .get_by_account_id(Region::EUW, summoner.account_id.as_str())
        .await
    {
        Ok(summoner) => {
            let conn = db.get().map_err(|e| UpdateError::PoolError(e))?;
            web::block(move || s::update_summoner(&conn, summoner_id, summoner))
                .await
                .map_err(|e| UpdateError::BlockError(e))
        }
        Err(e) => {
            println!(
                "Could not update summoner data for account id: {} (username: {})",
                summoner.account_id, summoner.name
            );
            Err(UpdateError::RiotError(e))
        }
    }
}

async fn update_matches_for_summoner(
    db: &Pool,
    api: &RiotApi,
    summoner: Summoner,
) -> Result<usize, UpdateError> {
    let begin_time = chrono::Utc.ymd(2000, 1, 1).and_hms(1, 1, 1).naive_utc();
    let begin_time = summoner.last_match_query_time.unwrap_or(begin_time);
    let mut begin_index = 0;
    let mut games_added = 0;
    let mut last_game_time: Option<chrono::NaiveDateTime> = None;

    println!("Starting match update loop for {}", summoner.name);

    'outer: loop {
        match api
            .match_v4()
            .get_matchlist(
                Region::EUW,
                summoner.account_id.as_str(),
                None,
                Some(begin_index),
                None,
                None,
                Some(begin_index + 100),
                None,
                None,
            )
            .await
        {
            Ok(Some(games)) => {
                let length = games.matches.len();
                let batch_last_game_time =
                    games.matches.get(0).map(|g| millis_to_chrono(g.timestamp));
                last_game_time = last_game_time.or(batch_last_game_time);
                println!("{:?} ({} games)", batch_last_game_time, length);
                for game in games.matches {
                    let naive_timestamp = millis_to_chrono(game.timestamp);

                    if naive_timestamp.le(&begin_time) {
                        break 'outer;
                    }

                    if handle_match(db, api, game, summoner.id).await? {
                        games_added += 1;
                    }
                }
                if length < 100 {
                    break;
                }
            }
            Ok(None) => break,
            Err(e) => {
                println!("MATCH RETRIEVAL FAILED ({:?})", e);
                break;
            }
        }
        begin_index += 100;
    }
    if last_game_time.is_some() {
        let conn = db.get().map_err(|e| UpdateError::PoolError(e))?;
        let summoner_id = summoner.id.clone();
        web::block(move || {
            set_summoner_last_query_time(&conn, summoner_id, last_game_time.unwrap())
        })
        .await
        .map_err(|e| UpdateError::BlockError(e))?;
    }
    println!(
        "Completed match update loop for {} [{} games added]",
        summoner.name, games_added
    );
    Ok(games_added)
}

async fn handle_match(
    db: &Pool,
    api: &RiotApi,
    match_reference: MatchReference,
    summoner_id: i32,
) -> Result<bool, UpdateError> {
    let conn = db.get().map_err(|e| UpdateError::PoolError(e))?;
    let game_id = match_reference.game_id.clone();
    match web::block(move || get_match_reference(&conn, game_id, summoner_id)).await {
        Ok(_) => Ok(false),
        Err(_) => {
            // Only add the match reference to the db if we could actually also fetch the details
            match get_match_details(api, match_reference.game_id).await {
                Some(match_details) => {
                    let conn = db.get().map_err(|e| UpdateError::PoolError(e))?;
                    web::block(move || {
                        let c = &conn;
                        c.transaction(move || {
                            add_match_reference(c, match_reference, summoner_id)?;
                            add_match_details(c, match_details)
                        })
                    })
                    .await
                    .map_err(|e| UpdateError::BlockError(e))?;
                    Ok(true)
                }
                None => Ok(false),
            }
        }
    }
}

async fn get_match_details(api: &RiotApi, game_id: i64) -> Option<Match> {
    match api.match_v4().get_match(Region::EUW, game_id).await {
        Ok(result) => result,
        Err(_) => None,
    }
}
