use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};

pub type VhdlToken = Token<VhdlTokenType>;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VhdlTokenType {
    Error,
    Eof,
    Whitespace,
    Comment,
    Identifier,

    // Literals
    StringLiteral,
    CharLiteral,
    BitStringLiteral,
    BasedLiteral,
    RealLiteral,
    IntegerLiteral,

    // Keywords
    EntityKw,
    ArchitectureKw,
    BeginKw,
    EndKw,
    ProcessKw,
    SignalKw,
    VariableKw,
    ConstantKw,
    ComponentKw,
    PortKw,
    MapKw,
    GenericKw,
    LibraryKw,
    UseKw,
    PackageKw,
    BodyKw,
    FunctionKw,
    ProcedureKw,
    TypeKw,
    SubtypeKw,
    RecordKw,
    ArrayKw,
    IfKw,
    ThenKw,
    ElseKw,
    ElsifKw,
    CaseKw,
    WhenKw,
    LoopKw,
    ForKw,
    WhileKw,
    ExitKw,
    NextKw,
    ReturnKw,
    WaitKw,
    UntilKw,
    InKw,
    OutKw,
    InoutKw,
    BufferKw,
    LinkageKw,
    DowntoKw,
    ToKw,
    GenerateKw,
    WithKw,
    SelectKw,
    AllKw,
    OthersKw,
    NullKw,
    OpenKw,
    IsKw,
    OfKw,
    RangeKw,
    ReverseRangeKw,
    AttributeKw,
    AliasKw,
    FileKw,
    AccessKw,
    AfterKw,
    AssertKw,
    ReportKw,
    SeverityKw,

    // Basic Types (Keywords)
    BitKw,
    BitVectorKw,
    BooleanKw,
    CharacterKw,
    IntegerKw,
    NaturalKw,
    PositiveKw,
    RealKw,
    StringKw,
    TimeKw,
    StdLogicKw,
    StdLogicVectorKw,
    UnsignedKw,
    SignedKw,

    // Logical Operators (Keywords)
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Xnor,
    Not,
    Sll,
    Srl,
    Sla,
    Sra,
    Rol,
    Ror,
    Mod,
    Rem,
    Abs,

    // Operators
    Arrow,
    Eq,
    Ne,
    Slash,
    Le,
    Lt,
    Ge,
    Gt,
    ShiftLeft,
    ShiftRight,
    Plus,
    Minus,
    Star,
    Pow,
    Assign,
    Colon,
    Ampersand,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Pipe,
    Hash,
    At,
    Question,
    Dollar,
    Percent,
    Caret,
    Tilde,
    Backslash,
    Exclamation,
}

impl TokenType for VhdlTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::BitStringLiteral | Self::BasedLiteral | Self::RealLiteral | Self::IntegerLiteral => UniversalTokenRole::Literal,
            Self::EntityKw
            | Self::ArchitectureKw
            | Self::BeginKw
            | Self::EndKw
            | Self::ProcessKw
            | Self::SignalKw
            | Self::VariableKw
            | Self::ConstantKw
            | Self::ComponentKw
            | Self::PortKw
            | Self::MapKw
            | Self::GenericKw
            | Self::LibraryKw
            | Self::UseKw
            | Self::PackageKw
            | Self::BodyKw
            | Self::FunctionKw
            | Self::ProcedureKw
            | Self::TypeKw
            | Self::SubtypeKw
            | Self::RecordKw
            | Self::ArrayKw
            | Self::IfKw
            | Self::ThenKw
            | Self::ElseKw
            | Self::ElsifKw
            | Self::CaseKw
            | Self::WhenKw
            | Self::LoopKw
            | Self::ForKw
            | Self::WhileKw
            | Self::ExitKw
            | Self::NextKw
            | Self::ReturnKw
            | Self::WaitKw
            | Self::UntilKw
            | Self::InKw
            | Self::OutKw
            | Self::InoutKw
            | Self::BufferKw
            | Self::LinkageKw
            | Self::DowntoKw
            | Self::ToKw
            | Self::GenerateKw
            | Self::WithKw
            | Self::SelectKw
            | Self::AllKw
            | Self::OthersKw
            | Self::NullKw
            | Self::OpenKw
            | Self::IsKw
            | Self::OfKw
            | Self::RangeKw
            | Self::ReverseRangeKw
            | Self::AttributeKw
            | Self::AliasKw
            | Self::FileKw
            | Self::AccessKw
            | Self::AfterKw
            | Self::AssertKw
            | Self::ReportKw
            | Self::SeverityKw
            | Self::BitKw
            | Self::BitVectorKw
            | Self::BooleanKw
            | Self::CharacterKw
            | Self::IntegerKw
            | Self::NaturalKw
            | Self::PositiveKw
            | Self::RealKw
            | Self::StringKw
            | Self::TimeKw
            | Self::StdLogicKw
            | Self::StdLogicVectorKw
            | Self::UnsignedKw
            | Self::SignedKw
            | Self::And
            | Self::Or
            | Self::Nand
            | Self::Nor
            | Self::Xor
            | Self::Xnor
            | Self::Not
            | Self::Sll
            | Self::Srl
            | Self::Sla
            | Self::Sra
            | Self::Rol
            | Self::Ror
            | Self::Mod
            | Self::Rem
            | Self::Abs => UniversalTokenRole::Keyword,
            Self::Arrow | Self::Eq | Self::Ne | Self::Slash | Self::Le | Self::Lt | Self::Ge | Self::Gt | Self::ShiftLeft | Self::ShiftRight | Self::Plus | Self::Minus | Self::Star | Self::Pow | Self::Assign | Self::Colon | Self::Ampersand => {
                UniversalTokenRole::Operator
            }
            Self::LeftParen
            | Self::RightParen
            | Self::LeftBracket
            | Self::RightBracket
            | Self::Semicolon
            | Self::Comma
            | Self::Dot
            | Self::Pipe
            | Self::Hash
            | Self::At
            | Self::Question
            | Self::Dollar
            | Self::Percent
            | Self::Caret
            | Self::Tilde
            | Self::Backslash
            | Self::Exclamation => UniversalTokenRole::Punctuation,
            Self::Error => UniversalTokenRole::None,
            Self::Eof => UniversalTokenRole::None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VhdlKeyword {
    Abs,
    Access,
    After,
    Alias,
    All,
    And,
    Architecture,
    Array,
    Assert,
    Attribute,
    Begin,
    Block,
    Body,
    Buffer,
    Bus,
    Case,
    Component,
    Configuration,
    Constant,
    Disconnect,
    Downto,
    Else,
    Elsif,
    End,
    Entity,
    Exit,
    File,
    For,
    Function,
    Generate,
    Generic,
    Group,
    Guarded,
    If,
    Impure,
    In,
    Inout,
    Is,
    Label,
    Library,
    Linkage,
    Literal,
    Loop,
    Map,
    Mod,
    Nand,
    New,
    Next,
    Nor,
    Not,
    Null,
    Of,
    On,
    Open,
    Or,
    Others,
    Out,
    Package,
    Port,
    Postponed,
    Procedural,
    Procedure,
    Process,
    Protected,
    Pure,
    Range,
    Record,
    Register,
    Reject,
    Rem,
    Report,
    Return,
    Rol,
    Ror,
    Select,
    Severity,
    Shared,
    Signal,
    Sla,
    Sll,
    Sra,
    Srl,
    Subtype,
    Then,
    To,
    Transport,
    Type,
    Units,
    Until,
    Use,
    Variable,
    Wait,
    When,
    While,
    With,
    Xnor,
    Xor,
}
