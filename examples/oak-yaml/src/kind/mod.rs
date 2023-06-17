use oak_core::SyntaxKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum YamlSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,
    Error,
    Eof,

    // YAML 特定语法
    DocumentStart,      // ---
    DocumentEnd,        // ...
    BlockSequenceEntry, // -
    FlowSequenceStart,  // [
    FlowSequenceEnd,    // ]
    FlowMappingStart,   // {
    FlowMappingEnd,     // }
    BlockEntry,         // -
    KeyIndicator,       // ?
    ValueIndicator,     // :
    BlockScalar,        // |, >
    FoldedScalar,       // >
    LiteralScalar,      // |

    // 字面    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符和    Identifier,
    Key,
    Value,

    // 标点符号
    Comma,       // ,
    Pipe,        // |
    GreaterThan, // >
    Ampersand,   // &
    Asterisk,    // *
    Exclamation, // !
    Percent,     // %

    // 引号
    SingleQuote, // '
    DoubleQuote, // "

    // 标签和锚    Tag,                // !tag
    Anchor, // &anchor
    Alias,  // *alias

    // 指令
    Directive, // %YAML, %TAG
}

impl SyntaxKind for YamlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
