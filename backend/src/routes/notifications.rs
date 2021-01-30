use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{auth::helpers::get_user_from_cache, handlers::notifications as n, AppState};

#[get("/notifications/")]
pub async fn get_notifications(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let db_conn = db_pool.clone().get().unwrap();
            match web::block(move || n::get_notifications(&db_conn, user)).await {
                Ok(notifications) => match serde_json::to_string(&notifications) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(e) => {
                    println!("Get notifications error: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
