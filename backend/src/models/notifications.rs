use crate::schema::*;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Notification {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub message: String,
}

#[derive(Insertable, Debug)]
#[table_name = "notifications"]
pub struct NewNotification {
    pub user_id: i32,
    pub title: String,
    pub message: String,
}
