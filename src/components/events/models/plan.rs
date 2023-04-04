use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Plan {
    pub start: DateTime<Local>,
    #[serde(with = "crate::supply::serde_duration")]
    pub duration: Duration,
    pub repeat: bool, // TODO implement
}

impl Plan {
    pub fn new(start: DateTime<Local>, duration: Duration, repeat: bool) -> Self {
        Self { start, duration, repeat }
    }
}
