use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HandlebarsElementType {
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
