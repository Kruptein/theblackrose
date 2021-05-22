#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub message: String,
}
