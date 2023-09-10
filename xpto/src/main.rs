use std::fs;

use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error(transparent)]
    StdIoError(std::io::Error),

    #[error(transparent)]
    SerdeJsonError(serde_json::Error),
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
#[serde(tag = "kind")]
enum Term {
    Int(Int),
    Str(Str),
    Print(Print),
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

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(text) => Val::Str(text.value),
        Term::Print(print) => {
            let val = eval(*print.value);
            match val {
                Val::Int(n) => print!("{n}"),
                Val::Bool(b) => print!("{b}"),
                Val::Str(s) => print!("{s}"),
                _ => panic!("Valor não suportado"),
            };

            Val::Void
        }
    }
}

fn main() -> AppResult<()> {
    let program =
        fs::read_to_string("./examples/hello.json").map_err(|error| AppError::StdIoError(error))?;
    let program =
        serde_json::from_str::<File>(&program).map_err(|error| AppError::SerdeJsonError(error))?;
    let term = program.expression;
    eval(term);

    Ok(())
}
