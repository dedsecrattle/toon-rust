//! Error types for TOON encoding and decoding

use thiserror::Error;

/// Errors that can occur during TOON encoding or decoding
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Parse error with position information
    #[error("Parse error at position {position}: {message}")]
    Parse { position: usize, message: String },

    /// Syntax error
    #[error("Syntax error: {0}")]
    Syntax(String),

    /// Invalid escape sequence
    #[error("Invalid escape sequence: {0}")]
    InvalidEscape(String),

    /// Array length mismatch
    #[error("Array length mismatch: expected {expected}, found {found}")]
    LengthMismatch { expected: usize, found: usize },

    /// Delimiter mismatch
    #[error("Delimiter mismatch: expected '{expected}', found '{found}'")]
    DelimiterMismatch { expected: char, found: char },

    /// Unterminated string
    #[error("Unterminated string")]
    UnterminatedString,

    /// Invalid number format
    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid header format
    #[error("Invalid header format: {0}")]
    InvalidHeader(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl Error {
    /// Create a parse error
    pub fn parse(position: usize, message: impl Into<String>) -> Self {
        Self::Parse {
            position,
            message: message.into(),
        }
    }

    /// Create a syntax error
    pub fn syntax(message: impl Into<String>) -> Self {
        Self::Syntax(message.into())
    }
}
