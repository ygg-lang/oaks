use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use std::fmt::{Display, Formatter};

pub type CrystalToken = Token<CrystalSyntaxKind>;

/// Crystal syntax node types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CrystalSyntaxKind {
    /// Whitespace characters
    Whitespace,
    /// Comment
    Comment,
    /// Identifier
    Identifier,
    /// Number literal
    Number,
    /// String literal
    String,
    /// Character literal
    Character,
    /// Symbol
    Symbol,
    /// class keyword
    ClassKeyword,
    /// module keyword
    ModuleKeyword,
    /// def keyword
    DefKeyword,
    /// end keyword
    EndKeyword,
    /// if keyword
    IfKeyword,
    /// else keyword
    ElseKeyword,
    /// elsif keyword
    ElsifKeyword,
    /// unless keyword
    UnlessKeyword,
    /// case keyword
    CaseKeyword,
    /// when keyword
    WhenKeyword,
    /// then keyword
    ThenKeyword,
    /// while keyword
    WhileKeyword,
    /// until keyword
    UntilKeyword,
    /// for keyword
    ForKeyword,
    /// in keyword
    InKeyword,
    /// do keyword
    DoKeyword,
    /// begin keyword
    BeginKeyword,
    /// rescue keyword
    RescueKeyword,
    /// ensure keyword
    EnsureKeyword,
    /// break keyword
    BreakKeyword,
    /// next keyword
    NextKeyword,
    /// return keyword
    ReturnKeyword,
    /// yield keyword
    YieldKeyword,
    /// super keyword
    SuperKeyword,
    /// self keyword
    SelfKeyword,
    /// true keyword
    TrueKeyword,
    /// false keyword
    FalseKeyword,
    /// nil keyword
    NilKeyword,
    /// and keyword
    AndKeyword,
    /// or keyword
    OrKeyword,
    /// not keyword
    NotKeyword,
    /// plus operator
    Plus,
    /// minus operator
    Minus,
    /// multiplication operator
    Star,
    /// division operator
    Slash,
    /// modulo operator
    Percent,
    /// exponentiation operator
    StarStar,
    /// assignment operator
    Equal,
    /// equality operator
    EqualEqual,
    /// inequality operator
    NotEqual,
    /// less than operator
    Less,
    /// less than or equal operator
    LessEqual,
    /// greater than operator
    Greater,
    /// greater than or equal operator
    GreaterEqual,
    /// spaceship operator
    Spaceship,
    /// match operator
    Match,
    /// not match operator
    NotMatch,
    /// and operator
    And,
    /// or operator
    Or,
    /// not operator
    Not,
    /// bitwise and operator
    BitwiseAnd,
    /// bitwise or operator
    BitwiseOr,
    /// bitwise xor operator
    BitwiseXor,
    /// bitwise not operator
    BitwiseNot,
    /// left shift operator
    LeftShift,
    /// right shift operator
    RightShift,
    /// logical and operator
    LogicalAnd,
    /// logical or operator
    LogicalOr,
    /// plus assignment operator
    PlusEqual,
    /// minus assignment operator
    MinusEqual,
    /// multiplication assignment operator
    StarEqual,
    /// division assignment operator
    SlashEqual,
    /// modulo assignment operator
    PercentEqual,
    /// exponentiation assignment operator
    StarStarEqual,
    /// and assignment operator
    AndEqual,
    /// or assignment operator
    OrEqual,
    /// xor assignment operator
    XorEqual,
    /// left shift assignment operator
    LeftShiftEqual,
    /// right shift assignment operator
    RightShiftEqual,
    /// logical and assignment operator
    LogicalAndEqual,
    /// logical or assignment operator
    LogicalOrEqual,
    /// left parenthesis
    LeftParen,
    /// right parenthesis
    RightParen,
    /// left brace
    LeftBrace,
    /// right brace
    RightBrace,
    /// left bracket
    LeftBracket,
    /// right bracket
    RightBracket,
    /// comma
    Comma,
    /// semicolon
    Semicolon,
    /// dot
    Dot,
    /// dot dot
    DotDot,
    /// dot dot dot
    DotDotDot,
    /// colon
    Colon,
    /// double colon
    DoubleColon,
    /// arrow
    Arrow,
    /// fat arrow
    FatArrow,
    /// question mark
    Question,
    /// at sign
    At,
    /// double at sign
    DoubleAt,
    /// dollar sign
    Dollar,
    /// newline
    Newline,
    /// end of file
    Eof,
    /// error
    Error,
    /// root node
    Root,
    /// program node
    Program,
    /// source file node
    SourceFile,
    /// class definition
    ClassDef,
    /// module definition
    ModuleDef,
    /// method definition
    MethodDef,
    /// block
    Block,
    /// if expression
    IfExpr,
    /// unless expression
    UnlessExpr,
    /// case expression
    CaseExpr,
    /// when clause
    WhenClause,
    /// while expression
    WhileExpr,
    /// until expression
    UntilExpr,
    /// for expression
    ForExpr,
    /// begin expression
    BeginExpr,
    /// rescue clause
    RescueClause,
    /// ensure clause
    EnsureClause,
    /// call expression
    CallExpr,
    /// index expression
    IndexExpr,
    /// member expression
    MemberExpr,
    /// binary expression
    BinaryExpr,
    /// unary expression
    UnaryExpr,
    /// assignment expression
    AssignExpr,
    /// literal expression
    LiteralExpr,
    /// identifier expression
    IdentifierExpr,
    /// array expression
    ArrayExpr,
    /// hash expression
    HashExpr,
    /// hash pair
    HashPair,
    /// block expression
    BlockExpr,
    /// lambda expression
    LambdaExpr,
    /// yield expression
    YieldExpr,
    /// return expression
    ReturnExpr,
    /// break expression
    BreakExpr,
    /// next expression
    NextExpr,
    /// super expression
    SuperExpr,
    /// self expression
    SelfExpr,
    /// parenthesized expression
    ParenExpr,
    /// type expression
    TypeExpr,
    /// generic type
    GenericType,
    /// union type
    UnionType,
    /// tuple type
    TupleType,
    /// named tuple type
    NamedTupleType,
    /// procedure type
    ProcType,
    /// pattern
    Pattern,
    /// identifier pattern
    IdentifierPattern,
    /// literal pattern
    LiteralPattern,
    /// array pattern
    ArrayPattern,
    /// hash pattern
    HashPattern,
    /// tuple pattern
    TuplePattern,
    /// parameter list
    ParamList,
    /// parameter
    Param,
    /// splat parameter
    SplatParam,
    /// double splat parameter
    DoubleSplatParam,
    /// block parameter
    BlockParam,
    /// annotation
    Annotation,
    /// macro definition
    MacroDef,
    /// macro call
    MacroCall,
    /// macro expression
    MacroExpr,
    /// alias
    Alias,
    /// include
    Include,
    /// extend
    Extend,
    /// require
    Require,
    /// private
    Private,
    /// protected
    Protected,
    /// public
    Public,
    /// abstract
    Abstract,
    /// virtual
    Virtual,
    /// override
    Override,
    /// struct definition
    StructDef,
    /// enum definition
    EnumDef,
    /// union definition
    UnionDef,
    /// lib definition
    LibDef,
    /// raise expression
    RaiseExpr,
    /// range expression
    RangeExpr,
    /// exclusive range expression
    ExclusiveRangeExpr,
    /// regex literal
    RegexLiteral,
    /// string interpolation
    StringInterpolation,
    /// interpolation expression
    InterpolationExpr,
    /// symbol literal
    SymbolLiteral,
    /// constant reference
    ConstantRef,
    /// instance variable
    InstanceVar,
    /// class variable
    ClassVar,
    /// global variable
    GlobalVar,
    /// getter method
    Getter,
    /// setter method
    Setter,
    /// operator definition
    OperatorDef,
}

impl CrystalSyntaxKind {
    /// Check if the syntax kind is trivia (whitespace, comment, or newline)
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment | Self::Newline)
    }

    /// Check if the syntax kind is a keyword
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::ClassKeyword
                | Self::ModuleKeyword
                | Self::DefKeyword
                | Self::EndKeyword
                | Self::IfKeyword
                | Self::ElseKeyword
                | Self::ElsifKeyword
                | Self::UnlessKeyword
                | Self::CaseKeyword
                | Self::WhenKeyword
                | Self::ThenKeyword
                | Self::WhileKeyword
                | Self::UntilKeyword
                | Self::ForKeyword
                | Self::InKeyword
                | Self::DoKeyword
                | Self::BeginKeyword
                | Self::RescueKeyword
                | Self::EnsureKeyword
                | Self::BreakKeyword
                | Self::NextKeyword
                | Self::ReturnKeyword
                | Self::YieldKeyword
                | Self::SuperKeyword
                | Self::SelfKeyword
                | Self::TrueKeyword
                | Self::FalseKeyword
                | Self::NilKeyword
                | Self::AndKeyword
                | Self::OrKeyword
                | Self::NotKeyword
        )
    }

    /// Check if the syntax kind is a literal
    pub fn is_literal(self) -> bool {
        matches!(self, Self::Number | Self::String | Self::Character | Self::Symbol | Self::RegexLiteral | Self::SymbolLiteral)
    }

    /// Check if the syntax kind is an operator
    pub fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::StarStar
                | Self::Equal
                | Self::EqualEqual
                | Self::NotEqual
                | Self::Less
                | Self::LessEqual
                | Self::Greater
                | Self::GreaterEqual
                | Self::Spaceship
                | Self::Match
                | Self::NotMatch
                | Self::And
                | Self::Or
                | Self::Not
                | Self::BitwiseAnd
                | Self::BitwiseOr
                | Self::BitwiseXor
                | Self::BitwiseNot
                | Self::LeftShift
                | Self::RightShift
                | Self::LogicalAnd
                | Self::LogicalOr
        )
    }

    /// Check if the syntax kind is an assignment operator
    pub fn is_assignment_operator(self) -> bool {
        matches!(
            self,
            Self::PlusEqual
                | Self::MinusEqual
                | Self::StarEqual
                | Self::SlashEqual
                | Self::PercentEqual
                | Self::StarStarEqual
                | Self::AndEqual
                | Self::OrEqual
                | Self::XorEqual
                | Self::LeftShiftEqual
                | Self::RightShiftEqual
                | Self::LogicalAndEqual
                | Self::LogicalOrEqual
        )
    }

    /// Check if the syntax kind is a delimiter
    pub fn is_delimiter(self) -> bool {
        matches!(self, Self::LeftParen | Self::RightParen | Self::LeftBrace | Self::RightBrace | Self::LeftBracket | Self::RightBracket | Self::Comma | Self::Semicolon)
    }
}

impl TokenType for CrystalSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::Number | Self::String | Self::Character | Self::Symbol | Self::RegexLiteral | Self::SymbolLiteral => UniversalTokenRole::Literal,
            Self::Error => UniversalTokenRole::Error,
            Self::Eof => UniversalTokenRole::Eof,
            k if k.is_keyword() => UniversalTokenRole::Keyword,
            k if k.is_operator() || k.is_assignment_operator() => UniversalTokenRole::Operator,
            k if k.is_delimiter() => UniversalTokenRole::Punctuation,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for CrystalSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl Display for CrystalSyntaxKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
