use actix_web::{post, web, HttpResponse, Responder};

use crate::{handlers::connections as h, rito::summoners::get_summoner_by_name, AppState};

#[post("/connections/{summoner_name}")]
pub async fn add_connection(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.0;

    let summoner = get_summoner_by_name(riot_api, db_pool, username.as_str()).await;

    if summoner.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let db = db_pool.get().unwrap();
    // h::add_connection(&db, user, summoner.unwrap());

    HttpResponse::Ok().finish()
}
