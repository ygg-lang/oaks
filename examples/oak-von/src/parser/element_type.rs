use oak_core::{ElementType, UniversalElementRole};

/// Element types for the VON (Value-Oriented Notation) parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VonElementType {
    /// Whitespace characters.
    Whitespace,
    /// Line breaks.
    Newline,
    /// Comments.
    Comment,
    /// End of file.
    Eof,
    /// An opening brace (`{`).
    LeftBrace,
    /// A closing brace (`}`).
    RightBrace,
    /// An opening bracket (`[`).
    LeftBracket,
    /// A closing bracket (`]`).
    RightBracket,
    /// A comma (`,`).
    Comma,
    /// A colon (`:`).
    Colon,
    /// An equal sign (`=`).
    Eq,
    /// A string literal.
    StringLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A boolean literal.
    BoolLiteral,
    /// A null literal.
    NullLiteral,
    /// An identifier.
    Identifier,
    /// A value element.
    Value,
    /// An object element.
    Object,
    /// An array element.
    Array,
    /// An entry in an object.
    ObjectEntry,
    /// An enum element.
    Enum,
    /// An error node in the parse tree.
    ErrorNode,
    /// An error element.
    Error,
    /// The root of the parse tree.
    Root,
    /// An element in an array.
    ArrayElement,
}

impl ElementType for VonElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,

            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VonTokenType> for VonElementType {
    fn from(token: crate::lexer::token_type::VonTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
