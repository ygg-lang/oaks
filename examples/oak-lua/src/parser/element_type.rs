use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum LuaElementType {
    Root,
    // 关键字
    And,
    Break,
    Do,
    Else,
    Elseif,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Until,
    While,

    // 标识符和字面量
    Identifier,
    Number,
    String,

    // 操作符
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Hash,       // #
    Ampersand,  // &
    Tilde,      // ~
    Pipe,       // |
    LtLt,       // <<
    GtGt,       // >>
    SlashSlash, // //
    EqEq,       // ==
    TildeEq,    // ~=
    LtEq,       // <=
    GtEq,       // >=
    Lt,         // <
    Gt,         // >
    Eq,         // =

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    ColonColon,   // ::
    Semicolon,    // ;
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    DotDot,       // ..
    DotDotDot,    // ...

    // 空白和注释
    Whitespace,
    Newline,
    Comment,

    // 特殊标记
    EndOfStream,
    Error,

    // 语法节点类型 (非终结符)
    SourceFile,
    FunctionDeclaration,
    ParameterList,
    Parameter,
    BlockStatement,
    LocalStatement,
    AssignmentStatement,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    RepeatStatement,
    DoStatement,
    BreakStatement,
    ReturnStatement,
    GotoStatement,
    LabelStatement,
    IdentifierExpression,
    LiteralExpression,
    BooleanLiteral,
    NilLiteral,
    ParenthesizedExpression,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    MemberExpression,
    IndexExpression,
    TableConstructorExpression,
    FunctionExpression,
    VarargExpression,
    TableField,
    FieldList,
    ArgumentList,
    VariableList,
    ExpressionList,
    NameList,
    FunctionName,
    FunctionBody,
    ChunkStatement,
    StatementList,
}

impl ElementType for LuaElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::LuaTokenType> for LuaElementType {
    fn from(token: crate::lexer::token_type::LuaTokenType) -> Self {
        unsafe { std::mem::transmute(token) }
    }
}
