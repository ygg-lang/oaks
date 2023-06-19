/// Stylus 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl oak_core::SyntaxKind for StylusSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, StylusSyntaxKind::Whitespace | StylusSyntaxKind::Comment | StylusSyntaxKind::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, StylusSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, StylusSyntaxKind::Whitespace | StylusSyntaxKind::Newline)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::Identifier
                | StylusSyntaxKind::Number
                | StylusSyntaxKind::String
                | StylusSyntaxKind::Color
                | StylusSyntaxKind::LeftBrace
                | StylusSyntaxKind::RightBrace
                | StylusSyntaxKind::LeftParen
                | StylusSyntaxKind::RightParen
                | StylusSyntaxKind::Colon
                | StylusSyntaxKind::Semicolon
                | StylusSyntaxKind::Comma
                | StylusSyntaxKind::Dot
                | StylusSyntaxKind::Hash
                | StylusSyntaxKind::Ampersand
                | StylusSyntaxKind::Plus
                | StylusSyntaxKind::Minus
                | StylusSyntaxKind::Star
                | StylusSyntaxKind::Slash
                | StylusSyntaxKind::Percent
                | StylusSyntaxKind::Equal
                | StylusSyntaxKind::Whitespace
                | StylusSyntaxKind::Newline
                | StylusSyntaxKind::Comment
                | StylusSyntaxKind::Eof
                | StylusSyntaxKind::Error
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::Root
                | StylusSyntaxKind::Document
                | StylusSyntaxKind::Rule
                | StylusSyntaxKind::Selector
                | StylusSyntaxKind::Property
                | StylusSyntaxKind::Value
                | StylusSyntaxKind::Block
        )
    }
}

impl StylusSyntaxKind {
    /// 检查是否为值类型
    pub fn is_value(self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::Number | StylusSyntaxKind::String | StylusSyntaxKind::Color | StylusSyntaxKind::Identifier
        )
    }

    /// 检查是否为操作符
    pub fn is_operator(self) -> bool {
        matches!(
            self,
            StylusSyntaxKind::Plus
                | StylusSyntaxKind::Minus
                | StylusSyntaxKind::Star
                | StylusSyntaxKind::Slash
                | StylusSyntaxKind::Percent
                | StylusSyntaxKind::Equal
        )
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
