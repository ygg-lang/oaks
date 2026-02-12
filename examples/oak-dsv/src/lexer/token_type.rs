use oak_core::{TokenType, UniversalTokenRole};

/// DSV token type
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DsvTokenType {
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,

    /// Field value (quoted or unquoted)
    Field,
    /// Quoted field value
    QuotedField,
    /// Unquoted field value
    UnquotedField,
    /// Field separator (e.g., , or \t)
    Separator,
    /// Quote character "
    Quote,
    /// Escaped quote ""
    EscapedQuote,

    /// End of file
    Eof,
    /// Error
    Error,
}

impl TokenType for DsvTokenType {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_comment(&self) -> bool {
        false
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Field | Self::QuotedField | Self::UnquotedField => UniversalTokenRole::Literal,
            Self::Separator | Self::Quote | Self::EscapedQuote => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
        }
    }
}
