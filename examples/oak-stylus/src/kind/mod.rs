/// Stylus 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum StylusSyntaxKind {
    // 节点种类
    Root,
    Document,
    Rule,
    Selector,
    Property,
    Value,
    Block,

    // 词法种类
    Identifier, // body, div, color, etc.
    Number,     // 10, 100px, 1.5em
    String,     // "Arial", 'Helvetica'
    Color,      // #fff, red, rgb(255,0,0)
    LeftBrace,  // {
    RightBrace, // }
    LeftParen,  // (
    RightParen, // )
    Colon,      // :
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    Hash,       // #
    Ampersand,  // &
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Equal,      // =
    Whitespace,
    Newline,
    Comment,
    Eof,
    Error,
}

impl oak_core::TokenType for StylusSyntaxKind {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = StylusSyntaxKind::Eof;

    fn role(&self) -> Self::Role {
        match self {
            StylusSyntaxKind::Identifier => oak_core::UniversalTokenRole::Name,
            StylusSyntaxKind::Number => oak_core::UniversalTokenRole::Literal,
            StylusSyntaxKind::String => oak_core::UniversalTokenRole::Literal,
            StylusSyntaxKind::Color => oak_core::UniversalTokenRole::Literal,
            StylusSyntaxKind::LeftBrace | StylusSyntaxKind::RightBrace | StylusSyntaxKind::LeftParen | StylusSyntaxKind::RightParen | StylusSyntaxKind::Colon | StylusSyntaxKind::Semicolon | StylusSyntaxKind::Comma => {
                oak_core::UniversalTokenRole::Punctuation
            }
            StylusSyntaxKind::Dot | StylusSyntaxKind::Hash | StylusSyntaxKind::Ampersand | StylusSyntaxKind::Plus | StylusSyntaxKind::Minus | StylusSyntaxKind::Star | StylusSyntaxKind::Slash | StylusSyntaxKind::Percent | StylusSyntaxKind::Equal => {
                oak_core::UniversalTokenRole::Operator
            }
            StylusSyntaxKind::Whitespace | StylusSyntaxKind::Newline => oak_core::UniversalTokenRole::Whitespace,
            StylusSyntaxKind::Comment => oak_core::UniversalTokenRole::Comment,
            StylusSyntaxKind::Error => oak_core::UniversalTokenRole::Error,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl oak_core::ElementType for StylusSyntaxKind {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            StylusSyntaxKind::Root => oak_core::UniversalElementRole::Root,
            StylusSyntaxKind::Document => oak_core::UniversalElementRole::Container,
            StylusSyntaxKind::Rule => oak_core::UniversalElementRole::Statement,
            StylusSyntaxKind::Selector => oak_core::UniversalElementRole::Binding,
            StylusSyntaxKind::Property => oak_core::UniversalElementRole::AttributeKey,
            StylusSyntaxKind::Value => oak_core::UniversalElementRole::Value,
            StylusSyntaxKind::Block => oak_core::UniversalElementRole::Container,
            _ => oak_core::UniversalElementRole::Value,
        }
    }
}

impl StylusSyntaxKind {
    /// 检查是否为值类型
    pub fn is_value(self) -> bool {
        matches!(self, StylusSyntaxKind::Number | StylusSyntaxKind::String | StylusSyntaxKind::Color | StylusSyntaxKind::Identifier)
    }

    /// 检查是否为操作符
    pub fn is_operator(self) -> bool {
        matches!(self, StylusSyntaxKind::Plus | StylusSyntaxKind::Minus | StylusSyntaxKind::Star | StylusSyntaxKind::Slash | StylusSyntaxKind::Percent | StylusSyntaxKind::Equal)
    }
}

impl core::fmt::Display for StylusSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match self {
            StylusSyntaxKind::Root => "Root",
            StylusSyntaxKind::Document => "Document",
            StylusSyntaxKind::Rule => "Rule",
            StylusSyntaxKind::Selector => "Selector",
            StylusSyntaxKind::Property => "Property",
            StylusSyntaxKind::Value => "Value",
            StylusSyntaxKind::Block => "Block",
            StylusSyntaxKind::Identifier => "Identifier",
            StylusSyntaxKind::Number => "Number",
            StylusSyntaxKind::String => "String",
            StylusSyntaxKind::Color => "Color",
            StylusSyntaxKind::LeftBrace => "LeftBrace",
            StylusSyntaxKind::RightBrace => "RightBrace",
            StylusSyntaxKind::LeftParen => "LeftParen",
            StylusSyntaxKind::RightParen => "RightParen",
            StylusSyntaxKind::Colon => "Colon",
            StylusSyntaxKind::Semicolon => "Semicolon",
            StylusSyntaxKind::Comma => "Comma",
            StylusSyntaxKind::Dot => "Dot",
            StylusSyntaxKind::Hash => "Hash",
            StylusSyntaxKind::Ampersand => "Ampersand",
            StylusSyntaxKind::Plus => "Plus",
            StylusSyntaxKind::Minus => "Minus",
            StylusSyntaxKind::Star => "Star",
            StylusSyntaxKind::Slash => "Slash",
            StylusSyntaxKind::Percent => "Percent",
            StylusSyntaxKind::Equal => "Equal",
            StylusSyntaxKind::Whitespace => "Whitespace",
            StylusSyntaxKind::Newline => "Newline",
            StylusSyntaxKind::Comment => "Comment",
            StylusSyntaxKind::Eof => "Eof",
            StylusSyntaxKind::Error => "Error",
        };
        write!(f, "{}", name)
    }
}
