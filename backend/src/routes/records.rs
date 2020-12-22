use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    auth::helpers::get_user_from_cache,
    handlers::{records::get_connection_records, users::get_user_by_id},
    AppState,
};

#[derive(Deserialize)]
pub struct RecordFilter {
    names: Option<String>,
}

impl RecordFilter {
    pub fn get_names(&self) -> Option<Vec<String>> {
        self.names
            .as_ref()
            .map(|names| names.split(",").map(|name| name.to_owned()).collect())
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
            let db_conn = db_pool.clone().get().unwrap();
            let user = move || get_user_by_id(&db_conn, user);
            let user = web::block(user).await.unwrap();
            let db_conn = db_pool.get().unwrap();
            match web::block(move || get_connection_records(&db_conn, user, query.0)).await {
                Ok(records) => match serde_json::to_string(&records) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(e) => {
                    println!("Get records error: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}
