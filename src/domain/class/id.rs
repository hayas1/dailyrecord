use serde::{Deserialize, Serialize};
use uuid::Uuid;

// reference https://github.com/SeaQL/sea-orm/issues/402
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Uuid", into = "Uuid")]
pub struct Id<T>(Uuid, #[serde(skip_serializing)] std::marker::PhantomData<T>);

impl<T> Id<T> {
    pub fn new() -> Self {
        // Self::from(Uuid::from(ulid::Ulid::new()))
        Self::from(Uuid::new_v4())
    }
    pub fn identifier(&self) -> Uuid {
        self.0
    }
}
impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Self {
        Self(id, std::marker::PhantomData)
    }
}
impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Self {
        id.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::from(self.identifier())
    }
}
impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

impl<T> std::str::FromStr for Id<T> {
    type Err = <Uuid as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(Uuid::from_str(s)?))
    }
}
