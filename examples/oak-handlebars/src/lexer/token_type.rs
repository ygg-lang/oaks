use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type HandlebarsToken = Token<HandlebarsTokenType>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HandlebarsTokenType {
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
