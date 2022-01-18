use riven::consts::{Champion, Queue};
use sqlx::{query, query_as, Error, PgPool};
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

#[derive(Debug, Serialize)]
pub struct ChampionWinrate {
    pub wins: u16,
    pub total: u16,
}

impl ChampionWinrate {
    fn new() -> ChampionWinrate {
        ChampionWinrate { wins: 0, total: 0 }
    }

    fn add(&mut self, win: bool) {
        self.wins += if win { 1 } else { 0 };
        self.total += 1;
    }

    pub(crate) fn join(&mut self, v: ChampionWinrate) -> () {
        self.wins += v.wins;
        self.total += v.total;
    }
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

struct AllWinrate {
    champion_id: i16,
    queue_id: i16,
    win: bool,
    name: String,
}

pub async fn get_all_winrates(
    conn: &PgPool,
    summoner_id: i32,
) -> Result<HashMap<i16, HashMap<String, HashMap<i16, ChampionWinrate>>>, Error> {
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

    let mut collection = HashMap::new();

    for row in data {
        let champion_data = collection.entry(row.champion_id).or_insert(HashMap::new());
        let summoner_data = champion_data.entry(row.name).or_insert(HashMap::new());
        let queue_data = summoner_data
            .entry(row.queue_id)
            .or_insert(ChampionWinrate::new());
        queue_data.add(row.win);
    }

    Ok(collection)
}
