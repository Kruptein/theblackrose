use actix_web::{error::BlockingError, web};
use diesel::result::Error;
use riven::{consts::Region, models::summoner_v4::Summoner as RiotSummoner, RiotApi};

use crate::models::summoners::Summoner;
use crate::{
    db::Pool,
    handlers::summoners::{self as h, add_summoner},
};

pub async fn get_summoner_by_name(
    api: &RiotApi,
    db_pool: &Pool,
    summoner_name: &str,
) -> Option<Summoner> {
    let conn = db_pool.get().unwrap();
    let result = get_summoner_name_from_model(db_pool, summoner_name).await;
    match result {
        Ok(x) => Some(x),
        Err(_) => match get_summoner_from_api(api, summoner_name).await {
            Some(summoner) => add_summoner(&conn, summoner).ok(),
            None => None,
        },
    }
}

async fn get_summoner_name_from_model(
    pool: &Pool,
    name: &str,
) -> Result<Summoner, BlockingError<Error>> {
    let db = pool.get().unwrap();
    let name = name.to_owned();
    web::block(move || h::get_summoner_by_name(&db, name.as_str())).await
}

async fn get_summoner_from_api(api: &RiotApi, name: &str) -> Option<RiotSummoner> {
    api.summoner_v4()
        .get_by_summoner_name(Region::EUW, name)
        .await
        .expect("Get summoner failed.")
}
