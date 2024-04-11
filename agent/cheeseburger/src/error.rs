use std::fmt::{Debug, Formatter, Display};

pub enum CheeseburgerError {
    ConfigLoadError,
    ConnectionError(String),
    InvalidQueryCodeError(String),
    StreamError
}

impl Display for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CheeseburgerError::ConfigLoadError => write!(f, "Failed to load config"),
            CheeseburgerError::ConnectionError(s)
                => write!(f, "Failed to connect to server: {}", s),
            CheeseburgerError::InvalidQueryCodeError(s)
                => write!(f, "Invalid query code: {}", s),
            CheeseburgerError::StreamError => write!(f, "Stream error"),
        }
    }
}

impl Debug for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheeseburgerError::ConfigLoadError => write!(f, "Failed to load config"),
            CheeseburgerError::ConnectionError(s)
                => write!(f, "Failed to connect to server: {}", s),
            CheeseburgerError::InvalidQueryCodeError(s)
                => write!(f, "Invalid query code: {}", s),
            CheeseburgerError::StreamError => write!(f, "Stream error"),
        }
    }
}

impl std::error::Error for CheeseburgerError {}
