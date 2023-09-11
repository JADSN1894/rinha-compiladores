use serde::Deserialize;

use super::location::Location;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Int {
    value: i32,
    location: Location,
}

impl Int {
    pub(crate) fn value(&self) -> i32 {
        self.value
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
