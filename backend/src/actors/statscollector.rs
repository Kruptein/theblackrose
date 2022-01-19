use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use actix::{spawn, Actor, Context};
use sqlx::PgPool;
use tokio::sync::{Mutex, RwLock};

use crate::{
    db::connections::get_unique_connections,
    handlers::stats::{get_all_winrates, ChampionWinrate},
};

type WinrateMap = Arc<RwLock<HashMap<i16, HashMap<String, HashMap<i16, ChampionWinrate>>>>>;

pub struct StatsCollectorActor {
    pub db: PgPool,
    pub modified_summoners: Arc<Mutex<HashSet<i32>>>,
    pub winrate_map: WinrateMap,
}

impl Actor for StatsCollectorActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        update_closure(self, ctx);
        // ctx.run_interval(Duration::from_secs(5), update_closure);
    }
}

fn update_closure(actor: &mut StatsCollectorActor, _: &mut Context<StatsCollectorActor>) {
    let db = actor.db.clone();
    let a = actor.modified_summoners.clone();
    let winrates = actor.winrate_map.clone();
    spawn(async { update_connections(db, a, winrates).await });
}

async fn update_connections(
    db: PgPool,
    _summoners: Arc<Mutex<HashSet<i32>>>,
    winrates: WinrateMap,
) {
    // let summoners = summoners.lock().await;
    let mut summoners = HashSet::new();
    for connection in get_unique_connections(&db).await.unwrap() {
        summoners.insert(connection.summoner_id);
    }

    let mut collection = winrates.write().await;

    for summoner in summoners.into_iter() {
        let data = get_all_winrates(&db, summoner).await;
        if data.is_err() {
            panic!("RIP");
        }
        let data = data.unwrap();
        for (k, v) in data.into_iter() {
            match collection.get_mut(&k) {
                Some(champion_data) => {
                    for (k, v) in v.into_iter() {
                        match champion_data.get_mut(&k) {
                            Some(summoner_data) => {
                                for (k, v) in v.into_iter() {
                                    match summoner_data.get_mut(&k) {
                                        Some(queue_data) => {
                                            queue_data.join(v);
                                        }
                                        None => {
                                            summoner_data.insert(k, v);
                                        }
                                    }
                                }
                            }
                            None => {
                                champion_data.insert(k, v);
                            }
                        }
                    }
                }
                None => {
                    collection.insert(k, v);
                }
            }
        }
    }
}
