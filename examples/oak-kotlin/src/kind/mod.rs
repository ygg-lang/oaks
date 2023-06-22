use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};

/// Kotlin 语法种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum KotlinSyntaxKind {
    // 节点种类
    Root,
    SourceFile,
    EndOfStream,

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

impl TokenType for KotlinSyntaxKind {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::EndOfStream;

    fn role(&self) -> Self::Role {
        match self {
            Self::Class
            | Self::Fun
            | Self::Val
            | Self::Var
            | Self::If
            | Self::Else
            | Self::When
            | Self::For
            | Self::While
            | Self::Return
            | Self::Break
            | Self::Continue
            | Self::Try
            | Self::Catch
            | Self::Finally
            | Self::Throw
            | Self::Import
            | Self::Package
            | Self::Public
            | Self::Private
            | Self::Protected
            | Self::Internal
            | Self::Abstract
            | Self::Final
            | Self::Open
            | Self::Override
            | Self::Companion
            | Self::Object
            | Self::Interface
            | Self::Enum
            | Self::Data
            | Self::Sealed
            | Self::Inline
            | Self::Suspend
            | Self::Operator
            | Self::Infix
            | Self::Tailrec
            | Self::External
            | Self::Annotation
            | Self::Crossinline
            | Self::Noinline
            | Self::Reified
            | Self::Vararg
            | Self::Out
            | Self::In
            | Self::Is
            | Self::As
            | Self::This
            | Self::Super
            | Self::Null
            | Self::True
            | Self::False
            | Self::Keyword => UniversalTokenRole::Keyword,

            Self::Identifier => UniversalTokenRole::Name,

            Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::IntLiteral | Self::FloatLiteral | Self::BooleanLiteral => UniversalTokenRole::Literal,

            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::Equals
            | Self::Less
            | Self::Greater
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::Exclamation
            | Self::Assign
            | Self::PlusAssign
            | Self::MinusAssign
            | Self::StarAssign
            | Self::SlashAssign
            | Self::PercentAssign
            | Self::EqEq
            | Self::NotEq
            | Self::Lt
            | Self::Gt
            | Self::LtEq
            | Self::GtEq
            | Self::AndAnd
            | Self::OrOr
            | Self::Dot
            | Self::Comma
            | Self::Colon
            | Self::Semi
            | Self::At
            | Self::Arrow
            | Self::DoubleColon
            | Self::Range
            | Self::Question
            | Self::ExclamationExclamation => UniversalTokenRole::Operator,

            Self::LParen | Self::RParen | Self::LBracket | Self::RBracket | Self::LBrace | Self::RBrace => UniversalTokenRole::Punctuation,

            Self::Comment => UniversalTokenRole::Comment,
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for KotlinSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root | Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
