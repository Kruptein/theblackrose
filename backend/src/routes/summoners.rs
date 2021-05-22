use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    handlers::summoners::{get_summoner_quick_stats, QuickStats},
    models::summoners::Summoner,
    rito::summoners::get_summoner_by_name,
    AppState,
};

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct SummonerInfo {
    core: Summoner,
    quick_stats: Option<QuickStats>,
}

#[get("/summoners/{summoner_name}/")]
pub async fn get_summoner(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.into_inner();

    let summoner = get_summoner_by_name(&riot_api, &db_pool, &username).await;
    match summoner {
        Some(summoner) => {
            let quick_stats = get_summoner_quick_stats(&db_pool, &summoner).await;
            let info = SummonerInfo {
                core: summoner,
                quick_stats: quick_stats.ok(),
            };
            match serde_json::to_string(&info) {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        None => HttpResponse::NotFound().finish(),
    }
}
