use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Racket language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RacketElementType {
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
    /// A list.
    List,
    /// A quotation.
    Quotation,
    /// An error token.
    Error,
    /// End of stream.
    Eof,
}

impl ElementType for RacketElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            Self::List => UniversalElementRole::Expression,
            Self::Quotation => UniversalElementRole::Expression,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RacketTokenType> for RacketElementType {
    fn from(token: crate::lexer::token_type::RacketTokenType) -> Self {
        use crate::lexer::token_type::RacketTokenType as T;
        match token {
            T::SourceFile => RacketElementType::SourceFile,
            T::Whitespace => RacketElementType::Whitespace,
            T::Newline => RacketElementType::Newline,
            T::Comment => RacketElementType::Comment,
            T::LineComment => RacketElementType::LineComment,
            T::NumberLiteral => RacketElementType::NumberLiteral,
            T::StringLiteral => RacketElementType::StringLiteral,
            T::CharacterLiteral => RacketElementType::CharacterLiteral,
            T::BooleanLiteral => RacketElementType::BooleanLiteral,
            T::Identifier => RacketElementType::Identifier,
            T::Symbol => RacketElementType::Symbol,
            T::Keyword => RacketElementType::Keyword,
            T::Define => RacketElementType::Define,
            T::Lambda => RacketElementType::Lambda,
            T::If => RacketElementType::If,
            T::Cond => RacketElementType::Cond,
            T::Case => RacketElementType::Case,
            T::Let => RacketElementType::Let,
            T::LetStar => RacketElementType::LetStar,
            T::Letrec => RacketElementType::Letrec,
            T::Begin => RacketElementType::Begin,
            T::Do => RacketElementType::Do,
            T::Quote => RacketElementType::Quote,
            T::Quasiquote => RacketElementType::Quasiquote,
            T::Unquote => RacketElementType::Unquote,
            T::UnquoteSplicing => RacketElementType::UnquoteSplicing,
            T::And => RacketElementType::And,
            T::Or => RacketElementType::Or,
            T::Not => RacketElementType::Not,
            T::Set => RacketElementType::Set,
            T::LeftParen => RacketElementType::LeftParen,
            T::RightParen => RacketElementType::RightParen,
            T::LeftBracket => RacketElementType::LeftBracket,
            T::RightBracket => RacketElementType::RightBracket,
            T::LeftBrace => RacketElementType::LeftBrace,
            T::RightBrace => RacketElementType::RightBrace,
            T::Dot => RacketElementType::Dot,
            T::Hash => RacketElementType::Hash,
            T::Quote_ => RacketElementType::Quote_,
            T::Quasiquote_ => RacketElementType::Quasiquote_,
            T::Unquote_ => RacketElementType::Unquote_,
            T::UnquoteSplicing_ => RacketElementType::UnquoteSplicing_,
            T::Error => RacketElementType::Error,
            T::Eof => RacketElementType::Eof,
        }
    }
}
