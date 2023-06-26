use crate::lexer::TokenType;
use oak_core::language::UniversalElementRole;

/// Element types for the Racket language parser.
///
/// This enum represents all possible element types in Racket,
/// including expressions, statements, and special constructs.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementType {
    /// Expression element.
    Expression,

    /// Statement element.
    Statement,

    /// For loop construct.
    For,

    /// List comprehension construct.
    ListComprehension,

    /// Block element.
    Block,
    /// Identifier element.
    Identifier,
    /// Number literal element.
    Number,
    /// String literal element.
    String,
    /// Boolean literal element.
    Boolean,
    /// Binary expression element.
    BinaryExpression,
    /// Unary expression element.
    UnaryExpression,
    /// Function call element.
    Call,
    /// Index access element.
    Index,
    /// Tuple element.
    Tuple,
    /// List element.
    List,
    /// Map/dictionary element.
    Map,

    /// End of file marker.
    Eof,
}

impl oak_core::language::ElementType for ElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            ElementType::Expression => UniversalElementRole::Expression,
            ElementType::Statement => UniversalElementRole::Statement,
            ElementType::For => UniversalElementRole::Statement,
            ElementType::ListComprehension => UniversalElementRole::Expression,
            ElementType::Block => UniversalElementRole::Container,
            ElementType::Identifier => UniversalElementRole::Reference,
            ElementType::Number | ElementType::String | ElementType::Boolean => UniversalElementRole::Value,
            ElementType::BinaryExpression | ElementType::UnaryExpression => UniversalElementRole::Expression,
            ElementType::Call => UniversalElementRole::Call,
            ElementType::Index => UniversalElementRole::Expression,
            ElementType::Tuple | ElementType::List | ElementType::Map => UniversalElementRole::Container,
            ElementType::Eof => UniversalElementRole::None,
        }
    }
}

impl From<TokenType> for ElementType {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::For => ElementType::For,
            TokenType::In => ElementType::Expression,
            TokenType::Identifier => ElementType::Identifier,
            TokenType::Number => ElementType::Number,
            TokenType::String => ElementType::String,
            TokenType::Boolean => ElementType::Boolean,
            TokenType::LParen | TokenType::RParen | TokenType::LBracket | TokenType::RBracket | TokenType::LBrace | TokenType::RBrace | TokenType::Comma | TokenType::Dot | TokenType::Colon | TokenType::Semicolon => ElementType::Expression,
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide
            | TokenType::Modulo
            | TokenType::Equals
            | TokenType::NotEquals
            | TokenType::LessThan
            | TokenType::LessThanOrEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanOrEqual
            | TokenType::And
            | TokenType::Or
            | TokenType::Not => ElementType::Expression,
            TokenType::Comment | TokenType::Whitespace => ElementType::Expression,
            TokenType::Require | TokenType::Provide | TokenType::Struct | TokenType::Class | TokenType::Match | TokenType::WithHandlers | TokenType::Raise => ElementType::Expression,
            TokenType::Eof => ElementType::Eof,
        }
    }
}
