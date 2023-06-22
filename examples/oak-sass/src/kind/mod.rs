use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// Sass 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SassSyntaxKind {
    // 节点种类
    SourceFile,
    ErrorNode,

    // 基础词法种类
    Whitespace,
    Newline,
    Error,
    Eof,

    // 标识符和字面量
    Identifier,
    NumberLiteral,
    FloatLiteral,
    StringLiteral,
    ColorLiteral,

    // Sass 关键字
    Import,
    Include,
    Extend,
    Mixin,
    Function,
    Return,
    If,
    Else,
    ElseIf,
    For,
    Each,
    While,
    Default,
    Important,
    Optional,
    Global,

    // CSS 属性关键字
    Color,
    Background,
    Border,
    Margin,
    Padding,
    Width,
    Height,
    Display,
    Position,
    Float,
    Clear,

    // 操作符
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %
    Eq,      // =
    EqEq,    // ==
    Ne,      // !=
    Lt,      // <
    Le,      // <=
    Gt,      // >
    Ge,      // >=
    And,     // and
    Or,      // or
    Not,     // not

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // 标点符号
    Semicolon,   // ;
    Colon,       // :
    Comma,       // ,
    Dot,         // .
    Hash,        // #
    Dollar,      // $
    At,          // @
    Ampersand,   // &
    Exclamation, // !
    Question,    // ?
    Tilde,       // ~

    // 注释
    LineComment,  // //
    BlockComment, // /* */

    // Sass 特殊符号
    Interpolation, // #{}
    Variable,      // $variable
    Selector,      // CSS 选择器
    Property,      // CSS 属性
    Value,         // CSS 值
    Unit,          // px, em, rem 等单位
}

impl TokenType for SassSyntaxKind {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::None,
            Self::Identifier | Self::Variable => UniversalTokenRole::Name,
            Self::NumberLiteral | Self::FloatLiteral | Self::StringLiteral | Self::ColorLiteral => UniversalTokenRole::Literal,
            Self::Import | Self::Include | Self::Extend | Self::Mixin | Self::Function | Self::Return | Self::If | Self::Else | Self::ElseIf | Self::For | Self::Each | Self::While | Self::Default | Self::Important | Self::Optional | Self::Global => {
                UniversalTokenRole::Keyword
            }
            Self::Plus | Self::Minus | Self::Star | Self::Slash | Self::Percent | Self::Eq | Self::EqEq | Self::Ne | Self::Lt | Self::Le | Self::Gt | Self::Ge | Self::And | Self::Or | Self::Not => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for SassSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::ErrorNode => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
