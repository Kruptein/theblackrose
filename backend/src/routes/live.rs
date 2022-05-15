use actix_web::{get, web, HttpResponse, Responder};
use riven::consts::PlatformRoute;

use crate::{handlers::live as l, rito::summoners::get_summoner_by_name, AppState};

#[get("/live/{summoner_name}/")]
pub async fn get_live_info(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.into_inner();

    let summoner = get_summoner_by_name(&riot_api, &db_pool, &username).await;
    if summoner.is_none() {
        return HttpResponse::NotFound().body("Summoner not found");
    }
    let summoner = summoner.unwrap();
    if summoner.summoner_id.is_none() {
        return HttpResponse::NotFound().body("Summoner found, but ID unknown");
    }

    let summoner_id = &summoner.summoner_id.unwrap();

    let live_game = riot_api
        .spectator_v4()
        .get_current_game_info_by_summoner(PlatformRoute::EUW1, summoner_id)
        .await;

    match live_game {
        Ok(data) => match data {
            Some(data) => {
                let info = l::get_live_info(db_pool, data).await;
                if info.is_err() {
                    println!("Win data collection failed: {:?}", info.unwrap_err());
                    return HttpResponse::InternalServerError().finish();
                }
                HttpResponse::Ok().json(&info.unwrap())
            }
            None => HttpResponse::NotFound().body("Summoner not in game at the moment."),
        },
        Err(e) => {
            println!("Get live game data error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
