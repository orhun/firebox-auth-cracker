use thiserror::Error as ThisError;

/// Implementation of the custom error types.
#[derive(ThisError, Debug)]
pub enum Error {
    /// Error that might occur while handling I/O operations.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;
