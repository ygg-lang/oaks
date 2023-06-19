use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SchemeSyntaxKind {
    // 空白字符和换行
    Whitespace,
    Newline,
    Comment,

    // 注释
    LineComment,

    // 字面量
    NumberLiteral,
    StringLiteral,
    CharacterLiteral,
    BooleanLiteral,

    // 标识符和符号
    Identifier,
    Symbol,

    // 关键字
    Keyword,
    Define,
    Lambda,
    If,
    Cond,
    Case,
    Let,
    LetStar,
    Letrec,
    Begin,
    Do,
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
    And,
    Or,
    Not,
    Set,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Dot,

    // 特殊符号
    Hash,
    Quote_,
    Quasiquote_,
    Unquote_,
    UnquoteSplicing_,

    // 错误和结束
    Error,
    Eof,
}

impl SyntaxKind for SchemeSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
