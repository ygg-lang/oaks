use oak_core::{SyntaxKind, Token};
use serde::{Deserialize, Serialize};

pub type ErlangToken = Token<ErlangSyntaxKind>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErlangSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,

    // 标识符和字面量
    Identifier,
    Atom,
    Variable,
    Number,
    String,
    Character,

    // Erlang 关键字
    After,
    And,
    Andalso,
    Band,
    Begin,
    Bnot,
    Bor,
    Bsl,
    Bsr,
    Bxor,
    Case,
    Catch,
    Cond,
    Div,
    End,
    Fun,
    If,
    Let,
    Not,
    Of,
    Or,
    Orelse,
    Query,
    Receive,
    Rem,
    Try,
    When,
    Xor,

    // 操作符
    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Equal,           // =
    EqualEqual,      // ==
    SlashEqual,      // /=
    EqualColonEqual, // =:=
    EqualSlashEqual, // =/=
    Less,            // <
    Greater,         // >
    LessEqual,       // =<
    GreaterEqual,    // >=
    PlusPlus,        // ++
    MinusMinus,      // --
    Exclamation,     // !
    Question,        // ?

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Dot,          // .
    Colon,        // :
    Arrow,        // ->
    Pipe,         // |
    PipePipe,     // ||
    Hash,         // #

    // 语法节点类型
    Root,
    Item,
    Module,
    Export,
    Attribute,
    Function,
    FunctionClause,
    Pattern,
    RecordPattern,
    Statement,
    Expr,
    BinaryExpr,
    CallExpr,
    FunExpr,
    CaseExpr,
    CaseClause,
    IfExpr,
    IfClause,
    TryExpr,
    CatchClause,
    ReceiveExpr,
    ReceiveClause,
    RecordExpr,

    // 特殊
    Error,
    Eof,
}

impl SyntaxKind for ErlangSyntaxKind {
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
        !self.is_element_type()
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Item
                | Self::Module
                | Self::Export
                | Self::Attribute
                | Self::Function
                | Self::FunctionClause
                | Self::Pattern
                | Self::RecordPattern
                | Self::Statement
                | Self::Expr
                | Self::BinaryExpr
                | Self::CallExpr
                | Self::FunExpr
                | Self::CaseExpr
                | Self::CaseClause
                | Self::IfExpr
                | Self::IfClause
                | Self::TryExpr
                | Self::CatchClause
                | Self::ReceiveExpr
                | Self::ReceiveClause
                | Self::RecordExpr
        )
    }
}
