use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
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

impl ErlangSyntaxKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Andalso
                | Self::Band
                | Self::Begin
                | Self::Bnot
                | Self::Bor
                | Self::Bsl
                | Self::Bsr
                | Self::Bxor
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Div
                | Self::End
                | Self::Fun
                | Self::If
                | Self::Let
                | Self::Not
                | Self::Of
                | Self::Or
                | Self::Orelse
                | Self::Query
                | Self::Receive
                | Self::Rem
                | Self::Try
                | Self::When
                | Self::Xor
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Equal
                | Self::EqualEqual
                | Self::SlashEqual
                | Self::EqualColonEqual
                | Self::EqualSlashEqual
                | Self::Less
                | Self::Greater
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::PlusPlus
                | Self::MinusMinus
                | Self::Exclamation
                | Self::Question
        )
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon | Self::Dot | Self::Colon | Self::Arrow | Self::Pipe | Self::PipePipe | Self::Hash)
    }
}

impl TokenType for ErlangSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier | Self::Atom | Self::Variable => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Character => UniversalTokenRole::Literal,
            kind if kind.is_keyword() => UniversalTokenRole::Keyword,
            kind if kind.is_operator() => UniversalTokenRole::Operator,
            kind if kind.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::Error,
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

impl ElementType for ErlangSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Module | Self::Function | Self::FunctionClause => UniversalElementRole::Definition,
            Self::Statement => UniversalElementRole::Statement,
            Self::Expr | Self::BinaryExpr | Self::CallExpr | Self::FunExpr | Self::CaseExpr | Self::IfExpr | Self::TryExpr | Self::ReceiveExpr | Self::RecordExpr => UniversalElementRole::Expression,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }
}
