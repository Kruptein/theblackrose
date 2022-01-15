use super::summoners::Summoner;

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MatchFeedParticipant {
    pub summoner: Option<Summoner>,
    pub general: ParticipantStatsGeneral,
    pub items: ParticipantStatsItems,
    pub kda: ParticipantStatsKda,
    pub progress: ParticipantStatsProgress,
    pub spells: ParticipantStatsSpells,
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
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: Option<String>,
    pub game_start_timestamp: Option<i64>,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i16,
    pub platform_id: String,
    pub queue_id: i16,
    pub season_id: i16,
    pub tournament_code: Option<String>,
    // meta
    pub data_version: String,
    pub match_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamStats {
    pub game_id: i64,
    pub team_id: i16,
    pub tower_kills: i64,
    pub rift_herald_kills: i64,
    pub first_blood: bool,
    pub inhibitor_kills: i64,
    pub first_baron: bool,
    pub first_dragon: bool,
    pub dragon_kills: i64,
    pub baron_kills: i64,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub first_rift_herald: bool,
    pub win: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsAccount {
    pub game_id: i64,
    pub summoner_id: i32,

    pub participant_id: i16,
    pub profile_icon: Option<i16>,
    pub puuid: Option<String>,
    pub riot_id_name: Option<String>,
    pub riot_id_tagline: Option<String>,
    pub summoner_level: Option<i16>,
    pub summoner_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsDamage {
    pub game_id: i64,
    pub summoner_id: i32,

    pub damage_dealt_to_buildings: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub damage_self_mitigated: i64,

    pub largest_critical_strike: i32,

    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magical_damage_taken: i64,

    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,

    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_taken: i64,

    pub total_heal: i64,
    pub total_heals_on_teammates: i64,
    pub total_units_healed: i64,

    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsGeneral {
    pub game_id: i64,
    pub summoner_id: i32,

    pub champion_id: Option<i16>,
    pub game_ended_in_early_surrender: Option<bool>,
    pub game_ended_in_surrender: Option<bool>,
    pub individual_position: Option<String>,
    pub team_early_surrendered: Option<bool>,
    pub team_id: i16,
    pub team_position: Option<String>,
    pub win: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsItems {
    pub game_id: i64,
    pub summoner_id: i32,

    pub consumables_purchased: Option<i16>,
    pub detector_wards_placed: Option<i16>,

    pub item0: Option<i16>,
    pub item1: Option<i16>,
    pub item2: Option<i16>,
    pub item3: Option<i16>,
    pub item4: Option<i16>,
    pub item5: Option<i16>,
    pub item6: Option<i16>,

    pub items_purches: Option<i16>,
    pub sight_wards_bought_in_game: Option<i16>,
    pub vision_wards_bought_in_game: Option<i16>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsKda {
    pub game_id: i64,
    pub summoner_id: i32,

    pub assists: Option<i16>,
    pub deaths: Option<i16>,
    pub double_kills: Option<i16>,
    pub dragon_kills: Option<i16>,
    pub first_blood_assist: Option<bool>,
    pub first_blood_kill: Option<bool>,
    pub first_tower_assist: Option<bool>,
    pub first_tower_kill: Option<bool>,
    pub inhibitor_kills: Option<i16>,
    pub inhibitor_takedowns: Option<i16>,
    pub inhibitors_lost: Option<i16>,
    pub killing_sprees: Option<i16>,
    pub kills: Option<i16>,
    pub largest_killing_spree: Option<i16>,
    pub largest_multi_kill: Option<i16>,
    pub neutral_minions_killed: Option<i16>,
    pub nexus_kills: Option<i16>,
    pub nexus_takedowns: Option<i16>,
    pub nexus_lost: Option<i16>,
    pub objectives_stolen: Option<i16>,
    pub objectives_stolen_assists: Option<i16>,
    pub penta_kills: Option<i16>,
    pub quadra_kills: Option<i16>,
    pub total_minions_killed: Option<i16>,
    pub triple_kills: Option<i16>,
    pub turret_kills: Option<i16>,
    pub turret_takedowns: Option<i16>,
    pub turrets_lost: Option<i16>,
    pub unreal_kills: Option<i16>,
    pub wards_killed: Option<i16>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsProgress {
    pub game_id: i64,
    pub summoner_id: i32,

    pub bounty_level: Option<i16>,
    pub champ_experience: Option<i32>,
    pub champ_level: Option<i16>,
    pub champion_transform: Option<i16>,
    pub gold_earned: Option<i32>,
    pub gold_spent: Option<i32>,
    pub longest_time_spent_living: Option<i16>,
    pub time_c_cing_others: Option<i16>,
    pub time_played: Option<i16>,
    pub total_time_cc_dealt: Option<i32>,
    pub total_time_spent_dead: Option<i16>,
    pub vision_score: Option<i16>,
    pub wards_placed: Option<i16>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ParticipantStatsSpells {
    pub game_id: i64,
    pub summoner_id: i32,

    pub spell1_casts: Option<i16>,
    pub spell2_casts: Option<i16>,
    pub spell3_casts: Option<i16>,
    pub spell4_casts: Option<i16>,
    pub summoner1_casts: Option<i16>,
    pub summoner1_id: Option<i16>,
    pub summoner2_casts: Option<i16>,
    pub summoner2_id: Option<i16>,
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

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for ParticipantStatsGeneral
where
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
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
        let champion_id = decoder.try_decode::<Option<i16>>()?;
        let game_ended_in_early_surrender = decoder.try_decode::<Option<bool>>()?;
        let game_ended_in_surrender = decoder.try_decode::<Option<bool>>()?;
        let individual_position = decoder.try_decode::<Option<String>>()?;
        let team_early_surrendered = decoder.try_decode::<Option<bool>>()?;
        let team_id = decoder.try_decode::<i16>()?;
        let team_position = decoder.try_decode::<Option<String>>()?;
        let win = decoder.try_decode::<bool>()?;
        ::std::result::Result::Ok(ParticipantStatsGeneral {
            game_id,
            summoner_id,
            champion_id,
            game_ended_in_early_surrender,
            game_ended_in_surrender,
            individual_position,
            team_early_surrendered,
            team_id,
            team_position,
            win,
        })
    }
}

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for ParticipantStatsProgress
where
    Option<i32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
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
        let bounty_level = decoder.try_decode::<Option<i16>>()?;
        let champ_experience = decoder.try_decode::<Option<i32>>()?;
        let champ_level = decoder.try_decode::<Option<i16>>()?;
        let champion_transform = decoder.try_decode::<Option<i16>>()?;
        let gold_earned = decoder.try_decode::<Option<i32>>()?;
        let gold_spent = decoder.try_decode::<Option<i32>>()?;
        let longest_time_spent_living = decoder.try_decode::<Option<i16>>()?;
        let time_c_cing_others = decoder.try_decode::<Option<i16>>()?;
        let time_played = decoder.try_decode::<Option<i16>>()?;
        let total_time_cc_dealt = decoder.try_decode::<Option<i32>>()?;
        let total_time_spent_dead = decoder.try_decode::<Option<i16>>()?;
        let vision_score = decoder.try_decode::<Option<i16>>()?;
        let wards_placed = decoder.try_decode::<Option<i16>>()?;

        ::std::result::Result::Ok(ParticipantStatsProgress {
            game_id,
            summoner_id,
            bounty_level,
            champ_experience,
            champ_level,
            champion_transform,
            gold_earned,
            gold_spent,
            longest_time_spent_living,
            time_c_cing_others,
            time_played,
            total_time_cc_dealt,
            total_time_spent_dead,
            vision_score,
            wards_placed,
        })
    }
}
