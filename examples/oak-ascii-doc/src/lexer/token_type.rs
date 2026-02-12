use oak_core::{Token, TokenType, UniversalTokenRole};

/// Type alias for an AsciiDoc token.
pub type AsciiDocToken = Token<AsciiDocTokenType>;

/// Token types for the AsciiDoc language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AsciiDocTokenType {
    /// Whitespace characters.
    Whitespace,
    /// Newline character.
    Newline,
    /// Level 1 header (`=`).
    Header1,
    /// Level 2 header (`==`).
    Header2,
    /// Level 3 header (`===`).
    Header3,
    /// Level 4 header (`====`).
    Header4,
    /// Level 5 header (`=====`).
    Header5,
    /// Level 6 header (`======`).
    Header6,
    /// Bold text marker (`*`).
    BoldMarker,
    /// Italic text marker (`_`).
    ItalicMarker,
    /// Monospace text marker (`` ` ``).
    MonospaceMarker,
    /// Code block marker (`----`).
    CodeBlockMarker,
    /// Link marker (`http`, `https`, `mailto`).
    LinkMarker,
    /// List item marker (`*`, `-`, `.`).
    ListMarker,
    /// Table cell delimiter (`|`).
    TableDelimiter,
    /// Comment (`//`).
    Comment,
    /// Plain text.
    Text,
    /// Hard line break (` +`).
    LineBreak,
    /// Page break marker (`<<<`).
    PageBreak,
    /// Attribute marker (`:name:`).
    AttributeMarker,
    /// Admonition marker (`NOTE:`, `TIP:`, etc.).
    AdmonitionMarker,
    /// Generic delimiter.
    Delimiter,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Colon `:`.
    Colon,
    /// Comma `,`.
    Comma,
    /// Dot `.`.
    Dot,
    /// End of stream.
    Eof,
    /// Lexing error.
    Error,
}

impl TokenType for AsciiDocTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
