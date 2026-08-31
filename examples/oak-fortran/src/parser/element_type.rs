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
        unsafe { std::mem::transmute(token) }
    }
}
