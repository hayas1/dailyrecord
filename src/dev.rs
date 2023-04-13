use std::collections::HashMap;

use crate::domain::{
    class::id::Id,
    entity::episode::{episode::Episode, plan::Plan},
};

use chrono::{Duration, Local};

pub fn episodes() -> HashMap<Id<Episode>, Episode> {
    let start = crate::supply::naive_today().and_hms_opt(13, 0, 0).unwrap();

    vec![
        Episode::new(
            "title".to_string(),
            "description".to_string(),
            None,
            Plan::new(start.and_local_timezone(Local).unwrap(), Duration::hours(1) + Duration::minutes(30), false),
        ),
        Episode::new(
            "epi2".to_string(),
            "episode's description".to_string(),
            None,
            Plan::new(
                start.and_local_timezone(Local).unwrap() + Duration::days(1) + Duration::hours(1),
                Duration::hours(1),
                false,
            ),
        ),
    ]
    .into_iter()
    .map(|e| (e.id.clone(), e))
    .collect() // TODO from storage layer
}
