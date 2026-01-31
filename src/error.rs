use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Command(String),
    Validation(String),
    NotFound(String),
    Permission(String),
    // Already(String),
    Config(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Command(msg) => write!(f, "Command failed: {}", msg),
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Permission(msg) => write!(f, "Permission denied: {}", msg),
            // Error::Already(msg) => write!(f, "Already exists: {}", msg),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::Command(error.to_string())
    }
}