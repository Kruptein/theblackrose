#[derive(Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Summoner {
    #[serde(skip)]
    pub id: i32,
    pub account_id: String,
    pub profile_icon_id: i32,
    pub revision_date: Option<i64>,
    pub name: String,
    pub summoner_id: Option<String>,
    pub puuid: Option<String>,
    pub summoner_level: Option<i64>,
    pub last_match_query_time: Option<chrono::NaiveDateTime>,
    pub update_in_progress: bool,
}

impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Summoner
where
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i64>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i64>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i64>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i64>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<chrono::NaiveDateTime>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<chrono::NaiveDateTime>: ::sqlx::types::Type<::sqlx::Postgres>,
    bool: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    bool: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.account_id);
        encoder.encode(&self.profile_icon_id);
        encoder.encode(&self.revision_date);
        encoder.encode(&self.name);
        encoder.encode(&self.summoner_id);
        encoder.encode(&self.puuid);
        encoder.encode(&self.summoner_level);
        encoder.encode(&self.last_match_query_time);
        encoder.encode(&self.update_in_progress);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        10usize * (4 + 4)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.id)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.account_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.profile_icon_id)
            + <Option<i64> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.revision_date,
            )
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.name)
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.summoner_id,
            )
            + <Option<String> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.puuid)
            + <Option<i64> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.summoner_level,
            )
            + <Option<chrono::NaiveDateTime> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.last_match_query_time,
            )
            + <bool as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(
                &self.update_in_progress,
            )
    }
}
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Summoner
where
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i64>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i64>: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i64>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<i64>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<chrono::NaiveDateTime>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<chrono::NaiveDateTime>: ::sqlx::types::Type<::sqlx::Postgres>,
    bool: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    bool: ::sqlx::types::Type<::sqlx::Postgres>,
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
        let account_id = decoder.try_decode::<String>()?;
        let profile_icon_id = decoder.try_decode::<i32>()?;
        let revision_date = decoder.try_decode::<Option<i64>>()?;
        let name = decoder.try_decode::<String>()?;
        let summoner_id = decoder.try_decode::<Option<String>>()?;
        let puuid = decoder.try_decode::<Option<String>>()?;
        let summoner_level = decoder.try_decode::<Option<i64>>()?;
        let last_match_query_time = decoder.try_decode::<Option<chrono::NaiveDateTime>>()?;
        let update_in_progress = decoder.try_decode::<bool>()?;
        ::std::result::Result::Ok(Summoner {
            id,
            account_id,
            profile_icon_id,
            revision_date,
            name,
            summoner_id,
            puuid,
            summoner_level,
            last_match_query_time,
            update_in_progress,
        })
    }
}
impl ::sqlx::Type<::sqlx::Postgres> for Summoner {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Summoner")
    }
}
