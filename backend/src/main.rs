mod db;
mod errors;
mod handlers;
mod models;
mod rito;
mod routes;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use crate::routes::users;

// use self::model::*;
use actix_web::{App, HttpServer, dev::ServiceRequest, middleware};
use db::{establish_pool, Pool};
// use diesel::prelude::*;
use dotenv::dotenv;
use riven::RiotApi;

pub struct AppState {
    db_conn: Pool,
    riot_api: RiotApi,
}

#[get("/api/v1/mastery/{username}")]
async fn get_masteries(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let masteries = rito::get_masteries(&data.riot_api, path.0).await;
    HttpResponse::Ok().body(masteries.to_string())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    let pool = establish_pool();

    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                riot_api: rito::create_riot_api(),
                db_conn: pool.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(users::get_users)
            .service(users::add_user)
            .service(users::delete_user)
            .service(users::get_user_by_id)
            .service(users::get_masteries)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await?;
    sys.await?;
    Ok(server)
}
