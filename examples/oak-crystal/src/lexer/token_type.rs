use oak_core::{Token, TokenType, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type CrystalToken = Token<CrystalTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CrystalTokenType {
    Whitespace,
    Comment,
    Identifier,
    Number,
    String,
    Character,
    Symbol,
    ClassKeyword,
    ModuleKeyword,
    DefKeyword,
    EndKeyword,
    IfKeyword,
    ElseKeyword,
    ElsifKeyword,
    UnlessKeyword,
    CaseKeyword,
    WhenKeyword,
    ThenKeyword,
    WhileKeyword,
    UntilKeyword,
    ForKeyword,
    InKeyword,
    DoKeyword,
    BeginKeyword,
    RescueKeyword,
    EnsureKeyword,
    BreakKeyword,
    NextKeyword,
    ReturnKeyword,
    YieldKeyword,
    SuperKeyword,
    SelfKeyword,
    TrueKeyword,
    FalseKeyword,
    NilKeyword,
    AndKeyword,
    OrKeyword,
    NotKeyword,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Spaceship,
    Match,
    NotMatch,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    LogicalAnd,
    LogicalOr,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    StarStarEqual,
    AndEqual,
    OrEqual,
    XorEqual,
    LeftShiftEqual,
    RightShiftEqual,
    LogicalAndEqual,
    LogicalOrEqual,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Dot,
    DotDot,
    DotDotDot,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,
    Question,
    At,
    DoubleAt,
    Dollar,
    Newline,
    Eof,
    Error,
    Root,
    Program,
    SourceFile,
    ClassDef,
    ModuleDef,
    MethodDef,
    Block,
    IfExpr,
    UnlessExpr,
    CaseExpr,
    WhenClause,
    WhileExpr,
    UntilExpr,
    ForExpr,
    BeginExpr,
    RescueClause,
    EnsureClause,
    CallExpr,
    IndexExpr,
    MemberExpr,
    BinaryExpr,
    UnaryExpr,
    AssignExpr,
    LiteralExpr,
    IdentifierExpr,
    ArrayExpr,
    HashExpr,
    HashPair,
    BlockExpr,
    LambdaExpr,
    YieldExpr,
    ReturnExpr,
    BreakExpr,
    NextExpr,
    SuperExpr,
    SelfExpr,
    ParenExpr,
    TypeExpr,
    GenericType,
    UnionType,
    TupleType,
    NamedTupleType,
    ProcType,
    Pattern,
    IdentifierPattern,
    LiteralPattern,
    ArrayPattern,
    HashPattern,
    TuplePattern,
    ParamList,
    Param,
    SplatParam,
    DoubleSplatParam,
    BlockParam,
    Annotation,
    MacroDef,
    MacroCall,
    MacroExpr,
    Alias,
    Include,
    Extend,
    Require,
    Private,
    Protected,
    Public,
    Abstract,
    Virtual,
    Override,
    StructDef,
    EnumDef,
    UnionDef,
    LibDef,
    RaiseExpr,
    RangeExpr,
    ExclusiveRangeExpr,
    RegexLiteral,
    StringInterpolation,
    InterpolationExpr,
    SymbolLiteral,
    ConstantRef,
    InstanceVar,
    ClassVar,
    GlobalVar,
    Getter,
    Setter,
    OperatorDef,
}

impl CrystalTokenType {
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

impl Display for CrystalTokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType for CrystalTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            _ => UniversalTokenRole::None,
        }
    }
}
