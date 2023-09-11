use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}
