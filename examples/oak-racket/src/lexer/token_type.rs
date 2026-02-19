use oak_core::language::UniversalTokenRole;

/// Token types for the Racket language lexer.
///
/// This enum represents all possible token types in Racket,
/// including keywords, identifiers, literals, punctuation, and operators.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenType {
    /// For keyword for iteration.
    For,
    /// In keyword for iteration context.
    In,
    /// Require keyword for module imports.
    Require,
    /// Provide keyword for module exports.
    Provide,
    /// Struct keyword for structure definitions.
    Struct,
    /// Class keyword for class definitions.
    Class,
    /// Match keyword for pattern matching.
    Match,
    /// With-handlers keyword for exception handling.
    WithHandlers,
    /// Raise keyword for raising exceptions.
    Raise,

    /// Identifier token.
    Identifier,

    /// Number literal token.
    Number,
    /// String literal token.
    String,
    /// Boolean literal token.
    Boolean,

    /// Left parenthesis `(`.
    LParen,
    /// Right parenthesis `)`.
    RParen,
    /// Left bracket `[`.
    LBracket,
    /// Right bracket `]`.
    RBracket,
    /// Left brace `{`.
    LBrace,
    /// Right brace `}`.
    RBrace,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,

    /// Plus operator `+`.
    Plus,
    /// Minus operator `-`.
    Minus,
    /// Multiply operator `*`.
    Multiply,
    /// Divide operator `/`.
    Divide,
    /// Modulo operator `%`.
    Modulo,
    /// Equality operator `=`.
    Equals,
    /// Inequality operator `!=`.
    NotEquals,
    /// Less than operator `<`.
    LessThan,
    /// Less than or equal operator `<=`.
    LessThanOrEqual,
    /// Greater than operator `>`.
    GreaterThan,
    /// Greater than or equal operator `>=`.
    GreaterThanOrEqual,
    /// Logical and operator `and`.
    And,
    /// Logical or operator `or`.
    Or,
    /// Logical not operator `not`.
    Not,

    /// Comment token.
    Comment,
    /// Whitespace token.
    Whitespace,
    /// End of file token.
    Eof,
}

impl oak_core::language::TokenType for TokenType {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = TokenType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            TokenType::For | TokenType::In | TokenType::Require | TokenType::Provide | TokenType::Struct | TokenType::Class | TokenType::Match | TokenType::WithHandlers | TokenType::Raise => UniversalTokenRole::Keyword,
            TokenType::Identifier => UniversalTokenRole::Name,
            TokenType::Number | TokenType::String | TokenType::Boolean => UniversalTokenRole::Literal,
            TokenType::LParen | TokenType::RParen | TokenType::LBracket | TokenType::RBracket | TokenType::LBrace | TokenType::RBrace | TokenType::Comma | TokenType::Dot | TokenType::Colon | TokenType::Semicolon => UniversalTokenRole::Punctuation,
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
            | TokenType::Not => UniversalTokenRole::Operator,
            TokenType::Comment => UniversalTokenRole::Comment,
            TokenType::Whitespace => UniversalTokenRole::Whitespace,
            TokenType::Eof => UniversalTokenRole::Eof,
        }
    }
}
