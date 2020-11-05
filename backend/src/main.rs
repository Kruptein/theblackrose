mod rito;
extern crate tokio;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use riven::RiotApi;

struct AppState {
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

    let server = HttpServer::new(|| {
        App::new()
            .data(AppState {
                riot_api: rito::create_riot_api(),
            })
            .service(get_masteries)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await?;
    sys.await?;
    Ok(server)
}
