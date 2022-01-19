use riven::consts::{Champion, Queue};
use sqlx::{query, query_as, Error, PgPool};

use crate::handlers::stats::ChampionWinrate;

pub struct AllWinrate {
    pub champion_id: i16,
    pub queue_id: i16,
    pub win: bool,
    pub name: String,
}

pub async fn get_winrates_for_summoner(
    conn: &PgPool,
    summoner_id: i32,
) -> Result<Vec<AllWinrate>, Error> {
    let data = query_as!(
        AllWinrate,
        r#"SELECT champion_id, queue_id, win, name
        FROM participant_general pg
        INNER JOIN summoners s
        ON s.id = pg.summoner_id
        INNER JOIN matches m
        ON m.game_id = pg.game_id
        WHERE s.id = $1 AND m.queue_id = 450
    "#,
        summoner_id,
    )
    .fetch_all(conn)
    .await?;

    Ok(data)
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
        FROM participant_general pg
        INNER JOIN summoners s ON s.summoner_id = $1 AND pg.summoner_id = s.id
        WHERE champion_id = $2"#,
            summoner_id,
            champion.0,
        )
        .fetch_one(conn)
        .await?;
        (a.win, a.total)
    } else {
        let queue: i16 = u16::from(queue.unwrap()).try_into().unwrap();
        let a = query!(
            "SELECT COUNT(CASE WHEN win THEN 1 END) as win, COUNT(*) as total
            FROM participant_general pg
            INNER JOIN summoners s ON s.summoner_id = $1 AND pg.summoner_id = s.id
            INNER JOIN matches m ON m.game_id = pg.game_id
            WHERE champion_id = $2 AND queue_id = $3",
            summoner_id,
            champion.0,
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
