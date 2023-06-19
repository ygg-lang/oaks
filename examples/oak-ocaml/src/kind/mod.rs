use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type OCamlToken = Token<OCamlSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OCamlSyntaxKind {
    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 关键字
    And,
    As,
    Assert,
    Begin,
    Class,
    Constraint,
    Do,
    Done,
    Downto,
    Else,
    End,
    Exception,
    External,
    False,
    For,
    Fun,
    Function,
    Functor,
    If,
    In,
    Include,
    Inherit,
    Initializer,
    Lazy,
    Let,
    Match,
    Method,
    Module,
    Mutable,
    New,
    Object,
    Of,
    Open,
    Or,
    Private,
    Rec,
    Sig,
    Struct,
    Then,
    To,
    True,
    Try,
    Type,
    Val,
    Virtual,
    When,
    While,
    With,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LeftArrow,
    RightArrow,
    OrOr,
    AndAnd,
    ColonColon,
    Pipe,
    Ampersand,
    Bang,
    Question,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Caret,
    Tilde,
    At,
    Hash,
    Dollar,
    Backtick,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    // 特殊

    // 错误和结束
    Error,
    Eof,
}

impl SyntaxKind for OCamlSyntaxKind {
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
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error | Self::Eof)
    }
}
