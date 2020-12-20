use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    auth::helpers::get_user_from_cache, handlers::connections as h,
    handlers::users::get_user_by_id, AppState,
};

// Ideally we would just be able to use Option<Vec<String>> here,
// but I couldn't get it to work. The function gets called when passing a ?names[]=test, but the query itself remains empty
#[derive(Deserialize)]
pub struct Filter {
    names: Option<String>,
    start: Option<i64>,
}

impl Filter {
    pub fn get_names(&self) -> Option<Vec<String>> {
        self.names
            .as_ref()
            .map(|names| names.split(",").map(|name| name.to_owned()).collect())
    }

    pub fn get_start_time(&self) -> Option<i64> {
        self.start
    }
}

#[get("/matches/")]
pub async fn get_matches(
    data: web::Data<AppState>,
    query: web::Query<Filter>,
    auth: BearerAuth,
) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let db_conn = db_pool.clone().get().unwrap();
            let user = move || get_user_by_id(&db_conn, user);
            let user = web::block(user).await.unwrap();
            let db_conn = db_pool.get().unwrap();
            match web::block(move || h::get_connection_matches(&db_conn, user, query)).await {
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
