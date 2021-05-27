use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    auth::helpers::get_user_from_cache, handlers::connections as h,
    handlers::users::get_user_by_id, AppState,
};

// Ideally we would just be able to use Option<Vec<String>> here,
// but I couldn't get it to work. The function gets called when passing a ?names[]=test, but the query itself remains empty
#[derive(Deserialize)]
pub struct MatchFilter {
    names: Option<String>,
    after: Option<i64>,
    before: Option<i64>,
    length: Option<i8>,
    queues: Option<String>,
}

impl MatchFilter {
    pub fn get_names(&self) -> Option<Vec<String>> {
        self.names
            .as_ref()
            .map(|names| names.split(",").map(|name| name.to_owned()).collect())
    }

    pub fn get_before_time(&self) -> Option<i64> {
        self.before
    }

    pub fn get_after_time(&self) -> Option<i64> {
        self.after
    }

    pub fn get_length(&self) -> i8 {
        self.length.unwrap_or(10)
    }

    pub fn get_queues(&self) -> Option<Vec<i32>> {
        // todo: UPDATE DEFAULT ARRAY HERE
        self.queues.as_ref().map(|queues| {
            queues
                .split(",")
                .map(|queue| queue.parse::<i32>().unwrap())
                .collect()
        })
    }
}

#[get("/matches/")]
pub async fn get_matches(
    data: web::Data<AppState>,
    query: web::Query<MatchFilter>,
    auth: BearerAuth,
) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let user = get_user_by_id(db_pool, user).await.unwrap();
            match h::get_connection_matches(db_pool, user, query.0).await {
                Ok(matches) => match serde_json::to_string(&matches) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(e) => {
                    println!("Get matches error: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
