use sqlx::{query_as, Error, PgPool};

use crate::{
    models::{matches::MatchFeedElement, users::User},
    routes::records::RecordFilter,
};

use super::matches::get_game_info;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ApiRecord {
    id: i32,
    record_type: i16,
    value: f32,
    game_id: i64,
    name: String,
    queue_id: i16,
}

struct ConnectionId {
    id: i32,
}

pub async fn get_connection_records(
    conn: &PgPool,
    user: User,
    filter: RecordFilter,
) -> Result<(Vec<ApiRecord>, Vec<MatchFeedElement>), Error> {
    let user_connections = match filter.get_names() {
        Some(names) => {
            query_as!(ConnectionId, r#"SELECT s.id as "id!: i32" FROM summoners s INNER JOIN connections c ON c.summoner_id = s.id WHERE s.name = any($1)"#, &names).fetch_all(conn).await?
        },
        None => query_as!(ConnectionId, "SELECT s.id FROM summoners s INNER JOIN connections c ON c.summoner_id = s.id WHERE c.user_id = $1", user.id).fetch_all(conn).await?
    };
    let user_connections = user_connections
        .into_iter()
        .map(|summoner| summoner.id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let records = query_as::<_, ApiRecord>(
        format!(
            "
        SELECT r.id, r.record_type, r.value, r.game_id, s.name, m.queue_id
        FROM records r
        INNER JOIN summoners s ON r.summoner_id = s.id
        INNER JOIN matches m ON m.game_id = r.game_id
        INNER JOIN (
            SELECT record_type, MAX(value) as max_value
            FROM records rr
            INNER JOIN matches mm ON mm.game_id = rr.game_id
            WHERE
                summoner_id IN ({})
                AND mm.queue_id IN ({})
            GROUP BY record_type
        ) l ON
            r.record_type = l.record_type
            AND r.value = l.max_value
        WHERE
            r.summoner_id IN ({})
            AND m.queue_id IN ({})
        ORDER BY record_type;",
            user_connections,
            filter.get_queues().unwrap_or(&"450".to_owned()),
            user_connections,
            filter.get_queues().unwrap_or(&"450".to_owned()),
        )
        .as_str(),
    )
    .fetch_all(conn)
    .await?;

    let mut match_info_vec: Vec<MatchFeedElement> = vec![];
    for record in records.iter() {
        match_info_vec.push(get_game_info(record.game_id, conn).await?);
    }
    Ok((records, match_info_vec))
}
