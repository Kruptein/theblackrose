use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    actor::ConnectionUpdateMessage, auth::helpers::get_user_from_cache, handlers::connections as h,
    handlers::users::get_user_by_id, rito::summoners::get_summoner_by_name, AppState,
};

#[get("/connections/")]
pub async fn get_connections(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let db_conn = db_pool.clone().get().unwrap();
            let user = move || get_user_by_id(&db_conn, user);
            let user = web::block(user).await.unwrap();
            let db_conn = db_pool.get().unwrap();
            match web::block(move || h::get_connection_short_info(&db_conn, user)).await {
                Ok(connections) => match serde_json::to_string(&connections) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(_) => HttpResponse::Ok().finish(),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[post("/connections/{summoner_name}/")]
pub async fn add_connection(
    data: web::Data<AppState>,
    path: web::Path<String>,
    auth: BearerAuth,
) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.0;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let user = get_user_by_id(&db_pool.get().unwrap(), user).unwrap();
            match get_summoner_by_name(riot_api, db_pool, username.as_str()).await {
                Some(summoner) => {
                    match h::add_connection(&db_pool.get().unwrap(), user, summoner) {
                        Ok(connection) => {
                            println!("Sending message");
                            &data
                                .update_task
                                .send(ConnectionUpdateMessage { connection })
                                .await;
                            HttpResponse::Created().finish()
                        }
                        Err(_) => HttpResponse::Conflict().finish(),
                    }
                }
                None => HttpResponse::NotFound().finish(),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
