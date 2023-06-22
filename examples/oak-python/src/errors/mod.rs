use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ParseError {
    InvalidRequest,
}

impl ParseError {
    pub fn new() -> Self {
        Self::InvalidRequest
    }
}

pub struct Fail {
    pub error: ParseError,
}

impl Fail {
    pub fn new(error: ParseError) -> Self {
        Self { error }
    }
}

pub struct PexError {
    pub kind: PexErrorKind,
}

impl PexError {
    pub fn new(kind: PexErrorKind) -> Self {
        Self { kind }
    }
}

pub enum PexErrorKind {
    SyntaxError {},
}

impl PexErrorKind {
    pub fn new_syntax_error() -> Self {
        Self::SyntaxError {}
    }
}
