use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HandlebarsSyntaxKind {
    // --- 词法标记 (Tokens) ---
    // 空白和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // Handlebars 特殊标记
    Open,              // {{
    Close,             // }}
    OpenUnescaped,     // {{{
    CloseUnescaped,    // }}}
    OpenRawBlock,      // {{{{
    CloseRawBlock,     // }}}}
    OpenEndRawBlock,   // {{{{/
    OpenBlock,         // {{#
    OpenInverseBlock,  // {{^
    CloseBlock,        // {{/
    OpenPartial,       // {{>
    OpenComment,       // {{!
    OpenCommentBlock,  // {{!--
    CloseCommentBlock, // --}}

    // 关键字
    Else, // else

    // 标识符和字面量
    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    Dot,          // .
    Slash,        // /
    Hash,         // #
    At,           // @
    Pipe,         // |
    Equal,        // =
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Caret,        // ^

    // 内容
    Content, // HTML/text content outside of handlebars expressions

    // --- 节点种类 (Elements) ---
    Root,
    Mustache,
    Block,
    InverseBlock,
    Partial,
    CommentNode, // Avoid conflict with Comment token
    ContentNode, // Avoid conflict with Content token
    Expression,
    SubExpression,
    Path,
    Parameter,
    ElseBlock,

    // 特殊
    Error,
    Eof,
}

impl TokenType for HandlebarsSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::NumberLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,
            Self::Open
            | Self::Close
            | Self::OpenUnescaped
            | Self::CloseUnescaped
            | Self::OpenRawBlock
            | Self::CloseRawBlock
            | Self::OpenEndRawBlock
            | Self::OpenBlock
            | Self::OpenInverseBlock
            | Self::CloseBlock
            | Self::OpenPartial
            | Self::OpenComment
            | Self::OpenCommentBlock
            | Self::CloseCommentBlock => UniversalTokenRole::Operator,
            Self::Else => UniversalTokenRole::Keyword,
            Self::Dot | Self::Slash | Self::Hash | Self::At | Self::Pipe | Self::Equal | Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::Caret => UniversalTokenRole::Operator,
            Self::Content => UniversalTokenRole::None,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for HandlebarsSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Mustache => UniversalElementRole::Expression,
            Self::Block | Self::InverseBlock => UniversalElementRole::Container,
            Self::Partial => UniversalElementRole::Container,
            Self::CommentNode => UniversalElementRole::Documentation,
            Self::ContentNode => UniversalElementRole::None,
            Self::Expression | Self::SubExpression => UniversalElementRole::Expression,
            Self::Path => UniversalElementRole::Binding,
            Self::Parameter => UniversalElementRole::Expression,
            Self::ElseBlock => UniversalElementRole::Container,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
