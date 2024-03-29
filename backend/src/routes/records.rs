use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    auth::helpers::get_user_from_cache, db::users::get_user_by_id,
    handlers::records::get_connection_records, AppState,
};

#[derive(Debug, Deserialize)]
pub struct RecordFilter {
    names: Option<String>,
    queues: Option<String>,
}

impl RecordFilter {
    pub fn get_names(&self) -> Option<Vec<String>> {
        self.names
            .as_ref()
            .map(|names| names.split(',').map(|name| name.to_owned()).collect())
    }

    pub fn get_queue_ids(&self) -> Vec<i16> {
        let ids = self.queues.as_ref().map(|q| {
            q.split(',')
                .filter_map(|s| s.parse::<i16>().ok())
                .collect::<Vec<i16>>()
        });

        ids.unwrap_or_else(|| vec![450])
    }
}

#[get("/records/")]
pub async fn get_records(
    data: web::Data<AppState>,
    query: web::Query<RecordFilter>,
    auth: BearerAuth,
) -> impl Responder {
    let db_pool = &data.db_conn;

    match get_user_from_cache(&data.tokens, auth.token()).await {
        Some(user) => {
            let user = get_user_by_id(db_pool, user).await.unwrap();
            match get_connection_records(db_pool, user, query.0).await {
                Ok(records) => HttpResponse::Ok().json(records),
                Err(e) => {
                    println!("Get records error: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
