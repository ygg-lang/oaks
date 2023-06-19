use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type LuaToken = Token<LuaSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LuaSyntaxKind {
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
    Comment,

    // 特殊标记
    Eof,
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
    FieldExpression,
    IndexExpression,
    TableExpression,
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

impl SyntaxKind for LuaSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        use LuaSyntaxKind::*;
        !matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | FunctionDeclaration | ParameterList | Parameter | BlockStatement |
            LocalStatement | AssignmentStatement | ExpressionStatement | IfStatement |
            WhileStatement | ForStatement | RepeatStatement | BreakStatement |
            ReturnStatement | GotoStatement | LabelStatement | IdentifierExpression |
            LiteralExpression | BooleanLiteral | NilLiteral | ParenthesizedExpression |
            BinaryExpression | UnaryExpression | CallExpression | FieldExpression |
            IndexExpression | TableExpression | FunctionExpression | VarargExpression |
            TableField | FieldList | ArgumentList | VariableList | ExpressionList |
            NameList | FunctionName | FunctionBody | ChunkStatement | StatementList
        )
    }

    fn is_element_type(&self) -> bool {
        use LuaSyntaxKind::*;
        matches!(
            self,
            Error | Eof |
            // 语法节点类型 (非终结符)
            SourceFile | FunctionDeclaration | ParameterList | Parameter | BlockStatement |
            LocalStatement | AssignmentStatement | ExpressionStatement | IfStatement |
            WhileStatement | ForStatement | RepeatStatement | BreakStatement |
            ReturnStatement | GotoStatement | LabelStatement | IdentifierExpression |
            LiteralExpression | BooleanLiteral | NilLiteral | ParenthesizedExpression |
            BinaryExpression | UnaryExpression | CallExpression | FieldExpression |
            IndexExpression | TableExpression | FunctionExpression | VarargExpression |
            TableField | FieldList | ArgumentList | VariableList | ExpressionList |
            NameList | FunctionName | FunctionBody | ChunkStatement | StatementList
        )
    }
}
