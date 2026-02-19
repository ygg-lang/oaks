/// Token type definitions for Scheme lexer.
use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type SchemeToken = Token<SchemeTokenType>;

impl TokenType for SchemeTokenType {
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
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Scheme language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SchemeTokenType {
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
    /// A symbol.
    Symbol,
    /// A keyword.
    Keyword,
    /// `define` keyword.
    Define,
    /// `lambda` keyword.
    Lambda,
    /// `if` keyword.
    If,
    /// `cond` keyword.
    Cond,
    /// `case` keyword.
    Case,
    /// `let` keyword.
    Let,
    /// `let*` keyword.
    LetStar,
    /// `letrec` keyword.
    Letrec,
    /// `begin` keyword.
    Begin,
    /// `do` keyword.
    Do,
    /// `quote` keyword.
    Quote,
    /// `quasiquote` keyword.
    Quasiquote,
    /// `unquote` keyword.
    Unquote,
    /// `unquote-splicing` keyword.
    UnquoteSplicing,
    /// `and` keyword.
    And,
    /// `or` keyword.
    Or,
    /// `not` keyword.
    Not,
    /// `set!` keyword.
    Set,
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
    /// `#`.
    Hash,
    /// `'`.
    Quote_,
    /// `` ` ``.
    Quasiquote_,
    /// `,`.
    Unquote_,
    /// `,@`.
    UnquoteSplicing_,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
    /// A source file.
    SourceFile,
}
