use std::error::Error;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{db::register_subject, errors::ServiceError, AppState};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String,
}

#[derive(Debug, Deserialize)]
struct Jwk {
    n: String,
    e: String,
    kid: String,
}

#[derive(Debug, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
struct JwtHeader {
    alg: Algorithm,
    typ: String,
    kid: String,
}

impl Jwks {
    fn find(&self, kid: &str) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid == kid)
    }
}

pub async fn validate_token(state: &AppState, token: &str) -> Result<bool, ServiceError> {
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
        Ok(token_data) => {
            register_subject(state, token, token_data.claims.sub).await;
            Ok(true)
        }
        Err(err) => Err(ServiceError::JWKSFetchError(err.to_string())),
    }
}

async fn fetch_jwks(uri: &str) -> Result<Jwks, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<Jwks>().await?;
    Ok(val)
}

fn find_kid(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.splitn(2, '.').collect();
    let json = base64::decode(parts[0]).unwrap();
    match serde_json::from_slice::<JwtHeader>(&json) {
        Ok(header) => Some(header.kid),
        Err(_) => None,
    }
}
