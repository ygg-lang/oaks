use oak_core::{Token, TokenType, UniversalTokenRole};

/// A token in the Sass language.
pub type SassToken = Token<SassTokenType>;

/// Token types for the Sass language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum SassTokenType {
    // Basics
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// A single-line comment starting with `//`.
    LineComment,
    /// A block comment delimited by `/*` and `*/`.
    BlockComment,
    /// End of file marker.
    Eof,
    /// Lexical error.
    Error,

    // Literals
    /// A string literal.
    StringLiteral,
    /// A character literal.
    CharLiteral,
    /// A numeric literal.
    NumberLiteral,
    /// A floating-point literal.
    FloatLiteral,
    /// A color literal (e.g., `#ffffff`).
    ColorLiteral,
    /// A Sass variable (e.g., `$name`).
    Variable,
    /// An identifier.
    Identifier,

    // Keywords
    /// The `@import` keyword.
    Import,
    /// The `@include` keyword.
    Include,
    /// The `@extend` keyword.
    Extend,
    /// The `@mixin` keyword.
    Mixin,
    /// The `@function` keyword.
    Function,
    /// The `@return` keyword.
    Return,
    /// The `@if` keyword.
    If,
    /// The `@else` keyword.
    Else,
    /// The `@else if` keyword.
    ElseIf,
    /// The `@for` keyword.
    For,
    /// The `@each` keyword.
    Each,
    /// The `@while` keyword.
    While,
    /// The `!default` flag.
    Default,
    /// The `!important` flag.
    Important,
    /// The `!optional` flag.
    Optional,
    /// The `!global` flag.
    Global,
    /// The `and` keyword.
    And,
    /// The `or` keyword.
    Or,
    /// The `not` keyword.
    Not,

    // Operators
    /// The `==` operator.
    EqEq,
    /// The `!=` operator.
    Ne,
    /// The `<=` operator.
    Le,
    /// The `>=` operator.
    Ge,
    /// The `+` operator.
    Plus,
    /// The `-` operator.
    Minus,
    /// The `*` operator.
    Star,
    /// The `/` operator.
    Slash,
    /// The `%` operator.
    Percent,
    /// The `=` operator.
    Eq,
    /// The `<` operator.
    Lt,
    /// The `>` operator.
    Gt,

    // Punctuation
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Semicolon `;`.
    Semicolon,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// Hash `#`.
    Hash,
    /// Dollar sign `$`.
    Dollar,
    /// At sign `@`.
    At,
    /// Ampersand `&`.
    Ampersand,
    /// Exclamation mark `!`.
    Exclamation,
    /// Question mark `?`.
    Question,
    /// Tilde `~`.
    Tilde,
}

impl TokenType for SassTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        if self.is_keyword() {
            return UniversalTokenRole::Keyword;
        }

        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Variable => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::FloatLiteral | Self::ColorLiteral => UniversalTokenRole::Literal,
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBrace
            | Self::RightBrace
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon
            | Self::Colon
            | Self::Comma
            | Self::Dot
            | Self::Hash
            | Self::Dollar
            | Self::At
            | Self::Ampersand
            | Self::Exclamation
            | Self::Question
            | Self::Tilde => UniversalTokenRole::Punctuation,
            Self::EqEq | Self::Ne | Self::Le | Self::Ge | Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Eq | Self::Lt | Self::Gt => UniversalTokenRole::Operator,
            _ => UniversalTokenRole::None,
        }
    }
}

impl SassTokenType {
    /// Returns `true` if the token type is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Import
                | Self::Include
                | Self::Extend
                | Self::Mixin
                | Self::Function
                | Self::Return
                | Self::If
                | Self::Else
                | Self::ElseIf
                | Self::For
                | Self::Each
                | Self::While
                | Self::Default
                | Self::Important
                | Self::Optional
                | Self::Global
                | Self::And
                | Self::Or
                | Self::Not
        )
    }
}
