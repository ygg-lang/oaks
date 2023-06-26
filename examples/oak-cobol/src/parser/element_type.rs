use crate::lexer::token_type::CobolTokenType;
use oak_core::{ElementType, UniversalElementRole};

/// COBOL element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CobolElementType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// String literal.
    String,
    /// Number literal.
    Number,
    /// "IDENTIFICATION" keyword.
    Identification,
    /// "DIVISION" keyword.
    Division,

    /// Root element.
    Root,
    /// Division element.
    DivisionElement,
    /// Paragraph element.
    Paragraph,
}

impl ElementType for CobolElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<CobolTokenType> for CobolElementType {
    fn from(token: CobolTokenType) -> Self {
        match token {
            CobolTokenType::Eof => Self::Eof,
            CobolTokenType::Whitespace => Self::Whitespace,
            CobolTokenType::Comment => Self::Comment,
            CobolTokenType::Identifier => Self::Identifier,
            CobolTokenType::String => Self::String,
            CobolTokenType::Number => Self::Number,
            CobolTokenType::Identification => Self::Identification,
            CobolTokenType::Division => Self::Division,
        }
    }
}
