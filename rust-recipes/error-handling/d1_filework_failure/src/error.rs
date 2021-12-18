
use failure::*;
// use failure_derive::*;

// Handle error types
#[derive(Debug, Fail)]
pub enum TransactionError {
    // Access zero-th element of the tuple contained in out enum error types
    // Add the display method in the failure type
    #[fail(display="Could not load file: {}", 0)]
    LoadError(std::io::Error),
    #[fail(display="Could not parse file: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display="Error: {}", 0)]
    Mess(&'static str),
}

// Implement custom error type 'LoadError' to handle 'std::io::Error'
impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

// Implement custom error type 'ParseError' to handle 'serde_json::Error'
impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

// Implement custom error type that accepts a &'static str
impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}
