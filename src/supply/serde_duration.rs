use chrono::Duration;
use serde::{Deserialize, Serialize};

pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    duration.num_milliseconds().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let duration = <i64>::deserialize(deserializer)?;
    Ok(Duration::milliseconds(duration))
}
