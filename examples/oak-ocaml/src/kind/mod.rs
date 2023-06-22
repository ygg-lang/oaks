use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
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

    // Element kinds
    Root,
    ModuleDef,
    LetBinding,
    MatchExpr,
    FunctionDef,
    TypeDefinition,
    Expression,

    // 错误和结束
    Error,
    Eof,
}

impl TokenType for OCamlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for OCamlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::ModuleDef | Self::LetBinding | Self::TypeDefinition => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }
}
