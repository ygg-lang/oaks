use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ParseError {}

pub struct PexError {}

pub enum PexErrorKind {
    SyntaxError {},
}
