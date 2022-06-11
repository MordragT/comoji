pub type ComojiResult<T> = Result<T, ComojiError>;

/// Global error collector
#[derive(Debug)]
pub enum ComojiError {
    IOError(std::io::Error),
    ConfyError(confy::ConfyError),
    Other(String),
}

impl From<std::io::Error> for ComojiError {
    fn from(err: std::io::Error) -> Self {
        ComojiError::IOError(err)
    }
}

impl From<confy::ConfyError> for ComojiError {
    fn from(err: confy::ConfyError) -> Self {
        ComojiError::ConfyError(err)
    }
}
