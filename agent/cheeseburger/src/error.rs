use std::fmt::{Debug, Formatter, Display};

pub enum CheeseburgerError {
    ConfigLoadError,
    ConnectionError(String),
}

impl Display for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CheeseburgerError::ConfigLoadError => write!(f, "Failed to load config"),
            CheeseburgerError::ConnectionError(s)
                => write!(f, "Failed to connect to server: {}", s),
        }
    }
}

impl Debug for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheeseburgerError::ConfigLoadError => write!(f, "Failed to load config"),
            CheeseburgerError::ConnectionError(s)
                => write!(f, "Failed to connect to server: {}", s),
        }
    }
}

impl std::error::Error for CheeseburgerError {}
