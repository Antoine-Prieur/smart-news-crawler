use chrono::{DateTime, Utc};
use serde::{Deserialize, Serializer};

pub fn serialize_mongo_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(1))?;
    map.serialize_entry("$date", &date.to_rfc3339())?;
    map.end()
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    match DateTime::parse_from_rfc3339(&s) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(e) => Err(serde::de::Error::custom(format!(
            "Failed to parse datetime '{}': {}",
            s, e
        ))),
    }
}
