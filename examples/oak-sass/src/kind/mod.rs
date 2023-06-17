use oak_core::SyntaxKind;

/// Sass 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SassSyntaxKind {
    // 节点种类
    SourceFile,

    // 基础词法种类
    Whitespace,
    Newline,
    Error,
    Eof,
    ErrorNode,

    // 标识符和字面量
    Identifier,
    NumberLiteral,
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

impl SyntaxKind for SassSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::SourceFile | Self::ErrorNode)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::SourceFile | Self::ErrorNode)
    }
}
