use crate::schema::*;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub auth0_subject: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub auth0_subject: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputUser {
    pub auth0_subject: String,
    pub email: String,
}
