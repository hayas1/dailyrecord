use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::{category::Category, plan::Plan};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub category: Option<Category>,
    pub plan: Plan,
}

impl Event {
    pub fn new(title: String, description: String, category: Option<Category>, plan: Plan) -> Self {
        Self { title, description, category, plan }
    }

    pub fn start(&self) -> DateTime<Local> {
        self.plan.start
    }
    pub fn end(&self) -> DateTime<Local> {
        self.plan.start + self.plan.duration
    }
    pub fn range(&self) -> std::ops::RangeInclusive<DateTime<Local>> {
        self.start()..=self.end()
    }
}
