use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

/// Type alias for a Fortran token.
pub type FortranToken = Token<FortranTokenType>;

impl FortranTokenType {
    /// Returns `true` if the token is trivia (whitespace, newline, or comment).
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    /// Returns `true` if the token is a keyword.
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

impl TokenType for FortranTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        self.is_trivia()
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

/// Token types for Fortran.
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum FortranTokenType {
    // Basic kind
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Comment.
    Comment,

    // Identifiers and literals
    /// Identifier.
    Identifier,
    /// Integer literal.
    IntegerLiteral,
    /// Number.
    Number,
    /// Number literal.
    NumberLiteral,
    /// Real literal.
    RealLiteral,
    /// Double precision literal.
    DoublePrecisionLiteral,
    /// Complex literal.
    ComplexLiteral,
    /// Character literal.
    CharacterLiteral,
    /// Character literal (alias).
    CharLiteral,
    /// String.
    String,
    /// String literal.
    StringLiteral,
    /// Logical literal.
    LogicalLiteral,
    /// `.true.` literal.
    True,
    /// `.false.` literal.
    False,

    // Fortran keywords
    /// `program` keyword.
    Program,
    /// `end program` keyword.
    EndProgram,
    /// `subroutine` keyword.
    Subroutine,
    /// `end subroutine` keyword.
    EndSubroutine,
    /// `function` keyword.
    Function,
    /// `end function` keyword.
    EndFunction,
    /// `module` keyword.
    Module,
    /// `end module` keyword.
    EndModule,
    /// `interface` keyword.
    Interface,
    /// `end interface` keyword.
    EndInterface,
    /// `type` keyword.
    Type,
    /// `end type` keyword.
    EndType,
    /// `if` keyword.
    If,
    /// `then` keyword.
    Then,
    /// `else if` keyword.
    ElseIf,
    /// `else` keyword.
    Else,
    /// `end if` keyword.
    EndIf,
    /// `do` keyword.
    Do,
    /// `end do` keyword.
    EndDo,
    /// `while` keyword.
    While,
    /// `select` keyword.
    Select,
    /// `case` keyword.
    Case,
    /// `end select` keyword.
    EndSelect,
    /// `where` keyword.
    Where,
    /// `end where` keyword.
    EndWhere,
    /// `forall` keyword.
    Forall,
    /// `end forall` keyword.
    EndForall,
    /// `associate` keyword.
    Associate,
    /// `end associate` keyword.
    EndAssociate,
    /// `block` keyword.
    Block,
    /// `end block` keyword.
    EndBlock,
    /// `critical` keyword.
    Critical,
    /// `end critical` keyword.
    EndCritical,
    /// `procedure` keyword.
    Procedure,
    /// `end procedure` keyword.
    EndProcedure,
    /// `abstract` keyword.
    Abstract,
    /// `allocatable` keyword.
    Allocatable,
    /// `allocate` keyword.
    Allocate,
    /// `deallocate` keyword.
    Deallocate,
    /// `assignment` keyword.
    Assignment,
    /// `bind` keyword.
    Bind,
    /// `call` keyword.
    Call,
    /// `class` keyword.
    Class,
    /// `common` keyword.
    Common,
    /// `contains` keyword.
    Contains,
    /// `continue` keyword.
    Continue,
    /// `cycle` keyword.
    Cycle,
    /// `data` keyword.
    Data,
    /// `default` keyword.
    Default,
    /// `dimension` keyword.
    Dimension,
    /// `elemental` keyword.
    Elemental,
    /// `entry` keyword.
    Entry,
    /// `equivalence` keyword.
    Equivalence,
    /// `exit` keyword.
    Exit,
    /// `external` keyword.
    External,
    /// `final` keyword.
    Final,
    /// `format` keyword.
    Format,
    /// `generic` keyword.
    Generic,
    /// `go` keyword.
    Go,
    /// `goto` keyword.
    Goto,
    /// `implicit` keyword.
    Implicit,
    /// `import` keyword.
    Import,
    /// `include` keyword.
    Include,
    /// `intent` keyword.
    Intent,
    /// `intrinsic` keyword.
    Intrinsic,
    /// `kind` keyword.
    Kind,
    /// `len` keyword.
    Len,
    /// `none` keyword.
    None,
    /// `nullify` keyword.
    Nullify,
    /// `only` keyword.
    Only,
    /// `optional` keyword.
    Optional,
    /// `parameter` keyword.
    Parameter,
    /// `pause` keyword.
    Pause,
    /// `pointer` keyword.
    Pointer,
    /// `print` keyword.
    Print,
    /// `private` keyword.
    Private,
    /// `protected` keyword.
    Protected,
    /// `public` keyword.
    Public,
    /// `pure` keyword.
    Pure,
    /// `read` keyword.
    Read,
    /// `recursive` keyword.
    Recursive,
    /// `result` keyword.
    Result,
    /// `return` keyword.
    Return,
    /// `rewind` keyword.
    Rewind,
    /// `save` keyword.
    Save,
    /// `stop` keyword.
    Stop,
    /// `target` keyword.
    Target,
    /// `use` keyword.
    Use,
    /// `value` keyword.
    Value,
    /// `volatile` keyword.
    Volatile,
    /// `wait` keyword.
    Wait,
    /// `write` keyword.
    Write,
    /// `inquire` keyword.
    Inquire,
    /// `backspace` keyword.
    Backspace,
    /// `close` keyword.
    Close,
    /// `open` keyword.
    Open,
    /// `to` keyword.
    To,
    /// `end` keyword.
    End,
    /// `double` keyword.
    Double,
    /// `precision` keyword.
    Precision,

    // Data types
    /// `integer` type.
    Integer,
    /// `real` type.
    Real,
    /// `double precision` type.
    DoublePrecision,
    /// `complex` type.
    Complex,
    /// `character` type.
    Character,
    /// `logical` type.
    Logical,

    // Operators
    /// Plus `+`.
    Plus, // +
    /// Minus `-`.
    Minus, // -
    /// Star `*`.
    Star, // *
    /// Slash `/`.
    Slash, // /
    /// Double star `**`.
    StarStar, // **
    /// Power `**` (alias).
    Power, // ** (alias for StarStar)
    /// Concatenate `//`.
    Concatenate, // //
    /// Equal `==`.
    Equal, // ==
    /// Equal equal `==` (alias).
    EqualEqual, // == (alias for Equal)
    /// Not equal `/=`.
    NotEqual, // /=
    /// Slash equal `/=` (alias).
    SlashEqual, // /= (alias for NotEqual)
    /// Less than `<`.
    LessThan, // <
    /// Less than `<` (alias).
    Less, // < (alias for LessThan)
    /// Greater than `>`.
    GreaterThan, // >
    /// Greater than `>` (alias).
    Greater, // > (alias for GreaterThan)
    /// Less than or equal `<=`.
    LessEqual, // <=
    /// Greater than or equal `>=`.
    GreaterEqual, // >=
    /// Assign `=`.
    Assign, // =
    /// Arrow `=>`.
    Arrow, // =>
    /// Logical AND `.and.`.
    And, // .and.
    /// Logical OR `.or.`.
    Or, // .or.
    /// Logical NOT `.not.`.
    Not, // .not.
    /// Logical EQV `.eqv.`.
    Eqv, // .eqv.
    /// Logical EQV (alias).
    Equivalent, // .eqv. (alias for Eqv)
    /// Logical NEQV `.neqv.`.
    Neqv, // .neqv.
    /// Logical NEQV (alias).
    NotEquivalent, // .neqv. (alias for Neqv)
    /// Relational EQ `.eq.`.
    Eq, // .eq.
    /// Relational NE `.ne.`.
    Ne, // .ne.
    /// Relational LT `.lt.`.
    Lt, // .lt.
    /// Relational LE `.le.`.
    Le, // .le.
    /// Relational GT `.gt.`.
    Gt, // .gt.
    /// Relational GE `.ge.`.
    Ge, // .ge.

    // Delimiters
    /// Left parenthesis `(`.
    LeftParen, // (
    /// Right parenthesis `)`.
    RightParen, // )
    /// Left bracket `[`.
    LeftBracket, // [
    /// Right bracket `]`.
    RightBracket, // ]
    /// Comma `,`.
    Comma, // ,
    /// Colon `:`.
    Colon, // :
    /// Double colon `::`.
    DoubleColon, // ::
    /// Double colon `::` (alias).
    ColonColon, // :: (alias for DoubleColon)
    /// Semicolon `;`.
    Semicolon, // ;
    /// Percent `%`.
    Percent, // %
    /// Ampersand `&`.
    Ampersand, // &
    /// Dot `.`.
    Dot, // .

    // Syntax node types
    /// Root node.
    Root,

    // Special
    /// Error token.
    Error,
    /// End of file.
    Eof,
    /// End of file (alternate).
    EndFile,
}
