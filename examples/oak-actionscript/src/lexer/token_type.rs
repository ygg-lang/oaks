use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type ActionScriptToken = Token<ActionScriptTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ActionScriptTokenType {
    Whitespace,
    Newline,
    Comment,
    Identifier,
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    BooleanLiteral,
    NullLiteral,
    As,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Delete,
    Do,
    Else,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Implements,
    Import,
    In,
    Instanceof,
    Interface,
    Internal,
    Is,
    Native,
    New,
    Null,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Use,
    Var,
    Void,
    While,
    With,
    Each,
    Get,
    Set,
    Namespace,
    Include,
    Dynamic,
    Final,
    Override,
    Array,
    Boolean,
    Date,
    Number,
    ObjectType,
    RegExp,
    StringType,
    Uint,
    Vector,
    VoidType,
    Xml,
    XmlList,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    EqualEqualEqual,
    NotEqual,
    NotEqualEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Increment,
    Decrement,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    Question,
    Colon,
    Dot,
    Arrow,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    At,
    Hash,
    Dollar,
    Ampersand,
    Backslash,
    Quote,
    DoubleQuote,
    Backtick,
    Eof,
    Program,
    Block,
    Variable,
    FunctionCall,
    MethodCall,
    PropertyAccess,
    ArrayAccess,
    ParameterList,
    UseItem,
    ModuleItem,
    StructItem,
    EnumItem,
    FunctionType,
    Root,
    Statement,
    Expression,
    Assignment,
    ConditionalExpression,
    BinaryExpression,
    UnaryExpression,
    IfStatement,
    ForStatement,
    WhileStatement,
    DoWhileStatement,
    SwitchStatement,
    TryStatement,
    ThrowStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    Error,
    LiteralExpression,
    IdentifierExpression,
    ParenthesizedExpression,
    SourceFile,
    BlockExpression,
    LetStatement,
    IfExpression,
    WhileExpression,
    LoopExpression,
    ForExpression,
    CallExpression,
    IndexExpression,
    FieldExpression,
}

impl TokenType for ActionScriptTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            t if t.is_keyword() => UniversalTokenRole::Keyword,
            t if t.is_operator() => UniversalTokenRole::Operator,
            t if t.is_punctuation() => UniversalTokenRole::Punctuation,
            Self::Identifier => UniversalTokenRole::Name,
            t if t.is_literal() => UniversalTokenRole::Literal,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ActionScriptTokenType {
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::StringLiteral | Self::CharLiteral | Self::NumberLiteral | Self::BooleanLiteral | Self::NullLiteral | Self::True | Self::False | Self::Null)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::As
                | Self::Break
                | Self::Case
                | Self::Catch
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Delete
                | Self::Do
                | Self::Else
                | Self::Extends
                | Self::Finally
                | Self::For
                | Self::Function
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::In
                | Self::Instanceof
                | Self::Interface
                | Self::Internal
                | Self::Is
                | Self::Native
                | Self::New
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Static
                | Self::Super
                | Self::Switch
                | Self::This
                | Self::Throw
                | Self::Try
                | Self::Typeof
                | Self::Use
                | Self::Var
                | Self::Void
                | Self::While
                | Self::With
                | Self::Each
                | Self::Get
                | Self::Set
                | Self::Namespace
                | Self::Include
                | Self::Dynamic
                | Self::Final
                | Self::Override
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Equal
                | Self::EqualEqual
                | Self::EqualEqualEqual
                | Self::NotEqual
                | Self::NotEqualEqual
                | Self::LessThan
                | Self::LessEqual
                | Self::GreaterThan
                | Self::GreaterEqual
                | Self::LogicalAnd
                | Self::LogicalOr
                | Self::LogicalNot
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::UnsignedRightShift
                | Self::PlusAssign
                | Self::MinusAssign
                | Self::StarAssign
                | Self::SlashAssign
                | Self::PercentAssign
                | Self::LeftShiftAssign
                | Self::RightShiftAssign
                | Self::UnsignedRightShiftAssign
                | Self::BitwiseAndAssign
                | Self::BitwiseOrAssign
                | Self::BitwiseXorAssign
                | Self::Question
        )
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Dot | Self::Comma | Self::Colon | Self::Semicolon)
    }
}
