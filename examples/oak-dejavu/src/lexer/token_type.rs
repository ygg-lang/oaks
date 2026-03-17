use crate::lexer::DejavuKeywords;
use oak_core::{Token, TokenType, UniversalTokenRole};

/// Alias for `Token<DejavuTokenType>`.
pub type DejavuToken = Token<DejavuTokenType>;
/// Alias for the syntax kind type.
pub type DejavuSyntaxKind = DejavuTokenType;

/// Token types for the Dejavu language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuTokenType {
    /// End of file token.
    Eof,
    /// Whitespace token.
    Whitespace,
    /// Error token.
    Error,

    // Literals
    /// Integer literal token.
    IntegerLiteral,
    /// Float literal token.
    FloatLiteral,
    /// Boolean literal token.
    BoolLiteral,
    /// String literal token.
    StringLiteral,
    /// Character literal token.
    CharLiteral,
    /// Identifier token.
    Identifier,
    /// Label token.
    Label,

    // Comments
    /// Line comment token.
    LineComment,
    /// Block comment token.
    BlockComment,

    // Keywords
    /// Keyword token.
    Keyword(DejavuKeywords),

    // Operators & Punctuation
    /// Ampersand operator `&`.
    Ampersand,
    /// Logical and operator `&&`.
    AndAnd,
    /// Arrow operator `->`.
    Arrow,
    /// At symbol `@`.
    At,
    /// Bang operator `!`.
    Bang,
    /// Bolt symbol `@`.
    Bolt,
    /// Caret operator `^`.
    Caret,
    /// Colon operator `:`.
    Colon,
    /// Path separator `::`.
    ColonColon,
    /// Assignment operator `:=`.
    ColonEq,
    /// Comma separator `,`.
    Comma,
    /// Dollar symbol `$`.
    Dollar,
    /// Dot operator `.`.
    Dot,
    /// Assignment operator `=`.
    Eq,
    /// Equality operator `==`.
    EqEq,
    /// Greater than or equal operator `>=`.
    GreaterEq,
    /// Greater than operator `>`.
    GreaterThan,
    /// Left brace `{`.
    LeftBrace,
    /// Left bracket `[`.
    LeftBracket,
    /// Left parenthesis `(`.
    LeftParen,
    /// Left shift operator `<<`.
    LeftShift,
    /// Less than or equal operator `<=`.
    LessEq,
    /// Less than operator `<`.
    LessThan,
    /// Left mathematical angle bracket `⟨` (U+27E8).
    LeftAngle,
    /// Right mathematical angle bracket `⟩` (U+27E9).
    RightAngle,
    /// Minus operator `-`.
    Minus,
    /// Minus assignment operator `-=`.
    MinusEq,
    /// Decrement operator `--`.
    MinusMinus,
    /// Not equal operator `!=`.
    NotEq,
    /// Logical or operator `||`.
    OrOr,
    /// Percent operator `%`.
    Percent,
    /// Percent assignment operator `%=`.
    PercentEq,
    /// Logical OR operator `|`.
    Or,
    /// Pipe operator `|>`.
    Pipe,
    /// Plus operator `+`.
    Plus,
    /// Plus assignment operator `+=`.
    PlusEq,
    /// Increment operator `++`.
    PlusPlus,
    /// Question operator `?`.
    Question,
    /// Right brace `}`.
    RightBrace,
    /// Right bracket `]`.
    RightBracket,
    /// Right parenthesis `)`.
    RightParen,
    /// Right shift operator `>>`.
    RightShift,
    /// Semicolon separator `;`.
    Semicolon,
    /// Slash operator `/`.
    Slash,
    /// Slash assignment operator `/=`.
    SlashEq,
    /// Star operator `*`.
    Star,
    /// Star assignment operator `*=`.
    StarEq,
    /// Tilde operator `~`.
    Tilde,
    /// Underscore token `_`.
    Underscore,

    // Template Specific
    /// Code start token.
    CodeStart,
    /// Code end token.
    CodeEnd,
    /// String part token for templates.
    StringPart,
    /// Template comment start token, default is `<#`.
    TemplateCommentStart,
    /// Template comment end token, default is `#>`.
    TemplateCommentEnd,
    /// Template control start token, default is `<$`.
    TemplateControlStart,
    /// Template control end token, default is `$>`.
    TemplateControlEnd,
}

impl TokenType for DejavuTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Keyword(_) => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::StringPart => UniversalTokenRole::Literal,
            Self::IntegerLiteral | Self::FloatLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}
