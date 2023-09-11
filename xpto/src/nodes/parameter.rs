use serde::Deserialize;

use super::location::Location;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Parameter {
    text: String,
    location: Location,
}

impl Parameter {
    pub(crate) fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
