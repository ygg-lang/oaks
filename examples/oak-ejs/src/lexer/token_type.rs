/// EJS Token Type module
///
/// This module defines the token types for EJS (Embedded JavaScript) templates.
use oak_core::{TokenType, UniversalTokenRole};

/// Token types for EJS templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EjsTokenType {
    /// Plain text content outside of EJS tags
    Text,
    /// Opening tag: `<%`
    OpenTag,
    /// Escaped output opening tag: `<%=`
    OpenTagOutputEscape,
    /// Raw output opening tag: `%-`
    OpenTagOutputRaw,
    /// Comment opening tag: `<%#`
    OpenTagComment,
    /// Escaped opening tag: `<%%`
    EscapedOpenTag,
    /// Closing tag: `%>`
    CloseTag,
    /// Trim mode closing tag: `-%>`
    CloseTagTrim,
    /// Identifier
    Identifier,
    /// String literal
    String,
    /// Number literal
    Number,
    /// Boolean literal
    Boolean,
    /// Whitespace characters
    Whitespace,
    /// Newline character
    Newline,
    /// Comment content
    Comment,
    /// Left parenthesis: `(`
    LeftParen,
    /// Right parenthesis: `)`
    RightParen,
    /// Left brace: `{`
    LeftBrace,
    /// Right brace: `}`
    RightBrace,
    /// Left bracket: `[`
    LeftBracket,
    /// Right bracket: `]`
    RightBracket,
    /// Comma: `,`
    Comma,
    /// Dot: `.`
    Dot,
    /// Colon: `:`
    Colon,
    /// Semicolon: `;`
    Semicolon,
    /// Equals: `=`
    Eq,
    /// Plus: `+`
    Plus,
    /// Minus: `-`
    Minus,
    /// Star: `*`
    Star,
    /// Slash: `/`
    Slash,
    /// Percent: `%`
    Percent,
    /// Bang: `!`
    Bang,
    /// Question: `?`
    Question,
    /// Less than: `<`
    Lt,
    /// Greater than: `>`
    Gt,
    /// Ampersand: `&`
    Amp,
    /// Pipe: `|`
    Pipe,
    /// Caret: `^`
    Caret,
    /// Tilde: `~`
    Tilde,
    /// End of file
    Eof,
    /// Error token
    Error,
}

impl TokenType for EjsTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace)
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
