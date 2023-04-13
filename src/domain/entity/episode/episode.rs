use crate::{domain::class::id::Id, repository::episode::EpisodeRepository};

use super::{category::Category, schedule::Schedule};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Episode {
    pub id: Id<Self>,
    pub title: String,
    pub description: String,
    pub category: Option<Category>,
    pub schedule: Schedule,
}

impl Episode {
    pub fn new(title: String, description: String, category: Option<Category>, schedule: Schedule) -> Self {
        let id = Id::new();
        Self { id, title, description, category, schedule }
    }

    pub fn start(&self) -> DateTime<Local> {
        self.schedule.start
    }
    pub fn end(&self) -> DateTime<Local> {
        self.schedule.start + self.schedule.duration
    }
    pub fn range(&self) -> std::ops::RangeInclusive<DateTime<Local>> {
        self.start()..=self.end()
    }
}

impl Episode {
    pub fn save_with<F: Fn(&mut Self)>(&self, f: F) -> anyhow::Result<()> {
        let mut modified = self.clone();
        f(&mut modified);
        anyhow::ensure!(self.id == modified.id); // FIXME type level id immutability

        gloo::console::log!("save", format!("{self:?}"), "->", format!("{modified:?}"));
        EpisodeRepository::update(modified)?;
        Ok(())
    }
}
