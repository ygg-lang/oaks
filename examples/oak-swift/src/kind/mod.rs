use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwiftSyntaxKind {
    // 基础 tokens
    Whitespace,
    Newline,
    Comment,
    Identifier,
    Error,
    Eof,

    // 字面量
    NumberLiteral,
    StringLiteral,
    CharLiteral,
    BooleanLiteral,

    // 关键字
    // 声明关键字
    Class,
    Struct,
    Enum,
    Protocol,
    Extension,
    Func,
    Var,
    Let,
    Init,
    Deinit,
    Subscript,
    Typealias,
    Import,

    // 控制流关键字
    If,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Repeat,
    Do,
    Break,
    Continue,
    Fallthrough,
    Return,
    Throw,
    Try,
    Catch,
    Finally,
    Guard,
    Defer,

    // 访问控制关键字
    Public,
    Private,
    Internal,
    Fileprivate,
    Open,

    // 修饰符关键字
    Static,
    Final,
    Override,
    Mutating,
    Nonmutating,
    Lazy,
    Weak,
    Unowned,
    Optional,
    Required,
    Convenience,
    Dynamic,
    Infix,
    Prefix,
    Postfix,

    // 类型关键字
    Any,
    AnyObject,
    Self_,
    Type,
    Protocol_,

    // 字面量关键字
    True,
    False,
    Nil,

    // 其他关键字
    As,
    Is,
    In,
    Where,
    Associatedtype,
    Operator,
    Precedencegroup,
    Indirect,
    Rethrows,
    Throws,
    Inout,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
    Question,
    QuestionQuestion,
    Dot,
    Arrow,
    Range,
    ClosedRange,

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
    At,
    Hash,
    Dollar,
    Underscore,
    Backslash,

    // 根节点
    SourceFile,
}

impl TokenType for SwiftSyntaxKind {
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
}

impl ElementType for SwiftSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
