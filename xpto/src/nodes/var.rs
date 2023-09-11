use serde::Deserialize;

use super::location::Location;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Var {
    text: String,
    location: Location,
}

impl Var {
    pub(crate) fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
