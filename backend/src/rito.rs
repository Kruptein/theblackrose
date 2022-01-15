pub mod summoners;

use riven::RiotApi;
use std::env;

pub fn create_riot_api() -> RiotApi {
    let api_key = env::var("RGAPI_KEY").expect("RGAPI_KEY is not set as an environment variable!");
    RiotApi::new(api_key)
}
