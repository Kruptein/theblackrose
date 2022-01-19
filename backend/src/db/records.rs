use sqlx::{query_as, Error, PgPool};

use crate::models::records::ApiRecord;

pub async fn get_records(
    conn: &PgPool,
    summoner_ids: Vec<i32>,
    queues: Vec<i16>,
) -> Result<Vec<ApiRecord>, Error> {
    let records = query_as!(
        ApiRecord,
        r#"
        SELECT r.id as "id!", r.record_type as "record_type!", r.value as "value!", r.game_id as "game_id!", s.name as "name!", m.queue_id as "queue_id!"
        FROM records r
        INNER JOIN summoners s ON r.summoner_id = s.id
        INNER JOIN matches m ON m.game_id = r.game_id
        INNER JOIN (
            SELECT record_type, MAX(value) as max_value
            FROM records rr
            INNER JOIN matches mm ON mm.game_id = rr.game_id
            WHERE
                summoner_id = ANY($1)
                AND mm.queue_id = ANY($2)
            GROUP BY rr.record_type
        ) l ON
            r.record_type = l.record_type
            AND r.value = l.max_value
            WHERE
            r.summoner_id = ANY($1)
            AND m.queue_id = ANY($2)
        ORDER BY r.record_type;"#,
        &summoner_ids,
        &queues
    )
    .fetch_all(conn)
    .await?;

    Ok(records)
}
