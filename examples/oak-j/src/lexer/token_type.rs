use oak_core::{TokenType, UniversalTokenRole};

/// J token type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JTokenType {
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,
    /// Comment
    Comment,

    /// String literal
    StringLiteral,
    /// Number literal
    NumberLiteral,
    /// Identifier
    Identifier,

    // J primitives (Verbs, Adverbs, Conjunctions)
    // Basic symbols
    /// Equal (=)
    Equal,
    /// Dot (.)
    Dot,
    /// Colon (:)
    Colon,

    // Assignment
    /// Global assignment (=:)
    IsGlobal,
    /// Local assignment (=.)
    IsLocal,

    // Common verbs
    /// Plus (+)
    Plus,
    /// Minus (-)
    Minus,
    /// Star (*)
    Star,
    /// Percent (%)
    Percent,
    /// Dollar ($)
    Dollar,
    /// Comma (,)
    Comma,
    /// Hash (#)
    Hash,
    /// Slash (/)
    Slash,
    /// Backslash (\)
    Backslash,
    /// Pipe (|)
    Pipe,
    /// Ampersand (&)
    Ampersand,
    /// Caret (^)
    Caret,
    /// Tilde (~)
    Tilde,
    /// Less (<)
    Less,
    /// Greater (>)
    Greater,

    // Parentheses
    /// Left parenthesis (()
    LeftParen,
    /// Right parenthesis ())
    RightParen,
    /// Left bracket ([)
    LeftBracket,
    /// Right bracket (])
    RightBracket,
    /// Left brace ({)
    LeftBrace,
    /// Right brace (})
    RightBrace,

    // Special
    /// End of file
    Eof,
    /// Error unit
    Error,
}

impl JTokenType {
    /// Is keyword (J mostly uses primitives)
    pub fn is_keyword(&self) -> bool {
        false
    }

    /// Is punctuation
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace)
    }

    /// Is ignored token (whitespace or comment)
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }
}

impl TokenType for JTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::StringLiteral | Self::NumberLiteral => UniversalTokenRole::Literal,
            Self::Identifier => UniversalTokenRole::Name,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Eof | Self::Error => UniversalTokenRole::None,
            _ => UniversalTokenRole::Operator,
        }
    }
}
