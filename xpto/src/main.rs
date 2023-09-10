use std::{
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
enum BinaryOp {
    Add,
    Sub,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
    Binary(Binary),
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

#[derive(Debug, Deserialize)]
enum Val {
    Void,
    Int(i32),
    Bool(bool),
    Str(String),
}

fn eval(term: Term) -> AppResult<Val> {
    match term {
        Term::Int(number) => Ok(Val::Int(number.value)),
        Term::Str(text) => Ok(Val::Str(text.value)),
        Term::Print(print) => {
            let val = eval(*print.value)?;
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
                let lhs = eval(*binary.lhs)?;
                let rhs = eval(*binary.rhs)?;

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Ok(Val::Int(a + b)),
                    (a, b) => Err(AppError::ImpossibleState(format!(
                        "both vals {a:?} and {b:?} must be numbers",
                    ))),
                }
            }
            BinaryOp::Sub => {
                let lhs = eval(*binary.lhs)?;
                let rhs = eval(*binary.rhs)?;

                match (lhs, rhs) {
                    (Val::Int(a), Val::Int(b)) => Ok(Val::Int(a - b)),
                    (a, b) => Err(AppError::ImpossibleState(format!(
                        "both vals {a:?} and {b:?} must be numbers",
                    ))),
                }
            }
        },
    }
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
    eval(term)?;

    Ok(())
}
