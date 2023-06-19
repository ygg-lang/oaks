use oak_core::{SyntaxKind, Token};
use serde::Serialize;

pub type FortranToken = Token<FortranSyntaxKind>;

/// Fortran 令牌种类
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize)]
pub enum FortranSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,

    // 标识符和字面量
    Identifier,
    IntegerLiteral,
    Number,
    NumberLiteral,
    RealLiteral,
    DoublePrecisionLiteral,
    ComplexLiteral,
    CharacterLiteral,
    CharLiteral,
    String,
    StringLiteral,
    LogicalLiteral,
    True,
    False,

    // Fortran 关键字
    Program,
    EndProgram,
    Subroutine,
    EndSubroutine,
    Function,
    EndFunction,
    Module,
    EndModule,
    Interface,
    EndInterface,
    Type,
    EndType,
    If,
    Then,
    ElseIf,
    Else,
    EndIf,
    Do,
    EndDo,
    While,
    Select,
    Case,
    EndSelect,
    Where,
    EndWhere,
    Forall,
    EndForall,
    Associate,
    EndAssociate,
    Block,
    EndBlock,
    Critical,
    EndCritical,
    Procedure,
    EndProcedure,
    Abstract,
    Allocatable,
    Allocate,
    Deallocate,
    Assignment,
    Bind,
    Call,
    Class,
    Common,
    Contains,
    Continue,
    Cycle,
    Data,
    Default,
    Dimension,
    Elemental,
    Entry,
    Equivalence,
    Exit,
    External,
    Final,
    Format,
    Generic,
    Go,
    Goto,
    Implicit,
    Import,
    Include,
    Intent,
    Intrinsic,
    Kind,
    Len,
    None,
    Nullify,
    Only,
    Optional,
    Parameter,
    Pause,
    Pointer,
    Print,
    Private,
    Protected,
    Public,
    Pure,
    Read,
    Recursive,
    Result,
    Return,
    Rewind,
    Save,
    Stop,
    Target,
    Use,
    Value,
    Volatile,
    Wait,
    Write,
    Inquire,
    Backspace,
    Close,
    Open,
    To,
    End,
    Double,
    Precision,

    // 数据类型
    Integer,
    Real,
    DoublePrecision,
    Complex,
    Character,
    Logical,

    // 操作符
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    StarStar,      // **
    Power,         // ** (alias for StarStar)
    Concatenate,   // //
    Equal,         // ==
    EqualEqual,    // == (alias for Equal)
    NotEqual,      // /=
    SlashEqual,    // /= (alias for NotEqual)
    LessThan,      // <
    Less,          // < (alias for LessThan)
    GreaterThan,   // >
    Greater,       // > (alias for GreaterThan)
    LessEqual,     // <=
    GreaterEqual,  // >=
    Assign,        // =
    Arrow,         // =>
    And,           // .and.
    Or,            // .or.
    Not,           // .not.
    Eqv,           // .eqv.
    Equivalent,    // .eqv. (alias for Eqv)
    Neqv,          // .neqv.
    NotEquivalent, // .neqv. (alias for Neqv)
    Eq,            // .eq.
    Ne,            // .ne.
    Lt,            // .lt.
    Le,            // .le.
    Gt,            // .gt.
    Ge,            // .ge.

    // 分隔符
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Colon,        // :
    DoubleColon,  // ::
    ColonColon,   // :: (alias for DoubleColon)
    Semicolon,    // ;
    Percent,      // %
    Ampersand,    // &
    Dot,          // .

    // 特殊
    Error,
    Eof,
    EndFile,
}

impl FortranSyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

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

impl SyntaxKind for FortranSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, FortranSyntaxKind::Whitespace | FortranSyntaxKind::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, FortranSyntaxKind::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, FortranSyntaxKind::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, FortranSyntaxKind::Error | FortranSyntaxKind::Eof)
    }

    fn is_element_type(&self) -> bool {
        false
    }
}
