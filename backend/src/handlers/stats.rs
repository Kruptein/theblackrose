use sqlx::{Error, PgPool};
use std::collections::HashMap;

use crate::db::stats::get_winrates_for_summoner;

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

pub async fn get_all_winrates(
    conn: &PgPool,
    summoner_id: i32,
) -> Result<HashMap<i16, HashMap<String, HashMap<i16, ChampionWinrate>>>, Error> {
    let data = get_winrates_for_summoner(conn, summoner_id).await?;

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
