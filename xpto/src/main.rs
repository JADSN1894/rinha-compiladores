use std::{
    collections::HashMap,
    fs,
    io::{stdin, Read},
};

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error(transparent)]
    StdIoError(std::io::Error),

    #[error(transparent)]
    SerdeJsonError(serde_json::Error),

    #[error("{0}")]
    ImpossibleState(String),
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Deserialize)]
struct Print {
    value: Box<Term>,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Str {
    value: String,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Int {
    value: i32,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Bool {
    value: bool,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Binary {
    lhs: Box<Term>,
    op: BinaryOp,
    rhs: Box<Term>,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct If {
    condition: Box<Term>,
    then: Box<Term>,
    otherwise: Box<Term>,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Parameter {
    text: String,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Let {
    name: Parameter,
    value: Box<Term>,
    next: Box<Term>,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Var {
    text: String,
    location: Location,
}

#[derive(Debug, Deserialize)]
enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
    Binary(Binary),
    If(If),
    Let(Let),
    Var(Var),
}

#[derive(Debug, Deserialize)]
struct File {
    name: String,
    expression: Term,
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Location {
    start: usize,
    end: usize,
    filename: String,
}

#[derive(Debug, Clone, Deserialize)]
enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

type Scope = HashMap<String, Val>;

impl Val {
    fn try_from(term: Term, scope: &mut Scope) -> AppResult<Self> {
        match term {
            Term::Int(number) => Ok(Val::Int(number.value)),
            Term::Str(text) => Ok(Val::Str(text.value)),
            Term::Bool(bool) => Ok(Val::Bool(bool.value)),
            Term::Print(print) => {
                let val = eval(*print.value, scope)?;
                match val {
                    Val::Int(n) => {
                        print!("{n}");
                        Ok(Val::Int(n))
                    }
                    Val::Bool(b) => {
                        print!("{b}");
                        Ok(Val::Bool(b))
                    }
                    Val::Str(s) => {
                        print!("{s}");
                        Ok(Val::Str(s))
                    }
                    Val::Void => Ok(Val::Void),
                }
            }
            Term::Binary(binary) => match binary.op {
                BinaryOp::Add => {
                    let lhs = eval(*binary.lhs, scope)?;
                    let rhs = eval(*binary.rhs, scope)?;

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
                    let lhs = eval(*binary.lhs, scope)?;
                    let rhs = eval(*binary.rhs, scope)?;

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
            },
            Term::If(ifi) => match eval(*ifi.condition, scope)? {
                Val::Bool(true) => eval(*ifi.then, scope),
                Val::Bool(false) => eval(*ifi.otherwise, scope),
                val => Err(AppError::ImpossibleState(format!("Is not bool: {val:?}"))),
            },
            Term::Let(leti) => {
                let name = leti.name.text;
                let value = eval(*leti.value, scope)?;

                scope.insert(name, value);

                eval(*leti.next, scope)
            }
            Term::Var(var) => {
                match scope.get(&var.text) {
                    Some(val) => Ok(val.clone()),
                    None => Err(AppError::ImpossibleState("Variável não encontrada".into())),
                }
            },
        }
    }
}

fn eval(term: Term, scope: &mut Scope) -> AppResult<Val> {
    Val::try_from(term, scope)
}

fn main() -> AppResult<()> {
    let mut program = String::new();
    stdin()
        .lock()
        .read_to_string(&mut program)
        .map_err(|error| AppError::StdIoError(error))?;

    let program =
        serde_json::from_str::<File>(&program).map_err(|error| AppError::SerdeJsonError(error))?;

    let term = program.expression;

    let mut scope = Scope::default();
    eval(term, &mut scope)?;

    Ok(())
}
