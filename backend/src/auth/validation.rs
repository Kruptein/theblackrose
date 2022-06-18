use std::collections::HashMap;

use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::{errors::ServiceError, AppState};

use super::{
    cache::update_token_cache,
    helpers::{fetch_jwks, find_kid},
    models::Claims,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();

    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let tokens = &state.tokens;

    match validate_token(&state.db_conn, tokens, credentials.token()).await {
        Ok(()) => Ok(req),
        Err(e) => {
            println!("Validate Token Error: {:?}", e);
            Err(AuthenticationError::from(config).into())
        }
    }
}

pub async fn validate_token(
    db_pool: &PgPool,
    tokens: &RwLock<HashMap<String, i32>>,
    token: &str,
) -> Result<(), ServiceError> {
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

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[authority]);
    validation.set_audience(&[audience]);

    let decoding_key = &DecodingKey::from_rsa_components(jwk.n.as_str(), jwk.e.as_str()).unwrap();

    match decode::<Claims>(token, decoding_key, &validation) {
        Ok(token_data) => {
            update_token_cache(db_pool, tokens, token, token_data.claims.sub.as_str()).await;
            Ok(())
        }
        Err(err) => Err(ServiceError::JWKSFetchError(err.to_string())),
    }
}
