use crate::{
    dev::events,
    domain::{class::id::Id, entity::event::event::Event},
};

use chrono::{NaiveDate, NaiveDateTime};
use once_cell::sync::Lazy;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops::RangeBounds,
    sync::Mutex,
};

pub struct EventRepository(pub(crate) HashMap<Id<Event>, Event>); // TODO Rc<RefCell<>>?
static CACHE: Lazy<Mutex<RefCell<EventRepository>>> = Lazy::new(|| Mutex::new(RefCell::new(EventRepository(events())))); // TODO empty

// TODO storage layer
impl EventRepository {
    pub fn register(event: Event) -> anyhow::Result<()> {
        if let Ok(cache) = CACHE.lock() {
            Ok((*cache.borrow_mut()).insert(event))
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }

    pub fn update(event: Event) -> anyhow::Result<()> {
        if let Ok(cache) = CACHE.lock() {
            if let Some(e) = (*cache.borrow_mut()).0.get_mut(&event.id) {
                if &event != e {
                    *e = event
                } else {
                    anyhow::bail!("not changed");
                }
            }
            Ok(())
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }

    pub fn search(date: &NaiveDate) -> anyhow::Result<BTreeMap<NaiveDateTime, Event>> {
        if let Ok(cache) = CACHE.lock() {
            let range = crate::supply::duration::range_date(date);
            Ok((*cache.borrow()).projection(range))
        } else {
            anyhow::bail!("cannot get cache lock")
        }
    }
}

impl EventRepository {
    pub fn insert(&mut self, event: Event) {
        if let Some(_) = self.0.insert(event.id.clone(), event) {
            todo!("same multi key") // FIXME multi key
        }
    }
    pub fn remove(&mut self, event: Event) -> Option<Event> {
        if let Some(found) = self.0.remove(&event.id) {
            if found != event {
                todo!("same multi key") // FIXME multi key
            } else {
                Some(found)
            }
        } else {
            None
        }
    }
    /// *O(n)*, regardless range
    pub fn projection<R: RangeBounds<NaiveDateTime>>(&self, range: R) -> BTreeMap<NaiveDateTime, Event> {
        let bt: BTreeMap<_, _> = self.0.values().map(|e| (e.start().naive_local(), e.clone())).collect();
        bt.range(range).map(|(t, e)| (t.clone(), e.clone())).collect()
    }
}
