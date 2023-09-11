use serde::Deserialize;

use super::location::Location;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Bool {
    value: bool,
    location: Location,
}

impl Bool {
    pub(crate) fn value(&self) -> bool {
        self.value
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
