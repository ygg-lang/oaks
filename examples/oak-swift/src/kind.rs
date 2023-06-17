use oak_core::SyntaxKind;

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
}

impl SyntaxKind for SwiftSyntaxKind {
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
