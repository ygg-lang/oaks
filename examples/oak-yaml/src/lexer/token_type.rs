use oak_core::{Token, TokenType, UniversalTokenRole};

/// Alias for `Token<YamlTokenType>`.
pub type YamlToken = Token<YamlTokenType>;

impl TokenType for YamlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            Self::Identifier | Self::Anchor | Self::Alias | Self::Tag => UniversalTokenRole::Name,
            Self::Colon | Self::Dash | Self::Pipe | Self::GreaterThan | Self::Question | Self::Ampersand | Self::Asterisk | Self::Exclamation => UniversalTokenRole::Operator,
            Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::DocumentStart | Self::DocumentEnd => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for the YAML language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum YamlTokenType {
    // Trivia
    /// Whitespace.
    Whitespace,
    /// A comment.
    Comment,

    // Literals
    /// A string literal.
    StringLiteral,
    /// A number literal.
    NumberLiteral,
    /// A boolean literal.
    BooleanLiteral,
    /// A null literal.
    NullLiteral,

    // Identifiers
    /// An identifier.
    Identifier,

    // Operators and punctuation
    /// Colon `:`.
    Colon, // :
    /// Dash `-`.
    Dash, // -
    /// Pipe `|` for block scalars.
    Pipe, // |
    /// Greater than `>` for folded scalars.
    GreaterThan, // >
    /// Question mark `?` for explicit keys.
    Question, // ?
    /// Ampersand `&` for anchors.
    Ampersand, // &
    /// Asterisk `*` for aliases.
    Asterisk, // *
    /// Exclamation mark `!` for tags.
    Exclamation, // !

    // Brackets
    /// Left bracket `[`.
    LeftBracket, // [
    /// Right bracket `]`.
    RightBracket, // ]
    /// Left brace `{`.
    LeftBrace, // {
    /// Right brace `}`.
    RightBrace, // }

    // Special
    /// An anchor `&anchor`.
    Anchor, // &anchor
    /// An alias `*alias`.
    Alias, // *alias
    /// A tag `!tag`.
    Tag, // !tag

    // Document markers
    /// Document start `---`.
    DocumentStart, // ---
    /// Document end `...`.
    DocumentEnd, // ...
    /// A document.
    Document,
    /// Root node.
    Root,

    // Newlines and indentation
    /// A newline.
    Newline,

    // Errors and EOF
    /// Lexing error.
    Error,
    /// End of stream.
    Eof,
}
