use oak_core::SyntaxKind;

/// DOT 语法种类（Graphviz
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        todo!()
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
