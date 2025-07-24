pub mod deserialize_date_time_format {
    use serde::Serializer;
    pub fn serialize<S>(
        date: &Option<chrono::DateTime<chrono::Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}
