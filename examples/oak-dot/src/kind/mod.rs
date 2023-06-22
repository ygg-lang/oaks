use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type DotToken = Token<DotSyntaxKind>;

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
    Root,
    Error,
    Eof,
}

impl TokenType for DotSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Graph | Self::Digraph | Self::Subgraph | Self::Node | Self::Edge | Self::Strict => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::String => UniversalTokenRole::Literal,
            Self::Number => UniversalTokenRole::Literal,
            Self::Arrow | Self::Line | Self::Equal => UniversalTokenRole::Operator,
            Self::Semicolon | Self::Comma | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::LeftParen | Self::RightParen => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for DotSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
