pub(crate) type AppResult<T> = Result<T, AppError>;


#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error(transparent)]
    StdIoError(std::io::Error),

    #[error(transparent)]
    SerdeJsonError(serde_json::Error),

    #[error("{0}")]
    ImpossibleState(String),
}