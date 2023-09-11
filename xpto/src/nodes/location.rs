use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Location {
    start: usize,
    end: usize,
    filename: String,
}
