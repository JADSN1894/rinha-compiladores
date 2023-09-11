use serde::Deserialize;

use super::{location::Location, parameter::Parameter, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
    location: Location,
}

impl Let {
    pub(crate) fn name(&self) -> &Parameter {
        &self.name
    }

    pub(crate) fn value(&self) -> &Term {
        self.value.as_ref()
    }

    pub(crate) fn next(&self) -> &Term {
        self.next.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
