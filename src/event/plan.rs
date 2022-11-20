use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Plan {
    pub start: DateTime<Local>,
    #[serde(with = "duration_millis")]
    pub duration: Duration,
    pub repeat: bool, // TODO implement
}

mod duration_millis {
    use super::*;

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
}

impl Plan {
    pub fn new(start: DateTime<Local>, duration: Duration, repeat: bool) -> Self {
        Self {
            start,
            duration,
            repeat,
        }
    }
}
