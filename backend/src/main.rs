mod auth;
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
use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, middleware::Logger, App, Error, HttpServer};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use db::{establish_pool, Pool};
// use diesel::prelude::*;
use dotenv::dotenv;
use riven::RiotApi;

pub struct AppState {
    db_conn: Pool,
    riot_api: RiotApi,
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(|| Default::default());

    match auth::validate_token(req.app_data().unwrap(), credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let local = tokio::task::LocalSet::new();
    let sys = actix_web::rt::System::run_in_tokio("server", &local);

    let pool = establish_pool();

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(AppState {
                riot_api: rito::create_riot_api(),
                db_conn: pool.clone(),
            })
            .wrap(auth)
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
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
