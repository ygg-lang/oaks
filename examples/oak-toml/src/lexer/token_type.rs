use oak_core::{TokenType, UniversalTokenRole};

/// Represents the different types of tokens in the TOML language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TomlTokenKind {
    /// Spaces, tabs, and other non-newline whitespace.
    Whitespace,
    /// Newline characters (\n or \r\n).
    Newline,
    /// TOML comments starting with #.
    Comment,

    // Literals
    /// A generic string literal.
    String,
    /// A basic string with escape sequences (surrounded by ").
    BasicString,
    /// A literal string without escape sequences (surrounded by ').
    LiteralString,
    /// An integer literal.
    Integer,
    /// A floating-point literal.
    Float,
    /// A boolean literal (true or false).
    Boolean,
    /// An RFC 3339 offset date-time.
    OffsetDateTime,
    /// A local date-time without offset.
    LocalDateTime,
    /// A local date without time.
    LocalDate,
    /// A local time without date.
    LocalTime,

    // Keywords/Identifiers
    /// A generic identifier.
    Identifier,
    /// A bare key used in key-value pairs.
    BareKey,

    // Symbols
    /// The equals sign (=) used for assignment.
    Equal, // =
    /// The dot (.) used for nested keys.
    Dot, // .
    /// The comma (,) used as a separator in arrays or inline tables.
    Comma, // ,
    /// The left brace ({) for inline tables.
    LeftBrace, // {
    /// The right brace (}) for inline tables.
    RightBrace, // }
    /// The left bracket ([) for arrays or tables.
    LeftBracket, // [
    /// The right bracket (]) for arrays or tables.
    RightBracket, // ]
    /// Double left brackets ([[) for array of tables.
    DoubleLeftBracket,
    /// Double right brackets (]]) for array of tables.
    DoubleRightBracket,

    // Internal
    /// Represents a lexing error.
    Error,
    /// Represents the end of the source stream.
    Eof,
    /// Represents the root of the TOML document.
    Root,
}

/// Alias for `TomlTokenKind` to conform to Oak's naming conventions.
pub type TomlTokenType = TomlTokenKind;

impl TokenType for TomlTokenKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::BasicString | Self::LiteralString | Self::String | Self::Integer | Self::Float | Self::Boolean | Self::OffsetDateTime | Self::LocalDateTime | Self::LocalDate | Self::LocalTime => UniversalTokenRole::Literal,
            Self::BareKey | Self::Identifier => UniversalTokenRole::Name,
            Self::Equal | Self::Dot | Self::Comma | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::DoubleLeftBracket | Self::DoubleRightBracket => UniversalTokenRole::Punctuation,
            Self::Root => UniversalTokenRole::None,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
        }
    }
}
