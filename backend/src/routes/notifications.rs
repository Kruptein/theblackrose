use actix_web::{delete, get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{auth::helpers::get_user_from_cache, handlers::notifications as n, AppState};

#[get("/notifications/")]
pub async fn get_notifications(data: web::Data<AppState>, auth: BearerAuth) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => match n::get_notifications(db_pool, user).await {
            Ok(notifications) => match serde_json::to_string(&notifications) {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(_) => HttpResponse::InternalServerError().finish(),
            },
            Err(e) => {
                println!("Get notifications error: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        None => HttpResponse::BadRequest().finish(),
    }
}

#[delete("/notifications/{notification_id}/")]
pub async fn remove_notification(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    auth: BearerAuth,
) -> impl Responder {
    let db_pool = &data.db_conn;
    let notification_id = path.0;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => match n::owns_notification(db_pool, notification_id, user).await {
            Ok(true) => match n::remove_notification(db_pool, notification_id).await {
                Ok(notifications) => match serde_json::to_string(&notifications) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(e) => {
                    println!("Get notifications error: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            },
            Ok(false) => HttpResponse::NotFound().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        None => HttpResponse::BadRequest().finish(),
    }
}
