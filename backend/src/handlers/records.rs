use diesel::sql_types::{Float, Int2, Int4, Int8, Text};
use diesel::{prelude::*, result::Error, sql_query, QueryDsl, RunQueryDsl};

use crate::{
    db::Conn,
    models::{matches::MatchFeedElement, users::User},
    routes::records::RecordFilter,
    schema::connections::dsl::{self as c, connections},
    schema::summoners::dsl::{self as s, summoners},
};

use super::matches::get_game_info;

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Record {
    #[sql_type = "Int4"]
    id: i32,
    #[sql_type = "Int2"]
    record_type: i16,
    #[sql_type = "Float"]
    value: f32,
    #[sql_type = "Int8"]
    game_id: i64,
    #[sql_type = "Text"]
    name: String,
    #[sql_type = "Int2"]
    queue_id: i16,
}

pub fn get_connection_records(
    conn: &Conn,
    user: User,
    filter: RecordFilter,
) -> Result<(Vec<Record>, Vec<MatchFeedElement>), Error> {
    let user_connections: Vec<i32> = match filter.get_names() {
        Some(names) => summoners
            .inner_join(connections)
            .filter(s::name.eq_any(names.to_owned()))
            .select(s::id)
            .get_results(conn)?,
        None => summoners
            .inner_join(connections)
            .filter(c::user_id.eq(user.id))
            .select(s::id)
            .get_results(conn)?,
    };
    let user_connections = user_connections
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let user_connections = format!("({})", user_connections);

    let query = format!("select r.id, r.record_type, r.value, r.game_id, s.name, m.queue_id FROM records r INNER JOIN summoners s ON r.summoner_id = s.id INNER JOIN matches m ON m.game_id = r.game_id INNER JOIN (SELECT record_type, MAX(value) as max_value FROM records WHERE summoner_id IN {} GROUP BY record_type) l ON r.record_type = l.record_type AND r.value = l.max_value WHERE r.summoner_id IN {} ORDER BY record_type;", user_connections, user_connections);

    // can't join the same table in diesel atm so we use a raw query here
    let records_query: Vec<Record> = sql_query(query).get_results(conn)?;
    let mut match_info_vec: Vec<MatchFeedElement> = vec![];
    for record in records_query.iter() {
        match_info_vec.push(get_game_info(record.game_id, conn)?);
    }
    Ok((records_query, match_info_vec))
}
