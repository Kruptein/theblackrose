use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{db::Pool, errors::ServiceError, handlers::users::register_subject, AppState};

use super::{
    helpers::{fetch_jwks, find_kid},
    models::Claims,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(|| Default::default());

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    // let db_conn = state.db_conn.clone();

    let token = credentials.token();

    let a = state.subjects.read().await;
    let x = a.get(token);
    match x.clone() {
        Some(_) => Ok(req),
        None => Ok(req)
        // None => match validate_token(db_conn, token.clone()).await {
        //     Ok(user) => {
        //         // let subjs = state.subjects.write().await;
        //         // subjs.insert(token.clone(), user);
        //         Ok(req)
        //     }
        //     Err(_) => Err(AuthenticationError::from(config).into()),
        // },
    }
}

pub async fn validate_token(db_pool: Pool, token: &str) -> Result<i32, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let audience = std::env::var("AUDIENCE").expect("AUDIENCE must be set");

    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await
    .expect("failed to fetch jwks");
    let kid = find_kid(token).unwrap();
    let jwk = jwks.find(&kid).unwrap();

    let mut validation = Validation {
        iss: Some(authority),
        algorithms: vec![Algorithm::RS256],
        ..Validation::default()
    };
    validation.set_audience(&[audience]);

    match decode::<Claims>(
        &token,
        &DecodingKey::from_rsa_components(jwk.n.as_str(), jwk.e.as_str()),
        &validation,
    ) {
        Ok(token_data) => Ok(register_subject(db_pool, token, token_data.claims.sub).await),
        Err(err) => Err(ServiceError::JWKSFetchError(err.to_string())),
    }
}
