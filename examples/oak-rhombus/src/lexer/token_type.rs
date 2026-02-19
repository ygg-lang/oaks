use oak_core::{Token, TokenType, UniversalTokenRole};

/// Token type for Rhombus lexer output.
pub type RhombusToken = Token<RhombusTokenType>;

impl TokenType for RhombusTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::LineComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment | Self::LineComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            Self::NumberLiteral | Self::StringLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Fun | Self::Val | Self::Var | Self::Let | Self::If | Self::Else | Self::Match | Self::Case | Self::Block | Self::Module | Self::Import | Self::Export | Self::Require | Self::Provide => UniversalTokenRole::Keyword,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Rhombus language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RhombusTokenType {
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
    /// A character literal.
    CharacterLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// An identifier.
    Identifier,
    /// `fun` keyword.
    Fun,
    /// `val` keyword.
    Val,
    /// `var` keyword.
    Var,
    /// `let` keyword.
    Let,
    /// `if` keyword.
    If,
    /// `else` keyword.
    Else,
    /// `match` keyword.
    Match,
    /// `case` keyword.
    Case,
    /// `block` keyword.
    Block,
    /// `module` keyword.
    Module,
    /// `import` keyword.
    Import,
    /// `export` keyword.
    Export,
    /// `require` keyword.
    Require,
    /// `provide` keyword.
    Provide,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `.`.
    Dot,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
    /// `;`.
    Semicolon,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
}
