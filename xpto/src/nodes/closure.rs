use serde::Deserialize;

use super::{parameter::Parameter, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Closure {
    body: Term,
    parameters: Vec<Parameter>,
}

impl Closure {
    pub(crate) fn new(body: Term, parameters: Vec<Parameter>) -> Self {
        Self { body, parameters }
    }

    pub(crate) fn body(&self) -> &Term {
        &self.body
    }

    pub(crate) fn parameters(&self) -> &[Parameter] {
        self.parameters.as_ref()
    }
}
