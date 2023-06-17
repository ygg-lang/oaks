use oak_core::SyntaxKind;

/// Kotlin 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum KotlinSyntaxKind {
    // 节点种类
    Root,
    SourceFile,

    // 关键    Class,
    Fun,
    Val,
    Var,
    If,
    Else,
    When,
    For,
    While,
    Return,
    Break,
    Continue,
    Try,
    Catch,
    Finally,
    Throw,
    Import,
    Package,
    Public,
    Private,
    Protected,
    Internal,
    Abstract,
    Final,
    Open,
    Override,
    Companion,
    Object,
    Interface,
    Enum,
    Data,
    Sealed,
    Inline,
    Suspend,
    Operator,
    Infix,
    Tailrec,
    External,
    Annotation,
    Crossinline,
    Noinline,
    Reified,
    Vararg,
    Out,
    In,
    Is,
    As,
    This,
    Super,
    Null,
    True,
    False,

    // 标识符和字面量
    Identifier,
    Keyword,
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    IntLiteral,
    FloatLiteral,
    BooleanLiteral,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equals,
    Less,
    Greater,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Exclamation,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    AndAnd,
    OrOr,
    Not,
    Question,
    Elvis,
    Range,
    Until,

    // 标点符号
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    DoubleColon,
    Arrow,
    At,
    Hash,
    Dollar,

    // 空白和注释
    Whitespace,
    Newline,
    Comment,
    LineComment,
    BlockComment,

    // 特殊
    Eof,
    Error,
}

impl SyntaxKind for KotlinSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment | Self::LineComment | Self::BlockComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment | Self::LineComment | Self::BlockComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root | Self::SourceFile)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}
