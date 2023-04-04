use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Category {
    pub fn new(id: String, title: String, description: String) -> Self {
        Self { id, title, description }
    }
}
