use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type RacketToken = Token<RacketTokenType>;

impl TokenType for RacketTokenType {
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
            Self::Dot | Self::Quote_ | Self::Quasiquote_ | Self::Unquote_ | Self::UnquoteSplicing_ => UniversalTokenRole::Operator,
            Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::StringLiteral | Self::CharacterLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            Self::Define
            | Self::Lambda
            | Self::If
            | Self::Cond
            | Self::Case
            | Self::Let
            | Self::LetStar
            | Self::Letrec
            | Self::Begin
            | Self::Do
            | Self::Quote
            | Self::Quasiquote
            | Self::Unquote
            | Self::UnquoteSplicing
            | Self::And
            | Self::Or
            | Self::Not
            | Self::Set => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the Racket language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RacketTokenType {
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
