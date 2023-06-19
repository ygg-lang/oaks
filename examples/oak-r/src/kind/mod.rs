#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum RSyntaxKind {
    // 空白符和换行
    Whitespace,
    Newline,

    // 注释
    Comment,

    // 字面量
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,
    NullLiteral,

    // 标识符
    Identifier,

    // 关键字
    If,
    Else,
    For,
    In,
    While,
    Repeat,
    Next,
    Break,
    Function,
    Return,
    True,
    False,
    Null,
    Inf,
    NaN,
    NA,

    // 运算符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    AndAnd,
    OrOr,
    Tilde,
    LeftArrow,
    RightArrow,
    DoubleLeftArrow,
    DoubleRightArrow,
    Pipe,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    TripleColon,
    Dot,
    Dollar,
    At,

    // 根节点
    Root,

    // 错误和结束
    Error,
    Eof,
}
