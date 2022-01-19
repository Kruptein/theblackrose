use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    db::summoners::{get_summoner_quick_stats, QuickStats},
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

#[derive(Debug, Deserialize)]
pub struct SummonerQuery {
    stats: Option<bool>,
}

#[get("/summoners/{summoner_name}/")]
pub async fn get_summoner(
    data: web::Data<AppState>,
    path: web::Path<String>,
    query: web::Query<SummonerQuery>,
) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.into_inner();
    let query = query.into_inner();

    let summoner = get_summoner_by_name(&riot_api, &db_pool, &username).await;
    match summoner {
        Some(summoner) => {
            let mut quick_stats: Option<QuickStats> = None;
            if query.stats.unwrap_or(false) {
                quick_stats = get_summoner_quick_stats(&db_pool, &summoner).await.ok();
            }
            let info = SummonerInfo {
                core: summoner,
                quick_stats,
            };
            HttpResponse::Ok().json(info)
        }
        None => HttpResponse::NotFound().finish(),
    }
}
