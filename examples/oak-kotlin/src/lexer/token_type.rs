use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type KotlinToken = Token<KotlinTokenType>;

impl TokenType for KotlinTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum KotlinTokenType {
    // 节点种类
    Root,
    SourceFile,
    EndOfStream,

    FunctionDeclaration,
    ClassDeclaration,
    VariableDeclaration,
    IfStatement,
    WhileStatement,
    ReturnStatement,
    Block,
    BinaryExpression,
    UnaryExpression,
    AssignmentExpression,
    CallExpression,
    MemberAccessExpression,
    LiteralExpression,
    IdentifierExpression,
    Parameter,
    TypeReference,

    // 关键字
    Class,
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
    Dot,
    Comma,
    Colon,
    Semi,
    Arrow,
    DoubleColon,
    Range,
    Question,
    ExclamationExclamation,
    At,

    // 标点符号
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // 其他
    Comment,
    Whitespace,
    Newline,
    Error,
}
