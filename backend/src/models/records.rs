use riven::models::match_v5::Participant;
use strum_macros::*;

#[derive(EnumIter, Clone, Debug, Deserialize, Display, PartialEq)]
pub enum RecordType {
    TotalMinionsKilled,
    Kills,
    Deaths,
    Assists,
    CsPerMinute,
    KDA,
}

impl RecordType {
    pub fn get_value(&self, stats: &Participant, duration: i64) -> f32 {
        match self {
            RecordType::Kills => stats.kills as f32,
            RecordType::Deaths => stats.deaths as f32,
            RecordType::Assists => stats.assists as f32,
            RecordType::KDA => (stats.kills + stats.assists) as f32 / stats.deaths.max(1) as f32,
            RecordType::TotalMinionsKilled => stats.total_minions_killed as f32,
            RecordType::CsPerMinute => (60 * stats.total_minions_killed) as f32 / duration as f32,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub id: i32,
    pub summoner_id: i32,
    pub game_id: i64,
    pub record_type: i16,
    pub value: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ApiRecord {
    pub id: i32,
    pub record_type: i16,
    pub value: f32,
    pub game_id: i64,
    pub name: String,
    pub queue_id: i16,
}
