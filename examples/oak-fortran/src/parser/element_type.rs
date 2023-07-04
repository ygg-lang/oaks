use oak_core::{ElementType, UniversalElementRole};

/// Element types for the Fortran parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum FortranElementType {
    /// Whitespace characters (spaces and tabs).
    Whitespace,
    /// Newline character.
    Newline,
    /// Fortran comment.
    Comment,

    /// Identifier (variable, function, or procedure name).
    Identifier,
    /// Integer literal value.
    IntegerLiteral,
    /// Numeric value.
    Number,
    /// Numeric literal value.
    NumberLiteral,
    /// Real number literal value.
    RealLiteral,
    /// Double precision literal value.
    DoublePrecisionLiteral,
    /// Complex number literal value.
    ComplexLiteral,
    /// Character literal value.
    CharacterLiteral,
    /// Character literal value (alias).
    CharLiteral,
    /// String value.
    String,
    /// String literal value.
    StringLiteral,
    /// Logical literal value.
    LogicalLiteral,
    /// True boolean literal.
    True,
    /// False boolean literal.
    False,

    /// PROGRAM keyword.
    Program,
    /// END PROGRAM keyword.
    EndProgram,
    /// SUBROUTINE keyword.
    Subroutine,
    /// END SUBROUTINE keyword.
    EndSubroutine,
    /// FUNCTION keyword.
    Function,
    /// END FUNCTION keyword.
    EndFunction,
    /// MODULE keyword.
    Module,
    /// END MODULE keyword.
    EndModule,
    /// INTERFACE keyword.
    Interface,
    /// END INTERFACE keyword.
    EndInterface,
    /// TYPE keyword.
    Type,
    /// END TYPE keyword.
    EndType,
    /// IF keyword.
    If,
    /// THEN keyword.
    Then,
    /// ELSE IF keyword.
    ElseIf,
    /// ELSE keyword.
    Else,
    /// END IF keyword.
    EndIf,
    /// DO keyword.
    Do,
    /// END DO keyword.
    EndDo,
    /// WHILE keyword.
    While,
    /// SELECT keyword.
    Select,
    /// CASE keyword.
    Case,
    /// END SELECT keyword.
    EndSelect,
    /// WHERE keyword.
    Where,
    /// END WHERE keyword.
    EndWhere,
    /// FORALL keyword.
    Forall,
    /// END FORALL keyword.
    EndForall,
    /// ASSOCIATE keyword.
    Associate,
    /// END ASSOCIATE keyword.
    EndAssociate,
    /// BLOCK keyword.
    Block,
    /// END BLOCK keyword.
    EndBlock,
    /// CRITICAL keyword.
    Critical,
    /// END CRITICAL keyword.
    EndCritical,
    /// PROCEDURE keyword.
    Procedure,
    /// END PROCEDURE keyword.
    EndProcedure,
    /// ABSTRACT keyword.
    Abstract,
    /// ALLOCATABLE keyword.
    Allocatable,
    /// ALLOCATE keyword.
    Allocate,
    /// DEALLOCATE keyword.
    Deallocate,
    /// ASSIGNMENT keyword.
    Assignment,
    /// BIND keyword.
    Bind,
    /// CALL keyword.
    Call,
    /// CLASS keyword.
    Class,
    /// COMMON keyword.
    Common,
    /// CONTAINS keyword.
    Contains,
    /// CONTINUE keyword.
    Continue,
    /// CYCLE keyword.
    Cycle,
    /// DATA keyword.
    Data,
    /// DEFAULT keyword.
    Default,
    /// DIMENSION keyword.
    Dimension,
    /// ELEMENTAL keyword.
    Elemental,
    /// ENTRY keyword.
    Entry,
    /// EQUIVALENCE keyword.
    Equivalence,
    /// EXIT keyword.
    Exit,
    /// EXTERNAL keyword.
    External,
    /// FINAL keyword.
    Final,
    /// FORMAT keyword.
    Format,
    /// GENERIC keyword.
    Generic,
    /// GO keyword.
    Go,
    /// GOTO keyword.
    Goto,
    /// IMPLICIT keyword.
    Implicit,
    /// IMPORT keyword.
    Import,
    /// INCLUDE keyword.
    Include,
    /// INTENT keyword.
    Intent,
    /// INTRINSIC keyword.
    Intrinsic,
    /// KIND keyword.
    Kind,
    /// LEN keyword.
    Len,
    /// NONE keyword.
    None,
    /// NULLIFY keyword.
    Nullify,
    /// ONLY keyword.
    Only,
    /// OPTIONAL keyword.
    Optional,
    /// PARAMETER keyword.
    Parameter,
    /// PAUSE keyword.
    Pause,
    /// POINTER keyword.
    Pointer,
    /// PRINT keyword.
    Print,
    /// PRIVATE keyword.
    Private,
    /// PROTECTED keyword.
    Protected,
    /// PUBLIC keyword.
    Public,
    /// PURE keyword.
    Pure,
    /// READ keyword.
    Read,
    /// RECURSIVE keyword.
    Recursive,
    /// RESULT keyword.
    Result,
    /// RETURN keyword.
    Return,
    /// REWIND keyword.
    Rewind,
    /// SAVE keyword.
    Save,
    /// STOP keyword.
    Stop,
    /// TARGET keyword.
    Target,
    /// USE keyword.
    Use,
    /// VALUE keyword.
    Value,
    /// VOLATILE keyword.
    Volatile,
    /// WAIT keyword.
    Wait,
    /// WRITE keyword.
    Write,
    /// INQUIRE keyword.
    Inquire,
    /// BACKSPACE keyword.
    Backspace,
    /// CLOSE keyword.
    Close,
    /// OPEN keyword.
    Open,
    /// TO keyword.
    To,
    /// END keyword.
    End,
    /// DOUBLE keyword.
    Double,
    /// PRECISION keyword.
    Precision,

    /// INTEGER type keyword.
    Integer,
    /// REAL type keyword.
    Real,
    /// DOUBLE PRECISION type keyword.
    DoublePrecision,
    /// COMPLEX type keyword.
    Complex,
    /// CHARACTER type keyword.
    Character,
    /// LOGICAL type keyword.
    Logical,

    /// Addition operator (+).
    Plus,
    /// Subtraction operator (-).
    Minus,
    /// Multiplication operator (*).
    Star,
    /// Division operator (/).
    Slash,
    /// Exponentiation operator (**).
    StarStar,
    /// Exponentiation operator (alias for StarStar).
    Power,
    /// Concatenation operator (//).
    Concatenate,
    /// Equality operator (==).
    Equal,
    /// Equality operator (alias for Equal).
    EqualEqual,
    /// Inequality operator (/=).
    NotEqual,
    /// Inequality operator (alias for NotEqual).
    SlashEqual,
    /// Less than operator (<).
    LessThan,
    /// Less than operator (alias for LessThan).
    Less,
    /// Greater than operator (>).
    GreaterThan,
    /// Greater than operator (alias for GreaterThan).
    Greater,
    /// Less than or equal operator (<=).
    LessEqual,
    /// Greater than or equal operator (>=).
    GreaterEqual,
    /// Assignment operator (=).
    Assign,
    /// Pointer assignment operator (=>).
    Arrow,
    /// Logical AND operator (.and.).
    And,
    /// Logical OR operator (.or.).
    Or,
    /// Logical NOT operator (.not.).
    Not,
    /// Logical equivalence operator (.eqv.).
    Eqv,
    /// Logical equivalence operator (alias for Eqv).
    Equivalent,
    /// Logical non-equivalence operator (.neqv.).
    Neqv,
    /// Logical non-equivalence operator (alias for Neqv).
    NotEquivalent,
    /// Equality operator (.eq.).
    Eq,
    /// Inequality operator (.ne.).
    Ne,
    /// Less than operator (.lt.).
    Lt,
    /// Less than or equal operator (.le.).
    Le,
    /// Greater than operator (.gt.).
    Gt,
    /// Greater than or equal operator (.ge.).
    Ge,

    /// Left parenthesis delimiter (().
    LeftParen,
    /// Right parenthesis delimiter ()).
    RightParen,
    /// Left bracket delimiter ([).
    LeftBracket,
    /// Right bracket delimiter (]).
    RightBracket,
    /// Comma delimiter (,).
    Comma,
    /// Colon delimiter (:).
    Colon,
    /// Double colon delimiter (::).
    DoubleColon,
    /// Double colon delimiter (alias for DoubleColon).
    ColonColon,
    /// Semicolon delimiter (;).
    Semicolon,
    /// Percent delimiter (%).
    Percent,
    /// Ampersand delimiter (&).
    Ampersand,
    /// Dot delimiter (.).
    Dot,

    /// Root node of the syntax tree.
    Root,

    /// Error token.
    Error,
    /// End of file token.
    Eof,
    /// End file keyword.
    EndFile,
}

impl FortranElementType {
    /// Returns true if this element type is trivia (whitespace, newline, or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    /// Returns true if this element type is a Fortran keyword.
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::Program
                | Self::EndProgram
                | Self::Subroutine
                | Self::EndSubroutine
                | Self::Function
                | Self::EndFunction
                | Self::Module
                | Self::EndModule
                | Self::Interface
                | Self::EndInterface
                | Self::Type
                | Self::EndType
                | Self::If
                | Self::Then
                | Self::ElseIf
                | Self::Else
                | Self::EndIf
                | Self::Do
                | Self::EndDo
                | Self::While
                | Self::Select
                | Self::Case
                | Self::EndSelect
                | Self::Where
                | Self::EndWhere
                | Self::Forall
                | Self::EndForall
                | Self::Associate
                | Self::EndAssociate
                | Self::Block
                | Self::EndBlock
                | Self::Critical
                | Self::EndCritical
                | Self::Procedure
                | Self::EndProcedure
                | Self::Abstract
                | Self::Allocatable
                | Self::Allocate
                | Self::Deallocate
                | Self::Assignment
                | Self::Bind
                | Self::Call
                | Self::Class
                | Self::Common
                | Self::Contains
                | Self::Continue
                | Self::Cycle
                | Self::Data
                | Self::Default
                | Self::Dimension
                | Self::Elemental
                | Self::Entry
                | Self::Equivalence
                | Self::Exit
                | Self::External
                | Self::Final
                | Self::Format
                | Self::Generic
                | Self::Go
                | Self::Goto
                | Self::Implicit
                | Self::Import
                | Self::Include
                | Self::Intent
                | Self::Intrinsic
                | Self::Kind
                | Self::Len
                | Self::None
                | Self::Nullify
                | Self::Only
                | Self::Optional
                | Self::Parameter
                | Self::Pause
                | Self::Pointer
                | Self::Print
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Pure
                | Self::Read
                | Self::Recursive
                | Self::Result
                | Self::Return
                | Self::Rewind
                | Self::Save
                | Self::Stop
                | Self::Target
                | Self::Use
                | Self::Value
                | Self::Volatile
                | Self::Wait
                | Self::Write
                | Self::Inquire
                | Self::Backspace
                | Self::Close
                | Self::Open
                | Self::To
                | Self::End
                | Self::Double
                | Self::Precision
                | Self::Integer
                | Self::Real
                | Self::DoublePrecision
                | Self::Complex
                | Self::Character
                | Self::Logical
        )
    }
}

impl ElementType for FortranElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::FortranTokenType> for FortranElementType {
    fn from(token: crate::lexer::token_type::FortranTokenType) -> Self {
                match token {
            crate::lexer::token_type::FortranTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::FortranTokenType::Newline => Self::Newline,
            crate::lexer::token_type::FortranTokenType::Comment => Self::Comment,
            crate::lexer::token_type::FortranTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::FortranTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::FortranTokenType::Number => Self::Number,
            crate::lexer::token_type::FortranTokenType::NumberLiteral => Self::NumberLiteral,
            crate::lexer::token_type::FortranTokenType::RealLiteral => Self::RealLiteral,
            crate::lexer::token_type::FortranTokenType::DoublePrecisionLiteral => Self::DoublePrecisionLiteral,
            crate::lexer::token_type::FortranTokenType::ComplexLiteral => Self::ComplexLiteral,
            crate::lexer::token_type::FortranTokenType::CharacterLiteral => Self::CharacterLiteral,
            crate::lexer::token_type::FortranTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::FortranTokenType::String => Self::String,
            crate::lexer::token_type::FortranTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::FortranTokenType::LogicalLiteral => Self::LogicalLiteral,
            crate::lexer::token_type::FortranTokenType::True => Self::True,
            crate::lexer::token_type::FortranTokenType::False => Self::False,
            crate::lexer::token_type::FortranTokenType::Program => Self::Program,
            crate::lexer::token_type::FortranTokenType::EndProgram => Self::EndProgram,
            crate::lexer::token_type::FortranTokenType::Subroutine => Self::Subroutine,
            crate::lexer::token_type::FortranTokenType::EndSubroutine => Self::EndSubroutine,
            crate::lexer::token_type::FortranTokenType::Function => Self::Function,
            crate::lexer::token_type::FortranTokenType::EndFunction => Self::EndFunction,
            crate::lexer::token_type::FortranTokenType::Module => Self::Module,
            crate::lexer::token_type::FortranTokenType::EndModule => Self::EndModule,
            crate::lexer::token_type::FortranTokenType::Interface => Self::Interface,
            crate::lexer::token_type::FortranTokenType::EndInterface => Self::EndInterface,
            crate::lexer::token_type::FortranTokenType::Type => Self::Type,
            crate::lexer::token_type::FortranTokenType::EndType => Self::EndType,
            crate::lexer::token_type::FortranTokenType::If => Self::If,
            crate::lexer::token_type::FortranTokenType::Then => Self::Then,
            crate::lexer::token_type::FortranTokenType::ElseIf => Self::ElseIf,
            crate::lexer::token_type::FortranTokenType::Else => Self::Else,
            crate::lexer::token_type::FortranTokenType::EndIf => Self::EndIf,
            crate::lexer::token_type::FortranTokenType::Do => Self::Do,
            crate::lexer::token_type::FortranTokenType::EndDo => Self::EndDo,
            crate::lexer::token_type::FortranTokenType::While => Self::While,
            crate::lexer::token_type::FortranTokenType::Select => Self::Select,
            crate::lexer::token_type::FortranTokenType::Case => Self::Case,
            crate::lexer::token_type::FortranTokenType::EndSelect => Self::EndSelect,
            crate::lexer::token_type::FortranTokenType::Where => Self::Where,
            crate::lexer::token_type::FortranTokenType::EndWhere => Self::EndWhere,
            crate::lexer::token_type::FortranTokenType::Forall => Self::Forall,
            crate::lexer::token_type::FortranTokenType::EndForall => Self::EndForall,
            crate::lexer::token_type::FortranTokenType::Associate => Self::Associate,
            crate::lexer::token_type::FortranTokenType::EndAssociate => Self::EndAssociate,
            crate::lexer::token_type::FortranTokenType::Block => Self::Block,
            crate::lexer::token_type::FortranTokenType::EndBlock => Self::EndBlock,
            crate::lexer::token_type::FortranTokenType::Critical => Self::Critical,
            crate::lexer::token_type::FortranTokenType::EndCritical => Self::EndCritical,
            crate::lexer::token_type::FortranTokenType::Procedure => Self::Procedure,
            crate::lexer::token_type::FortranTokenType::EndProcedure => Self::EndProcedure,
            crate::lexer::token_type::FortranTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::FortranTokenType::Allocatable => Self::Allocatable,
            crate::lexer::token_type::FortranTokenType::Allocate => Self::Allocate,
            crate::lexer::token_type::FortranTokenType::Deallocate => Self::Deallocate,
            crate::lexer::token_type::FortranTokenType::Assignment => Self::Assignment,
            crate::lexer::token_type::FortranTokenType::Bind => Self::Bind,
            crate::lexer::token_type::FortranTokenType::Call => Self::Call,
            crate::lexer::token_type::FortranTokenType::Class => Self::Class,
            crate::lexer::token_type::FortranTokenType::Common => Self::Common,
            crate::lexer::token_type::FortranTokenType::Contains => Self::Contains,
            crate::lexer::token_type::FortranTokenType::Continue => Self::Continue,
            crate::lexer::token_type::FortranTokenType::Cycle => Self::Cycle,
            crate::lexer::token_type::FortranTokenType::Data => Self::Data,
            crate::lexer::token_type::FortranTokenType::Default => Self::Default,
            crate::lexer::token_type::FortranTokenType::Dimension => Self::Dimension,
            crate::lexer::token_type::FortranTokenType::Elemental => Self::Elemental,
            crate::lexer::token_type::FortranTokenType::Entry => Self::Entry,
            crate::lexer::token_type::FortranTokenType::Equivalence => Self::Equivalence,
            crate::lexer::token_type::FortranTokenType::Exit => Self::Exit,
            crate::lexer::token_type::FortranTokenType::External => Self::External,
            crate::lexer::token_type::FortranTokenType::Final => Self::Final,
            crate::lexer::token_type::FortranTokenType::Format => Self::Format,
            crate::lexer::token_type::FortranTokenType::Generic => Self::Generic,
            crate::lexer::token_type::FortranTokenType::Go => Self::Go,
            crate::lexer::token_type::FortranTokenType::Goto => Self::Goto,
            crate::lexer::token_type::FortranTokenType::Implicit => Self::Implicit,
            crate::lexer::token_type::FortranTokenType::Import => Self::Import,
            crate::lexer::token_type::FortranTokenType::Include => Self::Include,
            crate::lexer::token_type::FortranTokenType::Intent => Self::Intent,
            crate::lexer::token_type::FortranTokenType::Intrinsic => Self::Intrinsic,
            crate::lexer::token_type::FortranTokenType::Kind => Self::Kind,
            crate::lexer::token_type::FortranTokenType::Len => Self::Len,
            crate::lexer::token_type::FortranTokenType::None => Self::None,
            crate::lexer::token_type::FortranTokenType::Nullify => Self::Nullify,
            crate::lexer::token_type::FortranTokenType::Only => Self::Only,
            crate::lexer::token_type::FortranTokenType::Optional => Self::Optional,
            crate::lexer::token_type::FortranTokenType::Parameter => Self::Parameter,
            crate::lexer::token_type::FortranTokenType::Pause => Self::Pause,
            crate::lexer::token_type::FortranTokenType::Pointer => Self::Pointer,
            crate::lexer::token_type::FortranTokenType::Print => Self::Print,
            crate::lexer::token_type::FortranTokenType::Private => Self::Private,
            crate::lexer::token_type::FortranTokenType::Protected => Self::Protected,
            crate::lexer::token_type::FortranTokenType::Public => Self::Public,
            crate::lexer::token_type::FortranTokenType::Pure => Self::Pure,
            crate::lexer::token_type::FortranTokenType::Read => Self::Read,
            crate::lexer::token_type::FortranTokenType::Recursive => Self::Recursive,
            crate::lexer::token_type::FortranTokenType::Result => Self::Result,
            crate::lexer::token_type::FortranTokenType::Return => Self::Return,
            crate::lexer::token_type::FortranTokenType::Rewind => Self::Rewind,
            crate::lexer::token_type::FortranTokenType::Save => Self::Save,
            crate::lexer::token_type::FortranTokenType::Stop => Self::Stop,
            crate::lexer::token_type::FortranTokenType::Target => Self::Target,
            crate::lexer::token_type::FortranTokenType::Use => Self::Use,
            crate::lexer::token_type::FortranTokenType::Value => Self::Value,
            crate::lexer::token_type::FortranTokenType::Volatile => Self::Volatile,
            crate::lexer::token_type::FortranTokenType::Wait => Self::Wait,
            crate::lexer::token_type::FortranTokenType::Write => Self::Write,
            crate::lexer::token_type::FortranTokenType::Inquire => Self::Inquire,
            crate::lexer::token_type::FortranTokenType::Backspace => Self::Backspace,
            crate::lexer::token_type::FortranTokenType::Close => Self::Close,
            crate::lexer::token_type::FortranTokenType::Open => Self::Open,
            crate::lexer::token_type::FortranTokenType::To => Self::To,
            crate::lexer::token_type::FortranTokenType::End => Self::End,
            crate::lexer::token_type::FortranTokenType::Double => Self::Double,
            crate::lexer::token_type::FortranTokenType::Precision => Self::Precision,
            crate::lexer::token_type::FortranTokenType::Integer => Self::Integer,
            crate::lexer::token_type::FortranTokenType::Real => Self::Real,
            crate::lexer::token_type::FortranTokenType::DoublePrecision => Self::DoublePrecision,
            crate::lexer::token_type::FortranTokenType::Complex => Self::Complex,
            crate::lexer::token_type::FortranTokenType::Character => Self::Character,
            crate::lexer::token_type::FortranTokenType::Logical => Self::Logical,
            crate::lexer::token_type::FortranTokenType::Plus => Self::Plus,
            crate::lexer::token_type::FortranTokenType::Minus => Self::Minus,
            crate::lexer::token_type::FortranTokenType::Star => Self::Star,
            crate::lexer::token_type::FortranTokenType::Slash => Self::Slash,
            crate::lexer::token_type::FortranTokenType::StarStar => Self::StarStar,
            crate::lexer::token_type::FortranTokenType::Power => Self::Power,
            crate::lexer::token_type::FortranTokenType::Concatenate => Self::Concatenate,
            crate::lexer::token_type::FortranTokenType::Equal => Self::Equal,
            crate::lexer::token_type::FortranTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::FortranTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::FortranTokenType::SlashEqual => Self::SlashEqual,
            crate::lexer::token_type::FortranTokenType::LessThan => Self::LessThan,
            crate::lexer::token_type::FortranTokenType::Less => Self::Less,
            crate::lexer::token_type::FortranTokenType::GreaterThan => Self::GreaterThan,
            crate::lexer::token_type::FortranTokenType::Greater => Self::Greater,
            crate::lexer::token_type::FortranTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::FortranTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::FortranTokenType::Assign => Self::Assign,
            crate::lexer::token_type::FortranTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::FortranTokenType::And => Self::And,
            crate::lexer::token_type::FortranTokenType::Or => Self::Or,
            crate::lexer::token_type::FortranTokenType::Not => Self::Not,
            crate::lexer::token_type::FortranTokenType::Eqv => Self::Eqv,
            crate::lexer::token_type::FortranTokenType::Equivalent => Self::Equivalent,
            crate::lexer::token_type::FortranTokenType::Neqv => Self::Neqv,
            crate::lexer::token_type::FortranTokenType::NotEquivalent => Self::NotEquivalent,
            crate::lexer::token_type::FortranTokenType::Eq => Self::Eq,
            crate::lexer::token_type::FortranTokenType::Ne => Self::Ne,
            crate::lexer::token_type::FortranTokenType::Lt => Self::Lt,
            crate::lexer::token_type::FortranTokenType::Le => Self::Le,
            crate::lexer::token_type::FortranTokenType::Gt => Self::Gt,
            crate::lexer::token_type::FortranTokenType::Ge => Self::Ge,
            crate::lexer::token_type::FortranTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::FortranTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::FortranTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::FortranTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::FortranTokenType::Comma => Self::Comma,
            crate::lexer::token_type::FortranTokenType::Colon => Self::Colon,
            crate::lexer::token_type::FortranTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::FortranTokenType::ColonColon => Self::ColonColon,
            crate::lexer::token_type::FortranTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::FortranTokenType::Percent => Self::Percent,
            crate::lexer::token_type::FortranTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::FortranTokenType::Dot => Self::Dot,
            crate::lexer::token_type::FortranTokenType::Root => Self::Root,
            crate::lexer::token_type::FortranTokenType::Error => Self::Error,
            crate::lexer::token_type::FortranTokenType::Eof => Self::Eof,
            crate::lexer::token_type::FortranTokenType::EndFile => Self::EndFile,
            _ => Self::Error,
        }
    }
}
