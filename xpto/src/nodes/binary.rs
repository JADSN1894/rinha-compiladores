use serde::Deserialize;

use super::{binary_op::BinaryOp, location::Location, term::Term};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
    location: Location,
}

impl Binary {
    pub(crate) fn lhs(&self) -> &Term {
        self.lhs.as_ref()
    }

    pub(crate) fn op(&self) -> &BinaryOp {
        &self.op
    }

    pub(crate) fn rhs(&self) -> &Term {
        self.rhs.as_ref()
    }

    pub(crate) fn location(&self) -> &Location {
        &self.location
    }
}
