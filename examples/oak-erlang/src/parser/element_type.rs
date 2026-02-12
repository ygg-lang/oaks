use oak_core::{ElementType, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Element types for the Erlang language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ErlangElementType {
    /// Whitespace characters.
    Whitespace,
    /// Newline characters.
    Newline,
    /// Comments.
    Comment,

    /// Identifiers.
    Identifier,
    /// Atoms.
    Atom,
    /// Variables.
    Variable,
    /// Numbers.
    Number,
    /// Strings.
    String,
    /// Characters.
    Character,

    /// `after` keyword.
    After,
    /// `and` keyword.
    And,
    /// `andalso` keyword.
    Andalso,
    /// `band` keyword.
    Band,
    /// `begin` keyword.
    Begin,
    /// `bnot` keyword.
    Bnot,
    /// `bor` keyword.
    Bor,
    /// `bsl` keyword.
    Bsl,
    /// `bsr` keyword.
    Bsr,
    /// `bxor` keyword.
    Bxor,
    /// `case` keyword.
    Case,
    /// `catch` keyword.
    Catch,
    /// `cond` keyword.
    Cond,
    /// `div` keyword.
    Div,
    /// `end` keyword.
    End,
    /// `fun` keyword.
    Fun,
    /// `if` keyword.
    If,
    /// `let` keyword.
    Let,
    /// `not` keyword.
    Not,
    /// `of` keyword.
    Of,
    /// `or` keyword.
    Or,
    /// `orelse` keyword.
    Orelse,
    /// `query` keyword.
    Query,
    /// `receive` keyword.
    Receive,
    /// `rem` keyword.
    Rem,
    /// `try` keyword.
    Try,
    /// `when` keyword.
    When,
    /// `xor` keyword.
    Xor,

    /// Plus operator (`+`).
    Plus,
    /// Minus operator (`-`).
    Minus,
    /// Star operator (`*`).
    Star,
    /// Slash operator (`/`).
    Slash,
    /// Equal operator (`=`).
    Equal,
    /// EqualEqual operator (`==`).
    EqualEqual,
    /// SlashEqual operator (`/=`).
    SlashEqual,
    /// EqualColonEqual operator (`=:=`).
    EqualColonEqual,
    /// EqualSlashEqual operator (`=/=`).
    EqualSlashEqual,
    /// Less operator (`<`).
    Less,
    /// Greater operator (`>`).
    Greater,
    /// LessEqual operator (`=<`).
    LessEqual,
    /// GreaterEqual operator (`>=`).
    GreaterEqual,
    /// PlusPlus operator (`++`).
    PlusPlus,
    /// MinusMinus operator (`--`).
    MinusMinus,
    /// Exclamation operator (`!`).
    Exclamation,
    /// Question operator (`?`).
    Question,

    /// Left parenthesis (`(`).
    LeftParen,
    /// Right parenthesis (`)`).
    RightParen,
    /// Left brace (`{`).
    LeftBrace,
    /// Right brace (`}`).
    RightBrace,
    /// Left bracket (`[`).
    LeftBracket,
    /// Right bracket (`]`).
    RightBracket,
    /// Comma (`,`).
    Comma,
    /// Semicolon (`;`).
    Semicolon,
    /// Dot (`.`).
    Dot,
    /// Colon (`:`).
    Colon,
    /// Arrow (`->`).
    Arrow,
    /// Pipe (`|`).
    Pipe,
    /// PipePipe (`||`).
    PipePipe,
    /// Hash (`#`).
    Hash,

    /// Root node of the Erlang AST.
    Root,
    /// An item in the module.
    Item,
    /// Module declaration.
    Module,
    /// Export attribute.
    Export,
    /// Generic attribute.
    Attribute,
    /// Function declaration.
    Function,
    /// A clause of a function.
    FunctionClause,
    /// A pattern matching expression.
    Pattern,
    /// A record pattern.
    RecordPattern,
    /// A statement.
    Statement,
    /// An expression.
    Expr,
    /// A binary expression.
    BinaryExpr,
    /// A function call.
    CallExpr,
    /// A fun expression.
    FunExpr,
    /// A case expression.
    CaseExpr,
    /// A clause in a case expression.
    CaseClause,
    /// An if expression.
    IfExpr,
    /// A clause in an if expression.
    IfClause,
    /// A try expression.
    TryExpr,
    /// A catch clause.
    CatchClause,
    /// A receive expression.
    ReceiveExpr,
    /// A clause in a receive expression.
    ReceiveClause,
    /// A record expression.
    RecordExpr,

    /// Error node.
    Error,
    /// End of file.
    Eof,
}

impl ErlangElementType {
    /// Returns `true` if the element type is a keyword.
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

    /// Returns `true` if the element type is an operator.
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

    /// Returns `true` if the element type is a punctuation.
    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon | Self::Dot | Self::Colon | Self::Arrow | Self::Pipe | Self::PipePipe | Self::Hash)
    }
}

impl ElementType for ErlangElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::ErlangTokenType> for ErlangElementType {
    fn from(token: crate::lexer::token_type::ErlangTokenType) -> Self {
        use crate::lexer::token_type::ErlangTokenType;
        match token {
            ErlangTokenType::Whitespace => Self::Whitespace,
            ErlangTokenType::Newline => Self::Newline,
            ErlangTokenType::Comment => Self::Comment,
            ErlangTokenType::Identifier => Self::Identifier,
            ErlangTokenType::Atom => Self::Atom,
            ErlangTokenType::Variable => Self::Variable,
            ErlangTokenType::Number => Self::Number,
            ErlangTokenType::String => Self::String,
            ErlangTokenType::Character => Self::Character,
            ErlangTokenType::After => Self::After,
            ErlangTokenType::And => Self::And,
            ErlangTokenType::Andalso => Self::Andalso,
            ErlangTokenType::Band => Self::Band,
            ErlangTokenType::Begin => Self::Begin,
            ErlangTokenType::Bnot => Self::Bnot,
            ErlangTokenType::Bor => Self::Bor,
            ErlangTokenType::Bsl => Self::Bsl,
            ErlangTokenType::Bsr => Self::Bsr,
            ErlangTokenType::Bxor => Self::Bxor,
            ErlangTokenType::Case => Self::Case,
            ErlangTokenType::Catch => Self::Catch,
            ErlangTokenType::Cond => Self::Cond,
            ErlangTokenType::Div => Self::Div,
            ErlangTokenType::End => Self::End,
            ErlangTokenType::Fun => Self::Fun,
            ErlangTokenType::If => Self::If,
            ErlangTokenType::Let => Self::Let,
            ErlangTokenType::Not => Self::Not,
            ErlangTokenType::Of => Self::Of,
            ErlangTokenType::Or => Self::Or,
            ErlangTokenType::Orelse => Self::Orelse,
            ErlangTokenType::Query => Self::Query,
            ErlangTokenType::Receive => Self::Receive,
            ErlangTokenType::Rem => Self::Rem,
            ErlangTokenType::Try => Self::Try,
            ErlangTokenType::When => Self::When,
            ErlangTokenType::Xor => Self::Xor,
            ErlangTokenType::Plus => Self::Plus,
            ErlangTokenType::Minus => Self::Minus,
            ErlangTokenType::Star => Self::Star,
            ErlangTokenType::Slash => Self::Slash,
            ErlangTokenType::Equal => Self::Equal,
            ErlangTokenType::EqualEqual => Self::EqualEqual,
            ErlangTokenType::SlashEqual => Self::SlashEqual,
            ErlangTokenType::EqualColonEqual => Self::EqualColonEqual,
            ErlangTokenType::EqualSlashEqual => Self::EqualSlashEqual,
            ErlangTokenType::Less => Self::Less,
            ErlangTokenType::Greater => Self::Greater,
            ErlangTokenType::LessEqual => Self::LessEqual,
            ErlangTokenType::GreaterEqual => Self::GreaterEqual,
            ErlangTokenType::PlusPlus => Self::PlusPlus,
            ErlangTokenType::MinusMinus => Self::MinusMinus,
            ErlangTokenType::Exclamation => Self::Exclamation,
            ErlangTokenType::Question => Self::Question,
            ErlangTokenType::LeftParen => Self::LeftParen,
            ErlangTokenType::RightParen => Self::RightParen,
            ErlangTokenType::LeftBrace => Self::LeftBrace,
            ErlangTokenType::RightBrace => Self::RightBrace,
            ErlangTokenType::LeftBracket => Self::LeftBracket,
            ErlangTokenType::RightBracket => Self::RightBracket,
            ErlangTokenType::Comma => Self::Comma,
            ErlangTokenType::Semicolon => Self::Semicolon,
            ErlangTokenType::Dot => Self::Dot,
            ErlangTokenType::Colon => Self::Colon,
            ErlangTokenType::Arrow => Self::Arrow,
            ErlangTokenType::Pipe => Self::Pipe,
            ErlangTokenType::PipePipe => Self::PipePipe,
            ErlangTokenType::Hash => Self::Hash,
            ErlangTokenType::Root => Self::Root,
            ErlangTokenType::Item => Self::Item,
            ErlangTokenType::Module => Self::Module,
            ErlangTokenType::Export => Self::Export,
            ErlangTokenType::Attribute => Self::Attribute,
            ErlangTokenType::Function => Self::Function,
            ErlangTokenType::FunctionClause => Self::FunctionClause,
            ErlangTokenType::Pattern => Self::Pattern,
            ErlangTokenType::RecordPattern => Self::RecordPattern,
            ErlangTokenType::Statement => Self::Statement,
            ErlangTokenType::Expr => Self::Expr,
            ErlangTokenType::BinaryExpr => Self::BinaryExpr,
            ErlangTokenType::CallExpr => Self::CallExpr,
            ErlangTokenType::FunExpr => Self::FunExpr,
            ErlangTokenType::CaseExpr => Self::CaseExpr,
            ErlangTokenType::CaseClause => Self::CaseClause,
            ErlangTokenType::IfExpr => Self::IfExpr,
            ErlangTokenType::IfClause => Self::IfClause,
            ErlangTokenType::TryExpr => Self::TryExpr,
            ErlangTokenType::CatchClause => Self::CatchClause,
            ErlangTokenType::ReceiveExpr => Self::ReceiveExpr,
            ErlangTokenType::ReceiveClause => Self::ReceiveClause,
            ErlangTokenType::RecordExpr => Self::RecordExpr,
            ErlangTokenType::Error => Self::Error,
        }
    }
}
