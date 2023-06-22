use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type DartToken = Token<DartSyntaxKind>;

/// Represents all possible syntax kinds in the Dart programming language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DartSyntaxKind {
    /// Root node of the syntax tree
    Root,
    /// Class declaration
    ClassDeclaration,
    /// Function declaration
    FunctionDeclaration,
    /// Whitespace characters (spaces, tabs)
    Whitespace,
    /// Newline character
    Newline,

    /// Identifier (variable names, function names, etc.)
    Identifier,
    /// Integer literal
    IntegerLiteral,
    /// Double (floating-point) literal
    DoubleLiteral,
    /// String literal
    StringLiteral,
    /// Boolean literal (true/false)
    BooleanLiteral,
    /// Null literal
    NullLiteral,

    /// abstract keyword
    Abstract,
    /// as keyword
    As,
    /// assert keyword
    Assert,
    /// async keyword
    Async,
    /// await keyword
    Await,
    /// break keyword
    Break,
    /// case keyword
    Case,
    /// catch keyword
    Catch,
    /// class keyword
    Class,
    /// const keyword
    Const,
    /// continue keyword
    Continue,
    /// covariant keyword
    Covariant,
    /// default keyword
    Default,
    /// deferred keyword
    Deferred,
    /// do keyword
    Do,
    /// dynamic keyword
    Dynamic,
    /// else keyword
    Else,
    /// enum keyword
    Enum,
    /// export keyword
    Export,
    /// extends keyword
    Extends,
    /// extension keyword
    Extension,
    /// external keyword
    External,
    /// factory keyword
    Factory,
    /// false keyword
    False,
    /// final keyword
    Final,
    /// finally keyword
    Finally,
    /// for keyword
    For,
    /// function keyword
    Function,
    /// get keyword
    Get,
    /// hide keyword
    Hide,
    /// if keyword
    If,
    /// implements keyword
    Implements,
    /// import keyword
    Import,
    /// in keyword
    In,
    /// interface keyword
    Interface,
    /// is keyword
    Is,
    /// late keyword
    Late,
    /// library keyword
    Library,
    /// mixin keyword
    Mixin,
    /// new keyword
    New,
    /// null keyword
    Null,
    /// on keyword
    On,
    /// operator keyword
    Operator,
    /// part keyword
    Part,
    /// required keyword
    Required,
    /// rethrow keyword
    Rethrow,
    /// return keyword
    Return,
    /// set keyword
    Set,
    /// show keyword
    Show,
    /// static keyword
    Static,
    /// super keyword
    Super,
    /// switch keyword
    Switch,
    /// sync keyword
    Sync,
    /// this keyword
    This,
    /// throw keyword
    Throw,
    /// true keyword
    True,
    /// try keyword
    Try,
    /// typedef keyword
    Typedef,
    /// var keyword
    Var,
    /// void keyword
    Void,
    /// while keyword
    While,
    /// with keyword
    With,
    /// yield keyword
    Yield,

    /// plus operator (+)
    Plus,
    /// minus operator (-)
    Minus,
    /// multiplication operator (*)
    Star,
    /// division operator (/)
    Slash,
    /// modulo operator (%)
    Percent,
    /// integer division operator (~/)
    TildeSlash,
    /// assignment operator (=)
    Equal,
    /// equality operator (==)
    EqualEqual,
    /// inequality operator (!=)
    BangEqual,
    /// less than operator (<)
    Less,
    /// greater than operator (>)
    Greater,
    /// less than or equal operator (<=)
    LessEqual,
    /// greater than or equal operator (>=)
    GreaterEqual,
    /// left shift operator (<<)
    LeftShift,
    /// right shift operator (>>)
    RightShift,
    /// bitwise AND operator (&)
    Ampersand,
    /// bitwise OR operator (|)
    Pipe,
    /// bitwise XOR operator (^)
    Caret,
    /// bitwise NOT operator (~)
    Tilde,
    /// logical NOT operator (!)
    Bang,
    /// logical AND operator (&&)
    AmpersandAmpersand,
    /// logical OR operator (||)
    PipePipe,
    /// ternary operator (?)
    Question,
    /// null-aware operator (??)
    QuestionQuestion,
    /// increment operator (++)
    PlusPlus,
    /// decrement operator (--)
    MinusMinus,
    /// plus assignment operator (+=)
    PlusEqual,
    /// minus assignment operator (-=)
    MinusEqual,
    /// multiplication assignment operator (*=)
    StarEqual,
    /// division assignment operator (/=)
    SlashEqual,
    /// modulo assignment operator (%=)
    PercentEqual,
    /// integer division assignment operator (~/=)
    TildeSlashEqual,
    /// left shift assignment operator (<<=)
    LeftShiftEqual,
    /// right shift assignment operator (>>=)
    RightShiftEqual,
    /// bitwise AND assignment operator (&=)
    AmpersandEqual,
    /// bitwise OR assignment operator (|=)
    PipeEqual,
    /// bitwise XOR assignment operator (^=)
    CaretEqual,
    /// null-aware assignment operator (??=)
    QuestionQuestionEqual,
    /// arrow operator (=>)
    Arrow,
    /// dot operator (.)
    Dot,
    /// cascade operator (..)
    DotDot,
    /// spread operator (...)
    DotDotDot,
    /// null-aware dot operator (?.)
    QuestionDot,

    /// left parenthesis (
    LeftParen,
    /// right parenthesis )
    RightParen,
    /// left bracket [
    LeftBracket,
    /// right bracket ]
    RightBracket,
    /// left brace {
    LeftBrace,
    /// right brace }
    RightBrace,
    /// semicolon ;
    Semicolon,
    /// comma ,
    Comma,
    /// colon :
    Colon,
    /// at symbol @
    At,
    /// hash symbol #
    Hash,

    /// line comment (//)
    LineComment,
    /// block comment (/* */)
    BlockComment,
    /// documentation comment (///)
    DocComment,

    /// error token
    Error,

    /// end of file
    Eof,
    /// Variable declaration
    VariableDeclaration,
}

impl TokenType for DartSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::DocComment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Identifier => UniversalTokenRole::Name,
            Self::IntegerLiteral | Self::DoubleLiteral | Self::StringLiteral | Self::BooleanLiteral | Self::NullLiteral => UniversalTokenRole::Literal,
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
            | Self::Yield => UniversalTokenRole::Keyword,
            Self::Plus
            | Self::Minus
            | Self::Star
            | Self::Slash
            | Self::Percent
            | Self::TildeSlash
            | Self::Equal
            | Self::EqualEqual
            | Self::BangEqual
            | Self::Less
            | Self::Greater
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::LeftShift
            | Self::RightShift
            | Self::Ampersand
            | Self::Pipe
            | Self::Caret
            | Self::Tilde
            | Self::Bang
            | Self::AmpersandAmpersand
            | Self::PipePipe
            | Self::Question
            | Self::QuestionQuestion
            | Self::PlusPlus
            | Self::MinusMinus
            | Self::PlusEqual
            | Self::MinusEqual
            | Self::StarEqual
            | Self::SlashEqual
            | Self::PercentEqual
            | Self::TildeSlashEqual
            | Self::LeftShiftEqual
            | Self::RightShiftEqual
            | Self::AmpersandEqual
            | Self::PipeEqual
            | Self::CaretEqual
            | Self::QuestionQuestionEqual
            | Self::Arrow
            | Self::Dot
            | Self::DotDot
            | Self::DotDotDot
            | Self::QuestionDot => UniversalTokenRole::Operator,
            Self::LeftParen | Self::RightParen | Self::LeftBracket | Self::RightBracket | Self::LeftBrace | Self::RightBrace | Self::Semicolon | Self::Comma | Self::Colon | Self::At | Self::Hash => UniversalTokenRole::Punctuation,
            Self::LineComment | Self::BlockComment | Self::DocComment => UniversalTokenRole::Comment,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof | Self::Root | Self::ClassDeclaration | Self::FunctionDeclaration | Self::VariableDeclaration => UniversalTokenRole::None,
        }
    }
}

impl ElementType for DartSyntaxKind {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::ClassDeclaration => UniversalElementRole::Definition,
            Self::FunctionDeclaration => UniversalElementRole::Definition,
            Self::VariableDeclaration => UniversalElementRole::Definition,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
