use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ErlangToken = Token<ErlangTokenType>;

impl ErlangTokenType {
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

impl TokenType for ErlangTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier | Self::Atom | Self::Variable => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Character => UniversalTokenRole::Literal,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            _ if self.is_operator() => UniversalTokenRole::Operator,
            _ if self.is_punctuation() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ErlangTokenType {
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
}
