// use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug)]
pub enum ParseError {}

pub struct PexError {}

pub enum PexErrorKind {
    SyntaxError {},
}
