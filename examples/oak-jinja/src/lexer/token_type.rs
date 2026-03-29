/// Jinja Token Type module
///
/// This module defines the token types for Jinja templates.
use oak_core::{TokenType, UniversalTokenRole};

/// Token types for Jinja templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JinjaTokenType {
    /// Text content
    Text,
    /// Opening delimiter: {{
    DoubleLeftBrace,
    /// Closing delimiter: }}
    DoubleRightBrace,
    /// Opening block delimiter: {%
    LeftBracePercent,
    /// Closing block delimiter: %}
    PercentRightBrace,
    /// Identifier
    Identifier,
    /// String literal
    String,
    /// Number literal
    Number,
    /// Boolean literal
    Boolean,
    /// Left brace: {
    LeftBrace,
    /// Right brace: }
    RightBrace,
    /// Left parenthesis: (
    LeftParen,
    /// Right parenthesis: )
    RightParen,
    /// Left bracket: [
    LeftBracket,
    /// Right bracket: ]
    RightBracket,
    /// Comma: ,
    Comma,
    /// Dot: .
    Dot,
    /// Colon: :
    Colon,
    /// Semicolon: ;
    Semicolon,
    /// Pipe: |
    Pipe,
    /// Equals: =
    Eq,
    /// Plus: +
    Plus,
    /// Minus: -
    Minus,
    /// Whitespace trim marker: -
    TrimMark,
    /// Star: *
    Star,
    /// Slash: /
    Slash,
    /// Percent: %
    Percent,
    /// Bang: !
    Bang,
    /// Question: ?
    Question,
    /// Less than: <
    Lt,
    /// Greater than: >
    Gt,
    /// Double equals: ==
    EqEq,
    /// Not equals: !=
    Neq,
    /// Less than or equal: <=
    LtEq,
    /// Greater than or equal: >=
    GtEq,
    /// Logical and keyword
    And,
    /// Logical or keyword
    Or,
    /// Logical not keyword
    Not,
    /// Ampersand: &
    Amp,
    /// Caret: ^
    Caret,
    /// Tilde: ~
    Tilde,
    /// Whitespace
    Whitespace,
    /// Comment
    Comment,
    /// End of file
    Eof,
    /// Error
    Error,
}

impl TokenType for JinjaTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::TrimMark)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::TrimMark => UniversalTokenRole::Whitespace,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
