/// 统一 Twig 语法种类（包含节点与词法）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl oak_core::SyntaxKind for TwigSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, TwigSyntaxKind::Whitespace | TwigSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, TwigSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, TwigSyntaxKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            TwigSyntaxKind::Root
                | TwigSyntaxKind::Variable
                | TwigSyntaxKind::Block
                | TwigSyntaxKind::Expression
                | TwigSyntaxKind::Array
                | TwigSyntaxKind::Object
                | TwigSyntaxKind::ErrorNode
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            TwigSyntaxKind::Root
                | TwigSyntaxKind::Variable
                | TwigSyntaxKind::Block
                | TwigSyntaxKind::Expression
                | TwigSyntaxKind::Array
                | TwigSyntaxKind::Object
                | TwigSyntaxKind::ErrorNode
        )
    }
}

impl TwigSyntaxKind {
    /// 判断是否为值类型
    pub fn is_value(self) -> bool {
        matches!(
            self,
            TwigSyntaxKind::String
                | TwigSyntaxKind::Number
                | TwigSyntaxKind::Boolean
                | TwigSyntaxKind::Null
                | TwigSyntaxKind::Array
                | TwigSyntaxKind::Object
        )
    }

    /// 判断是否为字面量
    pub fn is_literal(self) -> bool {
        matches!(self, TwigSyntaxKind::String | TwigSyntaxKind::Number | TwigSyntaxKind::Boolean | TwigSyntaxKind::Null)
    }

    /// 判断是否为表达式
    pub fn is_expression(self) -> bool {
        matches!(
            self,
            TwigSyntaxKind::Expression | TwigSyntaxKind::Variable | TwigSyntaxKind::Filter | TwigSyntaxKind::Function
        )
    }

    /// 判断是否为块级元素
    pub fn is_block(self) -> bool {
        matches!(self, TwigSyntaxKind::Block | TwigSyntaxKind::Tag)
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
