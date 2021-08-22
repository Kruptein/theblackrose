use riven::consts::{Champion, Queue};
use sqlx::{query, query_as, Error, PgPool};
use std::{collections::HashMap, convert::TryFrom};

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

struct AllWinrate {
    champion: String,
    queue: i32,
    win: bool,
    name: String,
}

pub async fn get_all_winrates(
    conn: &PgPool,
    summoner_id: i32,
) -> Result<HashMap<String, HashMap<String, HashMap<i32, ChampionWinrate>>>, Error> {
    let data = query_as!(
        AllWinrate,
        r#"SELECT champion, queue, win, name
        FROM match_references mr
        INNER JOIN participants p
        ON mr.game_id = p.game_id AND mr.summoner_id = p.summoner_id
        INNER JOIN participant_stats_general ps
        ON ps.participant_id = p.id
        INNER JOIN summoners s
        ON s.id = mr.summoner_id
        WHERE mr.summoner_id = $1 AND mr.queue = 450
    "#,
        summoner_id,
    )
    .fetch_all(conn)
    .await?;
    // let data: Vec<(String, i32, bool)> = a;

    let mut collection = HashMap::new();

    for row in data {
        let champion_data = collection.entry(row.champion).or_insert(HashMap::new());
        let summoner_data = champion_data.entry(row.name).or_insert(HashMap::new());
        let queue_data = summoner_data
            .entry(row.queue)
            .or_insert(ChampionWinrate::new());
        queue_data.add(row.win);
    }

    Ok(collection)
}
