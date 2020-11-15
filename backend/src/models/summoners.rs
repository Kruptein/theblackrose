use crate::schema::*;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Summoner {
    pub id: i32,
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: Option<i64>,
    pub name: String,
    pub summoner_id: Option<String>,
    pub puuid: Option<String>,
    pub summoner_level: Option<i64>,
    pub last_match_query_time: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "summoners"]
pub struct NewSummoner<'a> {
    pub account_id: &'a str,
    pub profile_icon_id: i32,
    pub revision_date: Option<i64>,
    pub name: &'a str,
    pub summoner_id: Option<&'a str>,
    pub puuid: Option<&'a str>,
    pub summoner_level: Option<i64>,
}
