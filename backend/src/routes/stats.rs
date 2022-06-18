use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{auth::helpers::get_user_from_cache, db::connections::get_connections, AppState};

#[get("/stats/winrates/")]
pub async fn get_winrates(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let db_pool = &data.db_conn;
            let connections = get_connections(db_pool, user).await.unwrap();
            let winrates = data.winrate_map.read().await;
            let mut shit = HashMap::new();
            for (champion, data) in winrates.iter() {
                let mut champion_map = HashMap::new();
                for (summoner, data) in data.iter() {
                    if connections.iter().any(|s| &s.name == summoner) {
                        champion_map.insert(summoner, data);
                    }
                }
                shit.insert(champion, champion_map);
            }
            HttpResponse::Ok().json(shit)
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
