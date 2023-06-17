#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HandlebarsSyntaxKind {
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
    CloseBlock,        // {{/
    OpenPartial,       // {{>
    OpenComment,       // {{!
    OpenCommentBlock,  // {{!--
    CloseCommentBlock, // --}}

    // 标识符和字面量
    Identifier,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,

    // 操作符和分隔符
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

    // 内容
    Content, // HTML/text content outside of handlebars expressions

    // 特殊
    Error,
    Eof,
}
