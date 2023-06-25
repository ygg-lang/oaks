use oak_core::{Token, TokenType, UniversalTokenRole};

/// Represents a token in an Elm source file.
pub type ElmToken = Token<ElmTokenType>;

/// Token types for the Elm language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElmTokenType {
    /// The root token.
    Root,
    /// Whitespace.
    Whitespace,
    /// A newline character.
    Newline,
    /// A comment.
    Comment,
    /// An identifier.
    Identifier,
    /// An integer number.
    Number,
    /// A floating-point number.
    Float,
    /// A string literal.
    String,
    /// A character literal.
    Char,

    // Keywords
    /// `if` keyword.
    If,
    /// `then` keyword.
    Then,
    /// `else` keyword.
    Else,
    /// `case` keyword.
    Case,
    /// `of` keyword.
    Of,
    /// `let` keyword.
    Let,
    /// `in` keyword.
    In,
    /// `type` keyword.
    Type,
    /// `alias` keyword.
    Alias,
    /// `module` keyword.
    Module,
    /// `where` keyword.
    Where,
    /// `import` keyword.
    Import,
    /// `exposing` keyword.
    Exposing,
    /// `as` keyword.
    As,
    /// `port` keyword.
    Port,

    // Operators
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `//`.
    DoubleSlash,
    /// `^`.
    Caret,
    /// `%`.
    Percent,
    /// `=`.
    Equal,
    /// `==`.
    EqualEqual,
    /// `/=`.
    NotEqual,
    /// `<`.
    Less,
    /// `>`.
    Greater,
    /// `<=`.
    LessEqual,
    /// `>=`.
    GreaterEqual,
    /// `&&`.
    DoubleAmpersand,
    /// `||`.
    DoublePipe,
    /// `++`.
    DoublePlus,
    /// `<<`.
    DoubleLess,
    /// `>>`.
    /// `>>`.
    DoubleGreater,
    /// `->`.
    Arrow,
    /// `|`.
    Pipe,
    /// `|>`.
    PipeGreater,
    /// `.`.
    Dot,
    /// `..`.
    DoubleDot,
    /// `...`.
    TripleDot,
    /// `,`.
    Comma,
    /// `:`.
    Colon,
    /// `;`.
    Semicolon,
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
    /// `\`.
    Backslash,
    /// `|`.
    Bar,

    /// An error token.
    Error,
    /// End of stream.
    Eof,
}

impl ElmTokenType {
    /// Returns true if the token is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(self, Self::If | Self::Then | Self::Else | Self::Case | Self::Of | Self::Let | Self::In | Self::Type | Self::Alias | Self::Module | Self::Where | Self::Import | Self::Exposing | Self::As | Self::Port)
    }
}

impl TokenType for ElmTokenType {
    /// The token role type.
    type Role = UniversalTokenRole;
    /// The end of stream token.
    const END_OF_STREAM: Self = Self::Eof;

    /// Returns true if the token should be ignored during parsing.
    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    /// Returns the role of the token.
    fn role(&self) -> Self::Role {
        match self {
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::Float | Self::String | Self::Char => UniversalTokenRole::Literal,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::DoubleSlash
            | Self::Caret
            | Self::Percent
            | Self::Equal
            | Self::EqualEqual
            | Self::NotEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::DoubleAmpersand
            | Self::DoublePipe
            | Self::DoublePlus
            | Self::DoubleLess
            | Self::DoubleGreater
            | Self::Arrow
            | Self::Pipe
            | Self::PipeGreater
            | Self::Dot
            | Self::DoubleDot
            | Self::TripleDot => UniversalTokenRole::Operator,
            Self::Comma | Self::Colon | Self::Semicolon | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Backslash | Self::Bar => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}
