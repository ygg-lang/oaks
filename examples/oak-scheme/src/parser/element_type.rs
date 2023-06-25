use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Scheme language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SchemeElementType {
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

impl ElementType for SchemeElementType {
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

impl From<crate::lexer::token_type::SchemeTokenType> for SchemeElementType {
    fn from(token: crate::lexer::token_type::SchemeTokenType) -> Self {
        use crate::lexer::token_type::SchemeTokenType as T;
        match token {
            T::SourceFile => SchemeElementType::SourceFile,
            T::Whitespace => SchemeElementType::Whitespace,
            T::Newline => SchemeElementType::Newline,
            T::Comment => SchemeElementType::Comment,
            T::LineComment => SchemeElementType::LineComment,
            T::NumberLiteral => SchemeElementType::NumberLiteral,
            T::StringLiteral => SchemeElementType::StringLiteral,
            T::CharacterLiteral => SchemeElementType::CharacterLiteral,
            T::BooleanLiteral => SchemeElementType::BooleanLiteral,
            T::Identifier => SchemeElementType::Identifier,
            T::Symbol => SchemeElementType::Symbol,
            T::Keyword => SchemeElementType::Keyword,
            T::Define => SchemeElementType::Define,
            T::Lambda => SchemeElementType::Lambda,
            T::If => SchemeElementType::If,
            T::Cond => SchemeElementType::Cond,
            T::Case => SchemeElementType::Case,
            T::Let => SchemeElementType::Let,
            T::LetStar => SchemeElementType::LetStar,
            T::Letrec => SchemeElementType::Letrec,
            T::Begin => SchemeElementType::Begin,
            T::Do => SchemeElementType::Do,
            T::Quote => SchemeElementType::Quote,
            T::Quasiquote => SchemeElementType::Quasiquote,
            T::Unquote => SchemeElementType::Unquote,
            T::UnquoteSplicing => SchemeElementType::UnquoteSplicing,
            T::And => SchemeElementType::And,
            T::Or => SchemeElementType::Or,
            T::Not => SchemeElementType::Not,
            T::Set => SchemeElementType::Set,
            T::LeftParen => SchemeElementType::LeftParen,
            T::RightParen => SchemeElementType::RightParen,
            T::LeftBracket => SchemeElementType::LeftBracket,
            T::RightBracket => SchemeElementType::RightBracket,
            T::LeftBrace => SchemeElementType::LeftBrace,
            T::RightBrace => SchemeElementType::RightBrace,
            T::Dot => SchemeElementType::Dot,
            T::Hash => SchemeElementType::Hash,
            T::Quote_ => SchemeElementType::Quote_,
            T::Quasiquote_ => SchemeElementType::Quasiquote_,
            T::Unquote_ => SchemeElementType::Unquote_,
            T::UnquoteSplicing_ => SchemeElementType::UnquoteSplicing_,
            T::Error => SchemeElementType::Error,
            T::Eof => SchemeElementType::Eof,
        }
    }
}
