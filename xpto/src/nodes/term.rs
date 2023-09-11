use serde::Deserialize;

use super::{
    binary::Binary, bool::Bool, call::Call, function::Function, ifi::If, int::Int, leti::Let,
    print::Print, str::Str, var::Var,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub(crate) enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
    Binary(Binary),
    If(If),
    Let(Let),
    Var(Var),
    Function(Function),
    Call(Call),
}
