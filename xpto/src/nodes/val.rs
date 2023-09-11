use std::collections::HashMap;

use serde::Deserialize;

use crate::error::{AppError, AppResult};

use super::{binary_op::BinaryOp, closure::Closure, term::Term};

pub(crate) type Scope = HashMap<String, Val>;

#[derive(Debug, Clone, Deserialize)]
pub(crate) enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
    // TODO: Mudar para estrutura Closure
    Closure(Closure),
}

impl Val {
    pub(crate) fn try_from(term: Term, scope: &mut Scope) -> AppResult<Self> {
        match term {
            Term::Int(number) => Ok(Val::Int(number.value())),
            Term::Str(text) => Ok(Val::Str(text.value().into())),
            Term::Bool(bool) => Ok(Val::Bool(bool.value())),
            Term::Print(print) => {
                let print = print.clone();
                let val = eval(print.value().clone(), scope)?;
                match val {
                    Val::Int(n) => Ok(Val::Int(n)),
                    Val::Bool(b) => Ok(Val::Bool(b)),
                    Val::Str(s) => Ok(Val::Str(s)),
                    Val::Void => Ok(Val::Void),
                    Val::Closure(closure) => Ok(Val::Closure(closure)),
                }
            }
            Term::Binary(binary) => match binary.op() {
                BinaryOp::Add => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Int(a + b)),
                        (Val::Str(a), Val::Int(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Int(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Str(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Sub => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Int(a - b)),
                        (Val::Str(a), Val::Int(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Int(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Str(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Lt => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a < b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Mul => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Int(a * b)),
                        (Val::Str(a), Val::Int(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Int(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (Val::Str(a), Val::Str(b)) => Ok(Val::Str(format!("{a}{b}"))),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Eq => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a == b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Neq => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a != b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Gt => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a > b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Lte => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a <= b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Gte => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Int(a), Val::Int(b)) => Ok(Val::Bool(a >= b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::And => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Bool(a), Val::Bool(b)) => Ok(Val::Bool(a && b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
                BinaryOp::Or => {
                    let lhs = eval(binary.lhs().clone(), scope)?;
                    let rhs = eval(binary.rhs().clone(), scope)?;

                    match (lhs, rhs) {
                        (Val::Bool(a), Val::Bool(b)) => Ok(Val::Bool(a || b)),
                        (a, b) => Err(AppError::ImpossibleState(format!(
                            "{a:?}{b:?} does not match any criteria",
                        ))),
                    }
                }
            },
            Term::If(ifi) => match eval(ifi.condition().clone(), scope)? {
                Val::Bool(true) => eval(ifi.then().clone(), scope),
                Val::Bool(false) => eval(ifi.otherwise().clone(), scope),
                val => Err(AppError::ImpossibleState(format!("Is not bool: {val:?}"))),
            },
            Term::Let(leti) => {
                let name = leti.name().text();
                let value = eval(leti.value().clone(), scope)?;

                scope.insert(name.into(), value);

                eval(leti.next().clone(), scope)
            }
            Term::Var(var) => match scope.get(var.text()) {
                Some(val) => Ok(val.clone()),
                None => Err(AppError::ImpossibleState("Variável não encontrada".into())),
            },
            Term::Function(func) => Ok(Val::Closure(Closure::new(
                func.value().clone(),
                func.parameters().to_vec(),
            ))),
            Term::Call(call) => {
                match eval(call.callee().clone(), scope)? {
                    Val::Closure(closure) => {
                        let mut new_scope = scope.clone();

                        //*  Juntar os parâmetros com os argumentos do call
                        for (param, arg) in closure
                            .parameters()
                            .into_iter()
                            .zip(call.arguments().to_vec())
                        {
                            new_scope.insert(param.text().into(), eval(arg, scope)?);
                        }

                        eval(closure.body().to_owned(), &mut new_scope)
                    }
                    val => Err(AppError::ImpossibleState(format!(
                        "{val:?} is not a funtion"
                    ))),
                }
            }
        }
    }
}

pub(crate) fn eval(term: Term, scope: &mut Scope) -> AppResult<Val> {
    Val::try_from(term, scope)
}
