pub mod deserialize_nullable_string {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<String> = Deserialize::deserialize(deserializer)?;
        Ok(value.unwrap_or_default())
    }
}

pub mod deserialize_nullable_vec {

    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let value: Option<Vec<T>> = Deserialize::deserialize(deserializer)?;
        Ok(value.unwrap_or_default())
    }
}

pub mod deserialize_nullable_i64 {
    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Option<i64> = Deserialize::deserialize(deserializer)?;
        Ok(value.unwrap_or_default())
    }
}

pub mod deserialize_date_time_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        if let Some(s) = s {
            if s.is_empty() {
                Ok(None)
            } else {
                match DateTime::parse_from_rfc3339(&s) {
                    Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
                    Err(_) => Err(serde::de::Error::custom(format!(
                        "Invalid datetime format: {s}",
                    ))),
                }
            }
        } else {
            Ok(None)
        }
    }
}
