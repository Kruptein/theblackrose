use actix_web::{get, web, HttpResponse, Responder};

use crate::{rito, AppState};

#[get("/api/v1/mastery/{username}")]
pub async fn get_masteries(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let masteries = rito::get_masteries(&data.riot_api, path.0).await;
    HttpResponse::Ok().body(masteries.to_string())
}
