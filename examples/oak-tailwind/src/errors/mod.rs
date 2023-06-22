#[derive(Copy, Clone, Debug)]
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

pub struct PexError {}

pub enum PexErrorKind {
    SyntaxError {},
}
