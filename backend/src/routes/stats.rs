use std::collections::HashMap;

use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::AppState;

#[get("/stats/winrates/")]
pub async fn get_winrates(data: web::Data<AppState>, _auth: BearerAuth) -> impl Responder {
    // match get_user_from_cache(&data.tokens, auth.token()).await {
    // Some(user) => {
    // let user = get_user_by_id(&db_pool, user).await.unwrap();
    let winrates = data.winrate_map.read().await;
    let mut shit = HashMap::new();
    winrates.iter().for_each(|(k, v)| {
        shit.insert(k, v);
    });
    match serde_json::to_string(&shit) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
    // match (db_pool, user, query.0).await {
    //     Ok(records) => match serde_json::to_string(&records) {
    //         Ok(data) => HttpResponse::Ok().json(data),
    //         Err(_) => HttpResponse::InternalServerError().finish(),
    //     },
    //     Err(e) => {
    //         println!("Get records error: {:?}", e);
    //         HttpResponse::InternalServerError().finish()
    //     }
    // }
    // }
    // None => HttpResponse::BadRequest().finish(),
    // }
}
