use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    auth::helpers::get_user_from_cache, handlers::connections as h,
    handlers::users::get_user_by_id, AppState,
};

#[get("/matches/")]
pub async fn get_matches(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let db_conn = db_pool.clone().get().unwrap();
            let user = move || get_user_by_id(&db_conn, user);
            let user = web::block(user).await.unwrap();
            let db_conn = db_pool.get().unwrap();
            match web::block(move || h::get_connection_matches(&db_conn, user)).await {
                Ok(matches) => match serde_json::to_string(&matches) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
