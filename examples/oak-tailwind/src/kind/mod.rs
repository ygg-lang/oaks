/// 统一 Tailwind 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TailwindSyntaxKind {
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

impl oak_core::TokenType for TailwindSyntaxKind {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = TailwindSyntaxKind::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            Self::Comment => oak_core::UniversalTokenRole::Comment,
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl oak_core::ElementType for TailwindSyntaxKind {
    type Role = oak_core::UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::Document | Self::Template => oak_core::UniversalElementRole::Root,
            Self::Error | Self::ErrorNode => oak_core::UniversalElementRole::Error,
            _ => oak_core::UniversalElementRole::None,
        }
    }
}

impl core::fmt::Display for TailwindSyntaxKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TailwindSyntaxKind::Root => f.write_str("Root"),
            TailwindSyntaxKind::Document => f.write_str("Document"),
            TailwindSyntaxKind::Template => f.write_str("Template"),
            TailwindSyntaxKind::Block => f.write_str("Block"),
            TailwindSyntaxKind::Variable => f.write_str("Variable"),
            TailwindSyntaxKind::Filter => f.write_str("Filter"),
            TailwindSyntaxKind::Function => f.write_str("Function"),
            TailwindSyntaxKind::Tag => f.write_str("Tag"),
            TailwindSyntaxKind::Comment => f.write_str("Comment"),
            TailwindSyntaxKind::Text => f.write_str("Text"),
            TailwindSyntaxKind::Expression => f.write_str("Expression"),
            TailwindSyntaxKind::String => f.write_str("String"),
            TailwindSyntaxKind::Number => f.write_str("Number"),
            TailwindSyntaxKind::Boolean => f.write_str("Boolean"),
            TailwindSyntaxKind::Null => f.write_str("Null"),
            TailwindSyntaxKind::Array => f.write_str("Array"),
            TailwindSyntaxKind::Object => f.write_str("Object"),
            TailwindSyntaxKind::Identifier => f.write_str("Identifier"),
            TailwindSyntaxKind::Operator => f.write_str("Operator"),
            TailwindSyntaxKind::ErrorNode => f.write_str("ErrorNode"),
            TailwindSyntaxKind::LeftBrace => f.write_str("{"),
            TailwindSyntaxKind::RightBrace => f.write_str("}"),
            TailwindSyntaxKind::LeftBracket => f.write_str("["),
            TailwindSyntaxKind::RightBracket => f.write_str("]"),
            TailwindSyntaxKind::DoubleLeftBrace => f.write_str("{{"),
            TailwindSyntaxKind::DoubleRightBrace => f.write_str("}}"),
            TailwindSyntaxKind::LeftBracePercent => f.write_str("{%"),
            TailwindSyntaxKind::PercentRightBrace => f.write_str("%}"),
            TailwindSyntaxKind::LeftParen => f.write_str("("),
            TailwindSyntaxKind::RightParen => f.write_str(")"),
            TailwindSyntaxKind::Pipe => f.write_str("|"),
            TailwindSyntaxKind::Comma => f.write_str(","),
            TailwindSyntaxKind::Dot => f.write_str("."),
            TailwindSyntaxKind::Colon => f.write_str(":"),
            TailwindSyntaxKind::Semicolon => f.write_str(";"),
            TailwindSyntaxKind::Eq => f.write_str("="),
            TailwindSyntaxKind::Plus => f.write_str("+"),
            TailwindSyntaxKind::Minus => f.write_str("-"),
            TailwindSyntaxKind::Star => f.write_str("*"),
            TailwindSyntaxKind::Slash => f.write_str("/"),
            TailwindSyntaxKind::Percent => f.write_str("%"),
            TailwindSyntaxKind::Bang => f.write_str("!"),
            TailwindSyntaxKind::Question => f.write_str("?"),
            TailwindSyntaxKind::Lt => f.write_str("<"),
            TailwindSyntaxKind::Gt => f.write_str(">"),
            TailwindSyntaxKind::Amp => f.write_str("&"),
            TailwindSyntaxKind::Caret => f.write_str("^"),
            TailwindSyntaxKind::Tilde => f.write_str("~"),
            TailwindSyntaxKind::Whitespace => f.write_str("Whitespace"),
            TailwindSyntaxKind::Eof => f.write_str("Eof"),
            TailwindSyntaxKind::Error => f.write_str("Error"),
        }
    }
}
