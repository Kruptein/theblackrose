#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
    iss: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwk {
    pub n: String,
    pub e: String,
    pub kid: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
pub struct JwtHeader {
    // alg: Algorithm,
    // typ: String,
    pub kid: String,
}

impl Jwks {
    pub fn find(&self, kid: &str) -> Option<&Jwk> {
        self.keys.iter().find(|k| k.kid == kid)
    }
}
