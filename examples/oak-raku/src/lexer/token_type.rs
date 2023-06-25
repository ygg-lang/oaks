use oak_core::{TokenType, UniversalTokenRole};

/// Raku token types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RakuTokenType {
    /// End of file.
    EndOfFile,
    /// Unknown token.
    Unknown,
    /// Whitespace.
    Whitespace,
    /// Comment.
    Comment,
    /// Identifier.
    Identifier,
    /// Number.
    Number,
    /// String.
    String,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// Operator.
    Operator,
    /// Keyword.
    Keyword,
    /// `my`
    My,
    /// `our`
    Our,
    /// `has`
    Has,
    /// `sub`
    Sub,
    /// `method`
    Method,
    /// `class`
    Class,
    /// `module`
    Module,
    /// `use`
    Use,
    /// `if`
    If,
    /// `else`
    Else,
    /// `elsif`
    Elsif,
    /// `unless`
    Unless,
    /// `for`
    For,
    /// `while`
    While,
    /// `loop`
    Loop,
    /// `repeat`
    Repeat,
    /// `until`
    Until,
    /// `gather`
    Gather,
    /// `take`
    Take,
}

impl TokenType for RakuTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfFile;

    fn role(&self) -> Self::Role {
        match self {
            Self::EndOfFile => UniversalTokenRole::Eof,
            Self::Unknown => UniversalTokenRole::Error,
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number => UniversalTokenRole::Literal,
            Self::String => UniversalTokenRole::Literal,
            Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket => UniversalTokenRole::Punctuation,
            Self::Semicolon | Self::Comma | Self::Dot | Self::Colon | Self::Arrow | Self::FatArrow => UniversalTokenRole::Punctuation,
            Self::Operator => UniversalTokenRole::Operator,
            Self::Keyword
            | Self::My
            | Self::Our
            | Self::Has
            | Self::Sub
            | Self::Method
            | Self::Class
            | Self::Module
            | Self::Use
            | Self::If
            | Self::Else
            | Self::Elsif
            | Self::Unless
            | Self::For
            | Self::While
            | Self::Loop
            | Self::Repeat
            | Self::Until
            | Self::Gather
            | Self::Take => UniversalTokenRole::Keyword,
        }
    }
}

impl Default for RakuTokenType {
    fn default() -> Self {
        Self::Unknown
    }
}
