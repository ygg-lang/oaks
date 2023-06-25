use oak_core::{Token, TokenType, UniversalTokenRole};

/// Handlebars token.
pub type _HandlebarsToken = Token<HandlebarsTokenType>;

/// Handlebars token type definition.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HandlebarsTokenType {
    // --- Tokens ---
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,

    /// Comment.
    Comment,

    // Handlebars specific tokens
    /// `{{`.
    Open,
    /// `}}`.
    Close,
    /// `{{{`.
    OpenUnescaped,
    /// `}}}`.
    CloseUnescaped,
    /// `{{{{`.
    OpenRawBlock,
    /// `}}}}`.
    CloseRawBlock,
    /// `{{{{/`.
    OpenEndRawBlock,
    /// `{{#`.
    OpenBlock,
    /// `{{^`.
    OpenInverseBlock,
    /// `{{/`.
    CloseBlock,
    /// `{{>`.
    OpenPartial,
    /// `{{!`.
    OpenComment,
    /// `{{!--`.
    OpenCommentBlock,
    /// `--}}`.
    CloseCommentBlock,

    // Keywords
    /// `else` keyword.
    Else,

    // Identifiers and literals
    /// Identifier.
    Identifier,
    /// String literal.
    StringLiteral,
    /// Number literal.
    NumberLiteral,
    /// Boolean literal.
    BooleanLiteral,
    /// `.` symbol.
    Dot,
    /// `/` symbol.
    Slash,
    /// `#` symbol.
    Hash,
    /// `@` symbol.
    At,
    /// `|` symbol.
    Pipe,
    /// `=` symbol.
    Equal,
    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `^` symbol.
    Caret,

    // Content
    /// HTML/text content.
    Content,

    // --- Elements ---
    /// Root node.
    Root,
    /// Mustache expression.
    Mustache,
    /// Block expression.
    Block,
    /// Inverse block expression.
    InverseBlock,
    /// Partial template reference.
    Partial,
    /// Comment node.
    CommentNode,
    /// Content node.
    ContentNode,
    /// Expression node.
    Expression,
    /// Path node.
    Path,
    /// Parameter node.
    Parameter,
    /// Else block.
    ElseBlock,

    // --- Special ---
    /// Error.
    Error,
    /// End of stream.
    Eof,
}

impl TokenType for HandlebarsTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}
