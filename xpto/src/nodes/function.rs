use serde::Deserialize;

use super::{location::Location, parameter::Parameter, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Function {
    parameters: Vec<Parameter>,
    value: Box<Term>,
    location: Location,
}

impl Function {
    pub(crate) fn parameters(&self) -> &[Parameter] {
        self.parameters.as_ref()
    }

    pub(crate) fn value(&self) -> &Term {
        self.value.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
