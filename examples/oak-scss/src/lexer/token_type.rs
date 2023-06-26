use oak_core::{TokenType, UniversalTokenRole};

/// Represents all token types recognized by the SCSS lexer.
///
/// This enum defines the complete set of token types that can be produced
/// when lexing SCSS source code, including keywords, operators, punctuation,
/// literals, and special tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ScssTokenType {
    /// The `@import` keyword for importing external stylesheets.
    Import,
    /// The `@include` keyword for including mixin content.
    Include,
    /// The `@mixin` keyword for defining reusable style blocks.
    Mixin,
    /// The `@function` keyword for defining custom functions.
    Function,
    /// The `@return` keyword for returning values from functions.
    Return,
    /// The `@if` keyword for conditional statements.
    If,
    /// The `@else` keyword for alternative conditional branches.
    Else,
    /// The `@for` keyword for iteration loops.
    For,
    /// The `@while` keyword for conditional loops.
    While,
    /// The `@each` keyword for iterating over lists or maps.
    Each,
    /// The `in` keyword used in `@each` loops.
    In,
    /// The `true` boolean literal.
    True,
    /// The `false` boolean literal.
    False,
    /// The `null` literal representing no value.
    Null,

    /// The equality operator `==`.
    EqEq,
    /// The inequality operator `!=`.
    Ne,
    /// The less-than-or-equal operator `<=`.
    Le,
    /// The greater-than-or-equal operator `>=`.
    Ge,
    /// The logical AND operator `&&`.
    AndAnd,
    /// The logical OR operator `||`.
    OrOr,
    /// The assignment operator `=`.
    Eq,
    /// The less-than operator `<`.
    Lt,
    /// The greater-than operator `>`.
    Gt,
    /// The bitwise AND operator `&`.
    And,
    /// The bitwise OR operator `|`.
    Or,
    /// The bitwise XOR operator `^`.
    Xor,
    /// The addition operator `+`.
    Plus,
    /// The subtraction operator `-`.
    Minus,
    /// The multiplication operator `*`.
    Star,
    /// The division operator `/`.
    Slash,
    /// The modulo operator `%`.
    Percent,
    /// The negation operator `!`.
    Bang,

    /// The left parenthesis `(`.
    LeftParen,
    /// The right parenthesis `)`.
    RightParen,
    /// The left brace `{`.
    LeftBrace,
    /// The right brace `}`.
    RightBrace,
    /// The left bracket `[`.
    LeftBracket,
    /// The right bracket `]`.
    RightBracket,
    /// The semicolon `;`.
    Semicolon,
    /// The colon `:`.
    Colon,
    /// The comma `,`.
    Comma,
    /// The dot `.`.
    Dot,
    /// The hash symbol `#`.
    Hash,
    /// The at symbol `@`.
    At,
    /// The dollar sign `$` used for variables.
    Dollar,

    /// An identifier token (variable names, function names, property names, etc.).
    Identifier,
    /// An integer literal token.
    IntegerLiteral,
    /// A string literal token.
    StringLiteral,

    /// Whitespace characters (spaces, tabs, etc.).
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment token (single-line or multi-line).
    Comment,
    /// End of file marker.
    Eof,
    /// An error token for unrecognized input.
    Error,
}

impl TokenType for ScssTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Import | Self::Include | Self::Mixin | Self::Function | Self::Return | Self::If | Self::Else | Self::For | Self::While | Self::Each | Self::In | Self::True | Self::False | Self::Null => UniversalTokenRole::Keyword,

            Self::EqEq | Self::Ne | Self::Le | Self::Ge | Self::AndAnd | Self::OrOr | Self::Eq | Self::Lt | Self::Gt | Self::And | Self::Or | Self::Xor | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Bang => {
                UniversalTokenRole::Operator
            }

            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Semicolon | Self::Colon | Self::Comma | Self::Dot | Self::Hash | Self::At | Self::Dollar => {
                UniversalTokenRole::Punctuation
            }

            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::StringLiteral => UniversalTokenRole::Literal,

            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
        }
    }
}
