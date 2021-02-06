#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub auth0_subject: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputUser {
    pub auth0_subject: String,
    pub email: String,
}
