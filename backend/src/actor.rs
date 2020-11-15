use std::time::Duration;

use actix::{prelude::*, spawn};
use actix_web::web;
use chrono::TimeZone;
use diesel::Connection;
use riven::{consts::Region, models::match_v4::Match, models::match_v4::MatchReference, RiotApi};

use crate::{
    db::Pool,
    handlers::connections::get_connection_summoners,
    handlers::matches::add_match_details,
    handlers::matches::{add_match_reference, get_match_reference},
    handlers::summoners::{self as s, set_summoner_last_query_time},
    models::summoners::Summoner,
    rito,
    utils::millis_to_chrono,
};

pub struct GameFetchActor {
    pub db: Pool,
}

impl Actor for GameFetchActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started actor");
        update_closure(self, ctx);
        ctx.run_interval(Duration::from_secs(1 * 60 * 60), update_closure);
    }
}

fn update_closure(actor: &mut GameFetchActor, _: &mut Context<GameFetchActor>) {
    let db = actor.db.clone();
    spawn(async { update_matches(db).await })
}

async fn update_matches(db: Pool) {
    let api = rito::create_riot_api();
    match get_connection_summoners(&db.get().unwrap()) {
        Ok(summoners) => {
            let db = &db.clone();
            for summoner in summoners {
                // first update_summoner
                update_summoner(db, &api, &summoner).await;
                update_matches_for_summoner(db, &api, summoner).await;
            }
        }
        Err(_) => println!("Could not retrieve account ids from database!"),
    };
}

async fn update_summoner(db: &Pool, api: &RiotApi, summoner: &Summoner) {
    let summoner_id = summoner.id;
    match api
        .summoner_v4()
        .get_by_account_id(Region::EUW, summoner.account_id.as_str())
        .await
    {
        Ok(summoner) => {
            let conn = db.get().unwrap();
            web::block(move || s::update_summoner(&conn, summoner_id, summoner))
                .await
                .unwrap();
        }
        Err(_) => println!(
            "Could not update summoner data for account id: {} (username: {})",
            summoner.account_id, summoner.name
        ),
    }
}

async fn update_matches_for_summoner(db: &Pool, api: &RiotApi, summoner: Summoner) {
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

                    if handle_match(db, api, game).await {
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
        let conn = db.get().unwrap();
        let summoner_id = summoner.id.clone();
        web::block(move || {
            set_summoner_last_query_time(&conn, summoner_id, last_game_time.unwrap())
        })
        .await
        .unwrap();
    }
    println!(
        "Completed match update loop for {} [{} games added]",
        summoner.name, games_added
    );
}

async fn handle_match(db: &Pool, api: &RiotApi, match_reference: MatchReference) -> bool {
    let conn = db.get().unwrap();
    let game_id = match_reference.game_id.clone();
    match web::block(move || get_match_reference(&conn, game_id)).await {
        Ok(_) => false,
        Err(_) => {
            // Only add the match reference to the db if we could actually also fetch the details
            match get_match_details(api, match_reference.game_id).await {
                Some(match_details) => {
                    let conn = db.get().unwrap();
                    web::block(move || {
                        let c = &conn;
                        c.transaction(move || {
                            add_match_reference(c, match_reference)?;
                            add_match_details(c, match_details)
                        })
                    })
                    .await
                    .unwrap();
                    true
                }
                None => false,
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
