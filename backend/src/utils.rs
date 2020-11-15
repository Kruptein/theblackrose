// todo fix the nano second part :4
pub fn millis_to_chrono(millis: i64) -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from_timestamp(millis / 1000, 0)
}
