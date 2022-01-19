use sqlx::{Error, PgPool};

use crate::{
    db::{
        connections::{get_summoner_ids_by_connection, get_summoner_ids_by_name},
        matches::get_game_details,
        records::get_records,
    },
    models::{matches::MatchFeedElement, records::ApiRecord, users::User},
    routes::records::RecordFilter,
};

pub async fn get_connection_records(
    conn: &PgPool,
    user: User,
    filter: RecordFilter,
) -> Result<(Vec<ApiRecord>, Vec<MatchFeedElement>), Error> {
    let user_connections = match filter.get_names() {
        Some(names) => get_summoner_ids_by_name(conn, names).await?,
        None => get_summoner_ids_by_connection(conn, user.id).await?,
    };

    let records = get_records(conn, user_connections, filter.get_queue_ids()).await?;

    let mut match_info_vec: Vec<MatchFeedElement> = vec![];
    for record in records.iter() {
        match_info_vec.push(get_game_details(record.game_id, conn).await?);
    }
    Ok((records, match_info_vec))
}
