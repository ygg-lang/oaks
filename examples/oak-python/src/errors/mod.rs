#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Python parse errors.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum ParseError {
    /// Invalid request error.
    InvalidRequest,
}

impl ParseError {
    /// Creates a new invalid request error.
    pub fn new() -> Self {
        Self::InvalidRequest
    }
}

/// Represents a failure during parsing.
pub struct Fail {
    /// The underlying parse error.
    pub error: ParseError,
}

impl Fail {
    /// Creates a new failure.
    pub fn new(error: ParseError) -> Self {
        Self { error }
    }
}

/// Pex error structure.
pub struct PexError {
    /// Kind of the error.
    pub kind: PexErrorKind,
}

impl PexError {
    /// Creates a new Pex error.
    pub fn new(kind: PexErrorKind) -> Self {
        Self { kind }
    }
}

/// Kinds of Pex errors.
pub enum PexErrorKind {
    /// Syntax error.
    SyntaxError {},
}

impl PexErrorKind {
    /// Creates a new syntax error kind.
    pub fn new_syntax_error() -> Self {
        Self::SyntaxError {}
    }
}
