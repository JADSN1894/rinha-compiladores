use serde::Deserialize;

use super::location::Location;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Str {
    value: String,
    location: Location,
}

impl Str {
    pub(crate) fn value(&self) -> &str {
        self.value.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
