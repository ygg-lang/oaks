use oak_core::{TokenType, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
/// Represents all possible token kinds in the AsciiDoc markup language.
pub enum AsciiDocTokenType {
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline characters
    Newline,

    /// Section level 1 header marker (`=`)
    Header1,
    /// Section level 2 header marker (`==`)
    Header2,
    /// Section level 3 header marker (`===`)
    Header3,
    /// Section level 4 header marker (`====`)
    Header4,
    /// Section level 5 header marker (`=====`)
    Header5,
    /// Section level 6 header marker (`======`)
    Header6,

    /// Bold formatting marker (`*` or `**`)
    BoldMarker,
    /// Italic formatting marker (`_` or `__`)
    ItalicMarker,
    /// Monospace formatting marker (`` ` `` or `` `` ` ``)
    MonospaceMarker,
    /// Code block marker (`----`)
    CodeBlockMarker,
    /// Link marker (`link:`)
    LinkMarker,
    /// List marker (`*`, `-`, `.`)
    ListMarker,

    /// Table delimiter (`|`, `!`)
    TableDelimiter,

    /// Comment (both single-line and multi-line)
    Comment,

    /// Plain text content
    Text,

    /// Line break
    LineBreak,
    /// Page break
    PageBreak,
    /// Delimiter
    Delimiter,

    /// Left bracket: `[`
    LeftBracket,
    /// Right bracket: `]`
    RightBracket,
    /// Left parenthesis: `(`
    LeftParen,
    /// Right parenthesis: `)`
    RightParen,
    /// Colon: `:`
    Colon,
    /// Comma: `,`
    Comma,
    /// Dot: `.`
    Dot,

    /// Error token
    Error,
    /// End of file marker
    Eof,
}

impl TokenType for AsciiDocTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Header1 | Self::Header2 | Self::Header3 | Self::Header4 | Self::Header5 | Self::Header6 => UniversalTokenRole::Keyword,
            Self::BoldMarker | Self::ItalicMarker | Self::MonospaceMarker | Self::CodeBlockMarker | Self::LinkMarker | Self::ListMarker => UniversalTokenRole::Operator,
            Self::Text => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}
