use oak_core::{ElementType, UniversalElementRole};

/// Handlebars element type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HandlebarsElementType {
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
    /// Sub-expression node.
    SubExpression,
    /// Path node.
    Path,
    /// Parameter node.
    Parameter,
    /// Else block.
    ElseBlock,

    // --- Special ---
    /// Error node.
    Error,
    /// End of stream.
    Eof,
}

impl ElementType for HandlebarsElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::HandlebarsTokenType> for HandlebarsElementType {
    fn from(token: crate::lexer::token_type::HandlebarsTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
