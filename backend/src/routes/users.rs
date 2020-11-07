use actix_web::{delete, get, post, web, Error, HttpResponse, Responder};

use crate::{handlers::users as h, models::InputUser, rito, AppState};

#[get("/api/v1/mastery/{username}")]
pub async fn get_masteries(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let masteries = rito::get_masteries(&data.riot_api, path.0).await;
    HttpResponse::Ok().body(masteries.to_string())
}

#[get("/users")]
pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let res = web::block(|| h::get_all_users(state))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    state: web::Data<AppState>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || h::get_user_by_id(state, user_id))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/users")]
pub async fn add_user(
    state: web::Data<AppState>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || h::add_user(state, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(
    state: web::Data<AppState>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let res = web::block(move || h::delete_user(state, user_id))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
