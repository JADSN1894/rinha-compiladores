use serde::Deserialize;

use super::{location::Location, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Print {
    value: Box<Term>,
    location: Location,
}

impl Print {
    pub(crate) fn value(&self) -> &Term {
        self.value.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
