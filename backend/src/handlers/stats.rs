use riven::consts::{Champion, Queue};
use sqlx::{query, Error, PgPool};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct ChampionWinrate {
    pub wins: u16,
    pub total: u16,
}

pub async fn get_winrate_for_champion(
    conn: &PgPool,
    champion: Champion,
    summoner_id: &str,
    queue: Option<Queue>,
) -> Result<ChampionWinrate, Error> {
    let data = if queue.is_none() {
        let a = query!(
            r#"SELECT COUNT(CASE WHEN win THEN 1 END) as win, COUNT(*) as total
        FROM participants p
        INNER JOIN participant_stats_general ps ON ps.participant_id = p.id
        INNER JOIN summoners s ON s.summoner_id = $1 AND p.summoner_id = s.id
        WHERE champion_id = $2"#,
            summoner_id,
            champion.identifier(),
        )
        .fetch_one(conn)
        .await?;
        (a.win, a.total)
    } else {
        let queue: u16 = queue.unwrap().into();
        let queue: i32 = queue.into();
        let a = query!(
            "SELECT COUNT(CASE WHEN win THEN 1 END) as win, COUNT(*) as total
            FROM participants p
            INNER JOIN participant_stats_general ps ON ps.participant_id = p.id
            INNER JOIN summoners s ON s.summoner_id = $1 AND p.summoner_id = s.id
            INNER JOIN match_references mr ON mr.game_id = p.game_id AND mr.summoner_id = s.id
            WHERE champion_id = $2 AND queue = $3",
            summoner_id,
            champion.identifier(),
            queue
        )
        .fetch_one(conn)
        .await?;
        (a.win, a.total)
    };
    Ok(ChampionWinrate {
        wins: u16::try_from(data.0.unwrap()).unwrap(),
        total: u16::try_from(data.1.unwrap()).unwrap(),
    })
}
