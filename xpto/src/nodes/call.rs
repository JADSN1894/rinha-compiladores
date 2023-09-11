use serde::Deserialize;

use super::{location::Location, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Call {
    callee: Box<Term>,
    arguments: Vec<Term>,
    location: Location,
}

impl Call {
    pub(crate) fn callee(&self) -> &Term {
        self.callee.as_ref()
    }

    pub(crate) fn arguments(&self) -> &[Term] {
        self.arguments.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
