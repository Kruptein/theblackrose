#[derive(sqlx::Type)]
pub struct User {
    pub id: i32,
    pub auth0_subject: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
