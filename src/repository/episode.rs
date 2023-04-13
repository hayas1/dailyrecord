use crate::{
    dev::episodes,
    domain::{class::id::Id, entity::episode::episode::Episode},
};

use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops::RangeBounds,
    sync::Mutex,
};

pub struct EpisodeRepository(pub(crate) HashMap<Id<Episode>, Episode>); // TODO Rc<RefCell<>>?
static CACHE: Lazy<Mutex<RefCell<EpisodeRepository>>> =
    Lazy::new(|| Mutex::new(RefCell::new(EpisodeRepository(episodes())))); // TODO empty

// TODO storage layer
impl EpisodeRepository {
    pub fn register(episode: Episode) -> anyhow::Result<()> {
        if let Ok(cache) = CACHE.lock() {
            Ok((*cache.borrow_mut()).insert(episode))
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }

    pub fn update(episode: Episode) -> anyhow::Result<()> {
        if let Ok(cache) = CACHE.lock() {
            if let Some(e) = (*cache.borrow_mut()).0.get_mut(&episode.id) {
                if &episode != e {
                    *e = episode
                } else {
                    anyhow::bail!("not changed");
                }
            }
            Ok(())
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }

    pub fn search(date: &NaiveDate) -> anyhow::Result<BTreeMap<NaiveDateTime, Episode>> {
        if let Ok(cache) = CACHE.lock() {
            let range = crate::supply::duration::range_date(date);
            Ok((*cache.borrow()).projection(range))
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }
}

impl EpisodeRepository {
    pub fn insert(&mut self, episode: Episode) {
        if let Some(_) = self.0.insert(episode.id.clone(), episode) {
            todo!("same multi key") // FIXME multi key
        }
    }
    pub fn remove(&mut self, episode: Episode) -> Option<Episode> {
        if let Some(found) = self.0.remove(&episode.id) {
            if found != episode {
                todo!("same multi key") // FIXME multi key
            } else {
                Some(found)
            }
        } else {
            None
        }
    }
    /// *O(n)*, regardless range
    pub fn projection<R: RangeBounds<NaiveDateTime>>(&self, range: R) -> BTreeMap<NaiveDateTime, Episode> {
        let bt: BTreeMap<_, _> = self.0.values().map(|e| (e.start().naive_local(), e.clone())).collect();
        bt.range(range).map(|(t, e)| (t.clone(), e.clone())).collect()
    }
}
