/// 统一 Twig 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TwigSyntaxKind {
    // 节点种类
    Root,
    Document,
    Template,
    Block,
    Variable,
    Filter,
    Function,
    Tag,
    Comment,
    Text,
    Expression,

    // 细分字面量类
    String,
    Number,
    Boolean,
    Null,
    Array,
    Object,
    Identifier,
    Operator,
    ErrorNode,

    // 词法种类
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    DoubleLeftBrace,   // {{
    DoubleRightBrace,  // }}
    LeftBracePercent,  // {%
    PercentRightBrace, // %}
    LeftParen,         // (
    RightParen,        // )
    Pipe,              // |
    Comma,             // ,
    Dot,               // .
    Colon,             // :
    Semicolon,         // ;
    Eq,                // =
    Plus,              // +
    Minus,             // -
    Star,              // *
    Slash,             // /
    Percent,           // %
    Bang,              // !
    Question,          // ?
    Lt,                // <
    Gt,                // >
    Amp,               // &
    Caret,             // ^
    Tilde,             // ~
    Whitespace,
    Eof,
    Error,
}

impl oak_core::TokenType for TwigSyntaxKind {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TwigSyntaxKind::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl oak_core::ElementType for TwigSyntaxKind {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Document | Self::Template => oak_core::UniversalElementRole::Root,
            Self::Error | Self::ErrorNode => oak_core::UniversalElementRole::Error,
            _ => oak_core::UniversalElementRole::None,
        }
    }
}

impl core::fmt::Display for TwigSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TwigSyntaxKind::Root => f.write_str("Root"),
            TwigSyntaxKind::Document => f.write_str("Document"),
            TwigSyntaxKind::Template => f.write_str("Template"),
            TwigSyntaxKind::Block => f.write_str("Block"),
            TwigSyntaxKind::Variable => f.write_str("Variable"),
            TwigSyntaxKind::Filter => f.write_str("Filter"),
            TwigSyntaxKind::Function => f.write_str("Function"),
            TwigSyntaxKind::Tag => f.write_str("Tag"),
            TwigSyntaxKind::Comment => f.write_str("Comment"),
            TwigSyntaxKind::Text => f.write_str("Text"),
            TwigSyntaxKind::Expression => f.write_str("Expression"),
            TwigSyntaxKind::String => f.write_str("String"),
            TwigSyntaxKind::Number => f.write_str("Number"),
            TwigSyntaxKind::Boolean => f.write_str("Boolean"),
            TwigSyntaxKind::Null => f.write_str("Null"),
            TwigSyntaxKind::Array => f.write_str("Array"),
            TwigSyntaxKind::Object => f.write_str("Object"),
            TwigSyntaxKind::Identifier => f.write_str("Identifier"),
            TwigSyntaxKind::Operator => f.write_str("Operator"),
            TwigSyntaxKind::ErrorNode => f.write_str("ErrorNode"),
            TwigSyntaxKind::LeftBrace => f.write_str("{"),
            TwigSyntaxKind::RightBrace => f.write_str("}"),
            TwigSyntaxKind::LeftBracket => f.write_str("["),
            TwigSyntaxKind::RightBracket => f.write_str("]"),
            TwigSyntaxKind::DoubleLeftBrace => f.write_str("{{"),
            TwigSyntaxKind::DoubleRightBrace => f.write_str("}}"),
            TwigSyntaxKind::LeftBracePercent => f.write_str("{%"),
            TwigSyntaxKind::PercentRightBrace => f.write_str("%}"),
            TwigSyntaxKind::LeftParen => f.write_str("("),
            TwigSyntaxKind::RightParen => f.write_str(")"),
            TwigSyntaxKind::Pipe => f.write_str("|"),
            TwigSyntaxKind::Comma => f.write_str(","),
            TwigSyntaxKind::Dot => f.write_str("."),
            TwigSyntaxKind::Colon => f.write_str(":"),
            TwigSyntaxKind::Semicolon => f.write_str(";"),
            TwigSyntaxKind::Eq => f.write_str("="),
            TwigSyntaxKind::Plus => f.write_str("+"),
            TwigSyntaxKind::Minus => f.write_str("-"),
            TwigSyntaxKind::Star => f.write_str("*"),
            TwigSyntaxKind::Slash => f.write_str("/"),
            TwigSyntaxKind::Percent => f.write_str("%"),
            TwigSyntaxKind::Bang => f.write_str("!"),
            TwigSyntaxKind::Question => f.write_str("?"),
            TwigSyntaxKind::Lt => f.write_str("<"),
            TwigSyntaxKind::Gt => f.write_str(">"),
            TwigSyntaxKind::Amp => f.write_str("&"),
            TwigSyntaxKind::Caret => f.write_str("^"),
            TwigSyntaxKind::Tilde => f.write_str("~"),
            TwigSyntaxKind::Whitespace => f.write_str("Whitespace"),
            TwigSyntaxKind::Eof => f.write_str("EOF"),
            TwigSyntaxKind::Error => f.write_str("Error"),
        }
    }
}
