use miette::Diagnostic;
use thiserror::Error;

pub type ComojiResult<T> = Result<T, ComojiError>;

/// Global error collector
#[derive(Debug, Error, Diagnostic)]
pub enum ComojiError {
    #[error(transparent)]
    Io(std::io::Error),
    #[error(transparent)]
    Dialoguer(dialoguer::Error),
}

impl From<std::io::Error> for ComojiError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<dialoguer::Error> for ComojiError {
    fn from(value: dialoguer::Error) -> Self {
        Self::Dialoguer(value)
    }
}
