use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    actors::gamefetcher::{ConnectionUpdateMessage, SummonerUpdateMessage},
    auth::helpers::get_user_from_cache,
    db::connections as db,
    db::users::get_user_by_id,
    rito::summoners::get_summoner_by_name,
    AppState,
};

#[get("/connections/")]
pub async fn get_connections(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let user = get_user_by_id(db_pool, user).await.unwrap();
            match db::get_connection_short_info(db_pool, user).await {
                Ok(connections) => HttpResponse::Ok().json(&connections),
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
    let username = path.into_inner();

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let user = get_user_by_id(db_pool, user).await.unwrap();
            match get_summoner_by_name(riot_api, db_pool, username.as_str()).await {
                Some(summoner) => match db::add_connection(db_pool, user, summoner).await {
                    Ok(connection) => {
                        data.update_task
                            .send(ConnectionUpdateMessage { connection })
                            .await
                            .unwrap();
                        HttpResponse::Created().finish()
                    }
                    Err(_) => HttpResponse::Conflict().finish(),
                },
                None => HttpResponse::NotFound().finish(),
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[get("/connection/{summoner_name}/refresh")]
pub async fn refresh_connection(
    data: web::Data<AppState>,
    path: web::Path<String>,
    auth: BearerAuth,
) -> impl Responder {
    let riot_api = &data.riot_api;
    let db_pool = &data.db_conn;
    let username = path.into_inner();

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(_) => match get_summoner_by_name(riot_api, db_pool, username.as_str()).await {
            Some(summoner) => {
                data.update_task
                    .send(SummonerUpdateMessage { summoner })
                    .await
                    .unwrap();
                HttpResponse::Ok().finish()
            }
            None => HttpResponse::NotFound().finish(),
        },
        None => HttpResponse::BadRequest().finish(),
    }
}
