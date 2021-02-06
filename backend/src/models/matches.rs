use super::summoners::Summoner;

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MatchFeedParticipant {
    pub participant: Participant,
    pub summoner: Summoner,
    pub general: ParticipantStatsGeneral,
    pub kills: ParticipantStatsKills,
}

#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MatchFeedElement {
    pub match_info: Match,
    pub participants: Vec<MatchFeedParticipant>,
}

pub struct MatchReference {
    pub game_id: i64,
    pub summoner_id: i32,
    pub role: Option<String>,
    pub platform_id: String,
    pub champion: String,
    pub queue: i32,
    pub lane: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Match {
    pub game_id: i64,
    pub queue_id: i16,
    pub game_type: String,
    pub game_duration: i64,
    pub platform_id: String,
    pub game_creation: i64,
    pub season_id: i16,
    pub game_version: String,
    pub map_id: i16,
    pub game_mode: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamStats {
    pub game_id: i64,
    pub team_id: i16,
    pub tower_kills: i32,
    pub rift_herald_kills: i32,
    pub first_blood: bool,
    pub inhibitor_kills: i32,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub baron_kills: i32,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub vilemaw_kills: i32,
    pub first_rift_herald: bool,
    pub win: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Participant {
    pub id: i32,
    pub game_id: i64,
    pub participant_id: i32,
    pub summoner_id: Option<i32>,
    pub champion_id: String,
    pub team_id: i16,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub highest_achieved_season_tier: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsGeneral {
    pub participant_id: i32,

    pub champ_level: i32,
    pub win: bool,

    pub gold_earned: i32,
    pub gold_spent: i32,

    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsKills {
    pub participant_id: i32,

    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,

    pub double_kills: i32,
    pub triple_kills: i32,
    pub quadra_kills: i32,
    pub penta_kills: i32,
    pub unreal_kills: i32,
    pub largest_multi_kill: i32,
    pub largest_killing_spree: i32,
    pub killing_sprees: i32,

    pub longest_time_spent_living: i32,

    pub first_tower_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_blood_kill: Option<bool>,
    pub first_blood_assist: Option<bool>,
    pub first_inhibitor_kill: Option<bool>,
    pub first_inhibitor_assist: Option<bool>,

    pub inhibitor_kills: Option<i32>,
    pub turret_kills: Option<i32>,

    pub neutral_minions_killed: i32,
    pub neutral_minions_killed_enemy_jungle: Option<i32>,
    pub neutral_minions_killed_team_jungle: Option<i32>,
    pub total_minions_killed: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantStatsDamage {
    pub participant_id: i32,

    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,

    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,

    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magical_damage_taken: i64,

    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_taken: i64,

    pub damage_dealt_to_turrets: i64,
    pub damage_dealt_to_objectives: i64,

    pub damage_self_mitigated: i64,
    pub largest_critical_strike: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantStatsScores {
    pub participant_id: i32,

    pub combat_player_score: Option<i32>,
    pub objective_player_score: Option<i32>,
    pub total_player_score: Option<i32>,

    pub total_score_rank: Option<i32>,

    pub team_objective: Option<i32>,

    pub player_score0: Option<i32>,
    pub player_score1: Option<i32>,
    pub player_score2: Option<i32>,
    pub player_score3: Option<i32>,
    pub player_score4: Option<i32>,
    pub player_score5: Option<i32>,
    pub player_score6: Option<i32>,
    pub player_score7: Option<i32>,
    pub player_score8: Option<i32>,
    pub player_score9: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantStatsUtility {
    pub participant_id: i32,

    pub total_units_healed: i32,
    pub total_heal: i64,

    pub total_time_crowd_control_dealt: i32,
    pub time_c_cing_others: i64,

    pub wards_placed: Option<i32>,
    pub vision_wards_bought_in_game: i32,
    pub vision_score: Option<i64>,
    pub wards_killed: Option<i32>,
    pub sight_wards_bought_in_game: Option<i32>,
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Participant
where
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i64: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    i16: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i16: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.game_id);
        encoder.encode(&self.participant_id);
        encoder.encode(&self.summoner_id);
        encoder.encode(&self.champion_id);
        encoder.encode(&self.team_id);
        encoder.encode(&self.spell1_id);
        encoder.encode(&self.spell2_id);
        encoder.encode(&self.highest_achieved_season_tier);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        9usize * (4 + 4)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.id)
            + <i64 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.game_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.participant_id)
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.summoner_id,
            )
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.champion_id)
            + <i16 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.team_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.spell1_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.spell2_id)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.highest_achieved_season_tier,
            )
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Participant
where
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    i16: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i16: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let game_id = decoder.try_decode::<i64>()?;
        let participant_id = decoder.try_decode::<i32>()?;
        let summoner_id = decoder.try_decode::<Option<i32>>()?;
        let champion_id = decoder.try_decode::<String>()?;
        let team_id = decoder.try_decode::<i16>()?;
        let spell1_id = decoder.try_decode::<i32>()?;
        let spell2_id = decoder.try_decode::<i32>()?;
        let highest_achieved_season_tier = decoder.try_decode::<Option<String>>()?;
        ::std::result::Result::Ok(Participant {
            id,
            game_id,
            participant_id,
            summoner_id,
            champion_id,
            team_id,
            spell1_id,
            spell2_id,
            highest_achieved_season_tier,
        })
    }
}
impl ::sqlx::Type<::sqlx::Postgres> for Participant {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Participant")
    }
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for MatchReference
where
    i64: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    i64: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.game_id);
        encoder.encode(&self.summoner_id);
        encoder.encode(&self.role);
        encoder.encode(&self.platform_id);
        encoder.encode(&self.champion);
        encoder.encode(&self.queue);
        encoder.encode(&self.lane);
        encoder.encode(&self.timestamp);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        8usize * (4 + 4)
            + <i64 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.game_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.summoner_id)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.role)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.platform_id)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.champion)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.queue)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.lane)
            + <i64 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.timestamp)
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for MatchReference
where
    i64: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i64: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let game_id = decoder.try_decode::<i64>()?;
        let summoner_id = decoder.try_decode::<i32>()?;
        let role = decoder.try_decode::<Option<String>>()?;
        let platform_id = decoder.try_decode::<String>()?;
        let champion = decoder.try_decode::<String>()?;
        let queue = decoder.try_decode::<i32>()?;
        let lane = decoder.try_decode::<Option<String>>()?;
        let timestamp = decoder.try_decode::<i64>()?;
        ::std::result::Result::Ok(MatchReference {
            game_id,
            summoner_id,
            role,
            platform_id,
            champion,
            queue,
            lane,
            timestamp,
        })
    }
}
impl ::sqlx::Type<::sqlx::Postgres> for MatchReference {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("MatchReference")
    }
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for ParticipantStatsKills
where
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.participant_id);
        encoder.encode(&self.kills);
        encoder.encode(&self.deaths);
        encoder.encode(&self.assists);
        encoder.encode(&self.double_kills);
        encoder.encode(&self.triple_kills);
        encoder.encode(&self.quadra_kills);
        encoder.encode(&self.penta_kills);
        encoder.encode(&self.unreal_kills);
        encoder.encode(&self.largest_multi_kill);
        encoder.encode(&self.largest_killing_spree);
        encoder.encode(&self.killing_sprees);
        encoder.encode(&self.longest_time_spent_living);
        encoder.encode(&self.first_tower_kill);
        encoder.encode(&self.first_tower_assist);
        encoder.encode(&self.first_blood_kill);
        encoder.encode(&self.first_blood_assist);
        encoder.encode(&self.first_inhibitor_kill);
        encoder.encode(&self.first_inhibitor_assist);
        encoder.encode(&self.inhibitor_kills);
        encoder.encode(&self.turret_kills);
        encoder.encode(&self.neutral_minions_killed);
        encoder.encode(&self.neutral_minions_killed_enemy_jungle);
        encoder.encode(&self.neutral_minions_killed_team_jungle);
        encoder.encode(&self.total_minions_killed);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        25usize * (4 + 4)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.participant_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.deaths)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.assists)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.double_kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.triple_kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.quadra_kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.penta_kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.unreal_kills)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.largest_multi_kill)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.largest_killing_spree,
            )
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.killing_sprees)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.longest_time_spent_living,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_tower_kill,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_tower_assist,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_blood_kill,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_blood_assist,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_inhibitor_kill,
            )
            + <Option<bool> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.first_inhibitor_assist,
            )
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.inhibitor_kills,
            )
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.turret_kills,
            )
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.neutral_minions_killed,
            )
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.neutral_minions_killed_enemy_jungle,
            )
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.neutral_minions_killed_team_jungle,
            )
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.total_minions_killed,
            )
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for ParticipantStatsKills
where
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<bool>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<bool>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let participant_id = decoder.try_decode::<i32>()?;
        let kills = decoder.try_decode::<i32>()?;
        let deaths = decoder.try_decode::<i32>()?;
        let assists = decoder.try_decode::<i32>()?;
        let double_kills = decoder.try_decode::<i32>()?;
        let triple_kills = decoder.try_decode::<i32>()?;
        let quadra_kills = decoder.try_decode::<i32>()?;
        let penta_kills = decoder.try_decode::<i32>()?;
        let unreal_kills = decoder.try_decode::<i32>()?;
        let largest_multi_kill = decoder.try_decode::<i32>()?;
        let largest_killing_spree = decoder.try_decode::<i32>()?;
        let killing_sprees = decoder.try_decode::<i32>()?;
        let longest_time_spent_living = decoder.try_decode::<i32>()?;
        let first_tower_kill = decoder.try_decode::<Option<bool>>()?;
        let first_tower_assist = decoder.try_decode::<Option<bool>>()?;
        let first_blood_kill = decoder.try_decode::<Option<bool>>()?;
        let first_blood_assist = decoder.try_decode::<Option<bool>>()?;
        let first_inhibitor_kill = decoder.try_decode::<Option<bool>>()?;
        let first_inhibitor_assist = decoder.try_decode::<Option<bool>>()?;
        let inhibitor_kills = decoder.try_decode::<Option<i32>>()?;
        let turret_kills = decoder.try_decode::<Option<i32>>()?;
        let neutral_minions_killed = decoder.try_decode::<i32>()?;
        let neutral_minions_killed_enemy_jungle = decoder.try_decode::<Option<i32>>()?;
        let neutral_minions_killed_team_jungle = decoder.try_decode::<Option<i32>>()?;
        let total_minions_killed = decoder.try_decode::<i32>()?;
        ::std::result::Result::Ok(ParticipantStatsKills {
            participant_id,
            kills,
            deaths,
            assists,
            double_kills,
            triple_kills,
            quadra_kills,
            penta_kills,
            unreal_kills,
            largest_multi_kill,
            largest_killing_spree,
            killing_sprees,
            longest_time_spent_living,
            first_tower_kill,
            first_tower_assist,
            first_blood_kill,
            first_blood_assist,
            first_inhibitor_kill,
            first_inhibitor_assist,
            inhibitor_kills,
            turret_kills,
            neutral_minions_killed,
            neutral_minions_killed_enemy_jungle,
            neutral_minions_killed_team_jungle,
            total_minions_killed,
        })
    }
}
impl ::sqlx::Type<::sqlx::Postgres> for ParticipantStatsKills {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("ParticipantStatsKills")
    }
}
