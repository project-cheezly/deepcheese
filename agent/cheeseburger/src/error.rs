use std::fmt::{Debug, Formatter, Display};

pub enum CheeseburgerError {
    ConfigLoadError,
    ConnectionError(String),
    InvalidQueryCodeError(String),
    ParseError(String),
    TradeError,
    NotFoundError,
}

impl CheeseburgerError {
    fn to_str(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CheeseburgerError::ConfigLoadError => write!(f, "Failed to load config"),
            CheeseburgerError::ConnectionError(s)
                => write!(f, "Failed to connect to server: {}", s),
            CheeseburgerError::InvalidQueryCodeError(s)
                => write!(f, "Invalid query code: {}", s),
            CheeseburgerError::ParseError(s)
                => write!(f, "Failed to parse: {}", s),
            CheeseburgerError::TradeError => write!(f, "Trade error"),
            CheeseburgerError::NotFoundError => write!(f, "Not found error"),
        }
    }
}

impl Display for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.to_str(f)
    }
}

impl Debug for CheeseburgerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_str(f)
    }
}

impl std::error::Error for CheeseburgerError {}
