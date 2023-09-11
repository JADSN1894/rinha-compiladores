use serde::Deserialize;

use super::{location::Location, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct File {
    name: String,
    expression: Term,
    location: Location,
}

impl File {
    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn expression(&self) -> &Term {
        &self.expression
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
