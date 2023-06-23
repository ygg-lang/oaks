use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GoElementType {
    // 非终端节点
    SourceFile,
    PackageClause,
    ImportDeclaration,
    ImportSpec,
    FunctionDeclaration,
    ParameterList,
    ParameterDecl,
    Block,
    VariableDeclaration,
    VariableSpec,
    ConstDeclaration,
    ConstSpec,
    TypeDeclaration,
    TypeSpec,
    StructType,
    FieldDeclList,
    FieldDecl,
    InterfaceType,
    MethodSpecList,
    MethodSpec,
    ExpressionList,
    AssignmentStatement,
    ShortVarDecl,
    ReturnStatement,
    IfStatement,
    ForStatement,
    SwitchStatement,
    ExprCaseClause,
    TypeSwitchStatement,
    TypeCaseClause,
    SelectStatement,
    CommClause,
    GoStatement,
    DeferStatement,
    CallExpression,
    IndexExpression,
    SelectorExpression,
    SliceExpression,
    TypeAssertion,
    UnaryExpression,
    BinaryExpression,
    LiteralValue,
    ElementList,
    KeyedElement,

    // 字面
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    RuneLiteral,
    BoolLiteral,

    // 标识
    Identifier,

    // 关键
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    For,
    Func,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,

    // 内置类型
    Bool,
    Byte,
    Complex64,
    Complex128,
    ErrorType,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Rune,
    String,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uintptr,

    // 特殊字面
    NilLiteral,
    NumberLiteral,
    CharLiteral,

    // 操作
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    LeftShift,      // <<
    RightShift,     // >>
    AmpersandCaret, // &^

    PlusAssign,           // +=
    MinusAssign,          // -=
    StarAssign,           // *=
    SlashAssign,          // /=
    PercentAssign,        // %=
    AmpersandAssign,      // &=
    PipeAssign,           // |=
    CaretAssign,          // ^=
    XorAssign,            // ^= (别名)
    LeftShiftAssign,      // <<=
    RightShiftAssign,     // >>=
    AmpersandCaretAssign, // &^=
    AndAssign,            // &=
    OrAssign,             // |=
    AndNotAssign,         // &^=
    AndNot,               // &^

    LogicalAnd, // &&
    LogicalOr,  // ||
    And,        // && (别名)
    Or,         // || (别名)
    Arrow,      // <-
    LeftArrow,  // <- (别名)
    Increment,  // ++
    Decrement,  // --

    Equal,      // ==
    Less,       // <
    Greater,    // >
    Assign,     // =
    LogicalNot, // !
    Not,        // ! (别名)

    NotEqual,     // !=
    LessEqual,    // <=
    GreaterEqual, // >=
    ColonAssign,  // :=
    Define,       // := (别名)
    Ellipsis,     // ...

    // 分隔
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Period,       // .
    Dot,          // . (别名)
    Semicolon,    // ;
    Colon,        // :

    // 空白和注
    Whitespace,
    Comment,

    // 特殊
    Eof,
    Error,
}

impl GoElementType {
    pub fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Break
                | Self::Case
                | Self::Chan
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Defer
                | Self::Else
                | Self::Fallthrough
                | Self::For
                | Self::Func
                | Self::Go
                | Self::Goto
                | Self::If
                | Self::Import
                | Self::Interface
                | Self::Map
                | Self::Package
                | Self::Range
                | Self::Return
                | Self::Select
                | Self::Struct
                | Self::Switch
                | Self::Type
                | Self::Var
        )
    }
}

impl ElementType for GoElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::GoTokenType> for GoElementType {
    fn from(token: crate::lexer::token_type::GoTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
