use serde::Deserialize;

use super::{location::Location, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
    location: Location,
}

impl If {
    pub(crate) fn condition(&self) -> &Term {
        self.condition.as_ref()
    }

    pub(crate) fn then(&self) -> &Term {
        self.then.as_ref()
    }

    pub(crate) fn otherwise(&self) -> &Term {
        self.otherwise.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
