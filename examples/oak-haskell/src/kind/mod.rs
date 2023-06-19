use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum HaskellSyntaxKind {
    // 空白和注
    Whitespace,
    Newline,
    Comment,

    // 关键
    Case,
    Class,
    Data,
    Default,
    Deriving,
    Do,
    Else,
    Foreign,
    If,
    Import,
    In,
    Infix,
    Infixl,
    Infixr,
    Instance,
    Let,
    Module,
    Newtype,
    Of,
    Then,
    Type,
    Where,
    Underscore,

    // 特殊关键
    As,
    Qualified,
    Hiding,

    // 标识符和字面
    Identifier,
    Constructor,
    Number,
    Integer,
    Float,
    String,
    StringLiteral,
    Char,
    CharLiteral,

    // 操作
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Arrow,
    LeftArrow,
    DoubleArrow,
    Pipe,
    Ampersand,
    Bang,
    Exclamation,
    Question,
    Colon,
    DoubleColon,
    Semicolon,
    Comma,
    Dot,
    DoubleDot,
    DotDot,
    Dollar,
    At,
    Tilde,
    Backslash,
    Append,

    // 分隔
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // 特殊符号
    Quote,
    Backquote,
    Backtick,

    // 错误和结
    Root,
    Error,
    Eof,
}

impl oak_core::SyntaxKind for HaskellSyntaxKind {
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
        !matches!(self, Self::Root | Self::Error)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::Error)
    }
}
