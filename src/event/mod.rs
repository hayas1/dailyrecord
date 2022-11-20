pub mod category;
pub mod plan;

use serde::{Deserialize, Serialize};

use self::{category::Category, plan::Plan};

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Event {
    pub title: String,
    pub description: String,
    pub category: Option<Category>,
    pub plan: Plan,
}

impl Event {
    pub fn new(title: String, description: String, category: Option<Category>, plan: Plan) -> Self {
        Self {
            title,
            description,
            category,
            plan,
        }
    }
}
