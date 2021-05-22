use riven::{consts::Region, models::summoner_v4::Summoner as RiotSummoner, RiotApi};
use sqlx::{Error, PgPool};

use crate::handlers::summoners::{self as h, add_summoner};
use crate::models::summoners::Summoner;

pub async fn get_summoner_by_name(
    api: &RiotApi,
    db_pool: &PgPool,
    summoner_name: &str,
) -> Option<Summoner> {
    let result = get_summoner_name_from_model(db_pool, summoner_name).await;
    match result {
        Ok(x) => Some(x),
        Err(_) => match get_summoner_from_api(api, summoner_name).await {
            Some(summoner) => add_summoner(db_pool, summoner).await.ok(),
            None => None,
        },
    }
}

async fn get_summoner_name_from_model(pool: &PgPool, name: &str) -> Result<Summoner, Error> {
    h::get_summoner_by_name(pool, name).await
}

async fn get_summoner_from_api(api: &RiotApi, name: &str) -> Option<RiotSummoner> {
    api.summoner_v4()
        .get_by_summoner_name(Region::EUW, name)
        .await
        .expect("Get summoner failed.")
}
