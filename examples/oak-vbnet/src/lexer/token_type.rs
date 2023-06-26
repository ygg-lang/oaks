use oak_core::{Token, TokenType, UniversalTokenRole};

/// A VB.NET token.
pub type VbNetToken = Token<VbNetTokenType>;

/// VB.NET token types
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VbNetTokenType {
    /// Root node
    Root,
    /// Expression
    Expression,
    /// Whitespace
    Whitespace,
    /// Newline
    Newline,

    /// Identifier
    Identifier,
    /// Integer literal
    IntegerLiteral,
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Character literal
    CharLiteral,
    /// Boolean literal
    BooleanLiteral,
    /// Date literal
    DateLiteral,
    /// Nothing literal
    NothingLiteral,

    /// The 'Namespace' keyword
    Namespace,
    /// The 'Imports' keyword
    Imports,
    /// The 'Class' keyword
    Class,
    /// The 'Interface' keyword
    Interface,
    /// The 'Structure' keyword
    Structure,
    /// The 'Enum' keyword
    Enum,
    /// The 'Module' keyword
    Module,
    /// The 'Delegate' keyword
    Delegate,
    /// The 'Event' keyword
    Event,
    /// The 'Function' keyword
    Function,
    /// The 'Sub' keyword
    Sub,
    /// The 'Property' keyword
    Property,
    /// The 'Dim' keyword
    Dim,
    /// The 'Const' keyword
    Const,
    /// The 'As' keyword
    As,
    /// The 'In' keyword
    In,
    /// The 'If' keyword
    If,
    /// The 'Then' keyword
    Then,
    /// The 'Else' keyword
    Else,
    /// The 'ElseIf' keyword
    ElseIf,
    /// The 'End' keyword
    End,
    /// The 'For' keyword
    For,
    /// The 'Each' keyword
    Each,
    /// The 'To' keyword
    To,
    /// The 'Step' keyword
    Step,
    /// The 'While' keyword
    While,
    /// The 'Do' keyword
    Do,
    /// The 'Loop' keyword
    Loop,
    /// The 'Until' keyword
    Until,
    /// The 'Select' keyword
    Select,
    /// The 'Case' keyword
    Case,
    /// The 'Default' keyword
    Default,
    /// The 'With' keyword
    With,
    /// The 'Try' keyword
    Try,
    /// The 'Catch' keyword
    Catch,
    /// The 'Finally' keyword
    Finally,
    /// The 'Throw' keyword
    Throw,
    /// The 'Exit' keyword
    Exit,
    /// The 'Continue' keyword
    Continue,
    /// The 'Next' keyword
    Next,
    /// The 'Return' keyword
    Return,
    /// The 'Me' keyword
    Me,
    /// The 'MyBase' keyword
    MyBase,
    /// The 'MyClass' keyword
    MyClass,
    /// The 'New' keyword
    New,
    /// The 'Of' keyword
    Of,
    /// The 'ByVal' keyword
    ByVal,
    /// The 'ByRef' keyword
    ByRef,
    /// The 'Optional' keyword
    Optional,
    /// The 'ParamArray' keyword
    ParamArray,
    /// The 'Public' keyword
    Public,
    /// The 'Private' keyword
    Private,
    /// The 'Protected' keyword
    Protected,
    /// The 'Friend' keyword
    Friend,
    /// The 'ProtectedFriend' keyword
    ProtectedFriend,
    /// The 'Shared' keyword
    Shared,
    /// The 'MustInherit' keyword
    MustInherit,
    /// The 'NotInheritable' keyword
    NotInheritable,
    /// The 'MustOverride' keyword
    MustOverride,
    /// The 'Overridable' keyword
    Overridable,
    /// The 'Overrides' keyword
    Overrides,
    /// The 'NotOverridable' keyword
    NotOverridable,
    /// The 'MustOverrideReadOnly' keyword
    MustOverrideReadOnly,
    /// The 'ReadOnly' keyword
    ReadOnly,
    /// The 'WriteOnly' keyword
    WriteOnly,
    /// The 'Static' keyword
    Static,
    /// The 'Partial' keyword
    Partial,
    /// The 'Async' keyword
    Async,
    /// The 'Await' keyword
    Await,
    /// The 'From' keyword for LINQ
    From,
    /// The 'Where' keyword for LINQ
    Where,
    /// The 'Order' keyword for LINQ
    Order,
    /// The 'By' keyword for LINQ
    By,
    /// The 'Group' keyword for LINQ
    Group,
    /// The 'Join' keyword for LINQ
    Join,
    /// The 'On' keyword for LINQ
    On,
    /// The 'Into' keyword for LINQ
    Into,
    /// The 'Let' keyword for LINQ
    Let,
    /// The 'Overloads' keyword
    Overloads,
    /// The 'Inherits' keyword
    Inherits,
    /// The 'Implements' keyword
    Implements,
    /// The 'Get' keyword
    Get,
    /// The 'Set' keyword
    Set,
    /// The 'Equals' keyword
    Equals,
    /// The 'Statement' keyword
    Statement,

    /// The '+' operator
    Plus,
    /// The '-' operator
    Minus,
    /// The '*' operator
    Star,
    /// The '/' operator
    Slash,
    /// The '\' operator
    Backslash,
    /// The '%' operator
    Percent,
    /// The '^' operator
    Caret,

    /// The '=' operator
    Equal,
    /// The '<>' operator
    NotEqual,
    /// The '<' operator
    LessThan,
    /// The '<=' operator
    LessEqual,
    /// The '>' operator
    GreaterThan,
    /// The '>=' operator
    GreaterEqual,

    /// The 'And' operator
    And,
    /// The 'Or' operator
    Or,
    /// The 'Not' operator
    Not,
    /// The 'Xor' operator
    Xor,
    /// The 'AndAlso' operator
    AndAlso,
    /// The 'OrElse' operator
    OrElse,
    /// The 'Is' operator
    Is,
    /// The 'IsNot' operator
    IsNot,
    /// The 'Like' operator
    Like,
    /// The 'TypeOf' operator
    TypeOf,

    /// The '(' delimiter
    LeftParen,
    /// The ')' delimiter
    RightParen,
    /// The '[' delimiter
    LeftBracket,
    /// The ']' delimiter
    RightBracket,
    /// The '{' delimiter
    LeftBrace,
    /// The '}' delimiter
    RightBrace,

    /// The ',' punctuation
    Comma,
    /// The ';' punctuation
    Semicolon,
    /// The ':' punctuation
    Colon,
    /// The '.' punctuation
    Dot,
    /// The '!' punctuation
    Exclamation,
    /// The '#' punctuation
    Hash,
    /// The '&' punctuation
    Ampersand,

    /// Line comment
    LineComment,
    /// Block comment
    BlockComment,

    /// Error
    Error,
    /// End of file
    Eof,
}

impl VbNetTokenType {
    /// Checks if it is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Namespace
                | Self::Imports
                | Self::Class
                | Self::Interface
                | Self::Structure
                | Self::Enum
                | Self::Module
                | Self::Delegate
                | Self::Event
                | Self::Function
                | Self::Sub
                | Self::Property
                | Self::Dim
                | Self::Const
                | Self::As
                | Self::In
                | Self::If
                | Self::Then
                | Self::Else
                | Self::ElseIf
                | Self::End
                | Self::For
                | Self::Each
                | Self::To
                | Self::Step
                | Self::While
                | Self::Do
                | Self::Loop
                | Self::Until
                | Self::Select
                | Self::Case
                | Self::Default
                | Self::With
                | Self::Try
                | Self::Catch
                | Self::Finally
                | Self::Throw
                | Self::Exit
                | Self::Continue
                | Self::Return
                | Self::Me
                | Self::MyBase
                | Self::MyClass
                | Self::New
                | Self::Of
                | Self::ByVal
                | Self::ByRef
                | Self::Optional
                | Self::ParamArray
                | Self::Public
                | Self::Private
                | Self::Protected
                | Self::Friend
                | Self::ProtectedFriend
                | Self::Shared
                | Self::MustInherit
                | Self::NotInheritable
                | Self::MustOverride
                | Self::Overridable
                | Self::Overrides
                | Self::NotOverridable
                | Self::MustOverrideReadOnly
                | Self::ReadOnly
                | Self::WriteOnly
                | Self::Static
                | Self::Partial
                | Self::Async
                | Self::Await
                | Self::From
                | Self::Where
                | Self::Order
                | Self::By
                | Self::Group
                | Self::Join
                | Self::On
                | Self::Into
                | Self::Let
                | Self::And
                | Self::Or
                | Self::Not
                | Self::Xor
                | Self::AndAlso
                | Self::OrElse
                | Self::Is
                | Self::IsNot
                | Self::Like
                | Self::TypeOf
                | Self::Overloads
                | Self::Inherits
                | Self::Implements
                | Self::Get
                | Self::Set
                | Self::Equals
                | Self::Statement
        )
    }
}

impl TokenType for VbNetTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalTokenRole::Error,
            _ if self.is_keyword() => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::DateLiteral | Self::NothingLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}
