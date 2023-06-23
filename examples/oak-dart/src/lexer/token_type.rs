use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type DartToken = Token<DartTokenType>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DartTokenType {
    Root,

    ClassDeclaration,

    FunctionDeclaration,

    VariableDeclaration,

    Whitespace,

    Newline,

    Identifier,

    IntegerLiteral,

    DoubleLiteral,

    StringLiteral,

    BooleanLiteral,

    NullLiteral,

    Abstract,

    As,

    Assert,

    Async,

    Await,

    Break,

    Case,

    Catch,

    Class,

    Const,

    Continue,

    Covariant,

    Default,

    Deferred,

    Do,

    Dynamic,

    Else,

    Enum,

    Export,

    Extends,

    Extension,

    External,

    Factory,

    False,

    Final,

    Finally,

    For,

    Function,

    Get,

    Hide,

    If,

    Implements,

    Import,

    In,

    Interface,

    Int,

    Is,

    Late,

    Library,

    Mixin,

    New,

    Null,

    On,

    Operator,

    Part,

    Required,

    Rethrow,

    Return,

    Set,

    Show,

    Static,

    Super,

    Switch,

    Sync,

    This,

    Throw,

    True,

    Try,

    Typedef,

    Var,

    Void,

    While,

    With,

    Yield,

    Plus,

    Minus,

    Star,

    Slash,

    Percent,

    TildeSlash,

    Equal,

    EqualEqual,

    BangEqual,

    Less,

    Greater,

    LessEqual,

    GreaterEqual,

    LeftShift,

    RightShift,

    Ampersand,

    Pipe,

    Caret,

    Tilde,

    Bang,

    AmpersandAmpersand,

    PipePipe,

    Question,

    QuestionQuestion,

    PlusPlus,

    MinusMinus,

    PlusEqual,

    MinusEqual,

    StarEqual,

    SlashEqual,

    PercentEqual,

    TildeSlashEqual,

    LeftShiftEqual,

    RightShiftEqual,

    AmpersandEqual,

    PipeEqual,

    CaretEqual,

    QuestionQuestionEqual,

    Arrow,

    Dot,

    DotDot,

    DotDotDot,

    QuestionDot,

    LeftParen,

    RightParen,

    LeftBracket,

    RightBracket,

    LeftBrace,

    RightBrace,

    Semicolon,

    Comma,

    Colon,

    At,

    Hash,

    LineComment,

    BlockComment,

    DocComment,

    Error,

    Eof,
}

impl TokenType for DartTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::DoubleLiteral | Self::StringLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}

impl DartTokenType {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::As
                | Self::Assert
                | Self::Async
                | Self::Await
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Covariant
                | Self::Default
                | Self::Deferred
                | Self::Do
                | Self::Dynamic
                | Self::Else
                | Self::Enum
                | Self::Export
                | Self::Extends
                | Self::Extension
                | Self::External
                | Self::Factory
                | Self::False
                | Self::Final
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::Get
                | Self::Hide
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Interface
                | Self::Int
                | Self::Is
                | Self::Late
                | Self::Library
                | Self::Mixin
                | Self::New
                | Self::Null
                | Self::On
                | Self::Operator
                | Self::Part
                | Self::Required
                | Self::Rethrow
                | Self::Return
                | Self::Set
                | Self::Show
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::Sync
                | Self::This
                | Self::Throw
                | Self::True
                | Self::Try
                | Self::Typedef
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Yield
        )
    }
}

pub type DartTokenKind = DartTokenType;
