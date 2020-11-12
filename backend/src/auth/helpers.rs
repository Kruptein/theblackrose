use std::{collections::HashMap, error::Error};

use tokio::sync::RwLock;

use super::models::{Jwks, JwtHeader};

pub async fn fetch_jwks(uri: &str) -> Result<Jwks, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<Jwks>().await?;
    Ok(val)
}

pub fn find_kid(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.splitn(2, '.').collect();
    let json = base64::decode(parts[0]).unwrap();
    match serde_json::from_slice::<JwtHeader>(&json) {
        Ok(header) => Some(header.kid),
        Err(_) => None,
    }
}

pub async fn get_user_from_cache(
    tokens: &RwLock<HashMap<String, i32>>,
    token: &str,
) -> Option<i32> {
    let tokens = tokens.read().await;
    tokens.get(token).map(i32::to_owned)
}
