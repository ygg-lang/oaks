use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Rhombus language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RhombusElementType {
    /// A source file.
    SourceFile,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// A comment.
    Comment,
    /// A line comment.
    LineComment,
    /// A numeric literal.
    NumberLiteral,
    /// A string literal.
    StringLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// An identifier.
    Identifier,
    /// A block.
    Block,
    /// A statement.
    Statement,
    /// An expression.
    Expression,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
}

impl ElementType for RhombusElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            Self::Block => UniversalElementRole::Statement,
            Self::Statement => UniversalElementRole::Statement,
            Self::Expression => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RhombusTokenType> for RhombusElementType {
    fn from(token: crate::lexer::token_type::RhombusTokenType) -> Self {
        use crate::lexer::token_type::RhombusTokenType as T;
        match token {
            T::Whitespace => RhombusElementType::Whitespace,
            T::Newline => RhombusElementType::Newline,
            T::Comment => RhombusElementType::Comment,
            T::LineComment => RhombusElementType::LineComment,
            T::NumberLiteral => RhombusElementType::NumberLiteral,
            T::StringLiteral => RhombusElementType::StringLiteral,
            T::BooleanLiteral => RhombusElementType::BooleanLiteral,
            T::Identifier => RhombusElementType::Identifier,
            T::Fun | T::Val | T::Var | T::Let | T::If | T::Else | T::Match | T::Case | T::Block | T::Module | T::Import | T::Export | T::Require | T::Provide => RhombusElementType::Identifier, // Keywords are identifiers for now
            T::LeftParen | T::RightParen | T::LeftBracket | T::RightBracket | T::LeftBrace | T::RightBrace | T::Dot | T::Comma | T::Colon | T::Semicolon => RhombusElementType::Identifier,      // Punctuation
            T::Error => RhombusElementType::Error,
            T::Eof => RhombusElementType::Eof,
            _ => RhombusElementType::Error,
        }
    }
}
