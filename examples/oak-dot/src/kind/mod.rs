use oak_core::SyntaxKind;

/// DOT 语法种类（Graphviz
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum DotSyntaxKind {
    // 基本 kind
    Identifier,
    String,
    Number,
    Whitespace,
    Newline,

    // DOT 关键字
    Graph,
    Digraph,
    Subgraph,
    Node,
    Edge,
    Strict,

    // 操作符
    Arrow,     // ->
    Line,      // --
    Equal,     // =
    Semicolon, // ;
    Comma,     // ,

    // 分隔符
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // 注释
    Comment,

    // 特殊
    Error,
    Eof,
}

impl SyntaxKind for DotSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error | Self::Eof)
    }
}
