use crate::schema::*;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Connection {
    pub user_id: i32,
    pub summoner_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "connections"]
pub struct NewConnection {
    pub user_id: i32,
    pub summoner_id: i32,
}
