use oak_core::{Token, TokenType, UniversalTokenRole};

/// A VHDL token.
pub type VhdlToken = Token<VhdlTokenType>;

/// Represents the different types of tokens in a VHDL source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VhdlTokenType {
    /// Error token
    Error,
    /// End of file token
    Eof,
    /// Whitespace token
    Whitespace,
    /// Comment token
    Comment,
    /// Identifier token
    Identifier,

    // Literals
    /// String literal token
    StringLiteral,
    /// Character literal token
    CharLiteral,
    /// Bit string literal token
    BitStringLiteral,
    /// Based literal token
    BasedLiteral,
    /// Real literal token
    RealLiteral,
    /// Integer literal token
    IntegerLiteral,

    // Keywords
    /// `entity` keyword
    EntityKw,
    /// `architecture` keyword
    ArchitectureKw,
    /// `begin` keyword
    BeginKw,
    /// `end` keyword
    EndKw,
    /// `process` keyword
    ProcessKw,
    /// `signal` keyword
    SignalKw,
    /// `variable` keyword
    VariableKw,
    /// `constant` keyword
    ConstantKw,
    /// `component` keyword
    ComponentKw,
    /// `port` keyword
    PortKw,
    /// `map` keyword
    MapKw,
    /// `generic` keyword
    GenericKw,
    /// `library` keyword
    LibraryKw,
    /// `use` keyword
    UseKw,
    /// `package` keyword
    PackageKw,
    /// `body` keyword
    BodyKw,
    /// `function` keyword
    FunctionKw,
    /// `procedure` keyword
    ProcedureKw,
    /// `type` keyword
    TypeKw,
    /// `subtype` keyword
    SubtypeKw,
    /// `record` keyword
    RecordKw,
    /// `array` keyword
    ArrayKw,
    /// `if` keyword
    IfKw,
    /// `then` keyword
    ThenKw,
    /// `else` keyword
    ElseKw,
    /// `elsif` keyword
    ElsifKw,
    /// `case` keyword
    CaseKw,
    /// `when` keyword
    WhenKw,
    /// `loop` keyword
    LoopKw,
    /// `for` keyword
    ForKw,
    /// `while` keyword
    WhileKw,
    /// `exit` keyword
    ExitKw,
    /// `next` keyword
    NextKw,
    /// `return` keyword
    ReturnKw,
    /// `wait` keyword
    WaitKw,
    /// `until` keyword
    UntilKw,
    /// `in` keyword
    InKw,
    /// `out` keyword
    OutKw,
    /// `inout` keyword
    InoutKw,
    /// `buffer` keyword
    BufferKw,
    /// `linkage` keyword
    LinkageKw,
    /// `downto` keyword
    DowntoKw,
    /// `to` keyword
    ToKw,
    /// `generate` keyword
    GenerateKw,
    /// `with` keyword
    WithKw,
    /// `select` keyword
    SelectKw,
    /// `all` keyword
    AllKw,
    /// `others` keyword
    OthersKw,
    /// `null` keyword
    NullKw,
    /// `open` keyword
    OpenKw,
    /// `is` keyword
    IsKw,
    /// `of` keyword
    OfKw,
    /// `range` keyword
    RangeKw,
    /// `reverse_range` keyword
    ReverseRangeKw,
    /// `attribute` keyword
    AttributeKw,
    /// `alias` keyword
    AliasKw,
    /// `file` keyword
    FileKw,
    /// `access` keyword
    AccessKw,
    /// `after` keyword
    AfterKw,
    /// `assert` keyword
    AssertKw,
    /// `report` keyword
    ReportKw,
    /// `severity` keyword
    SeverityKw,

    // Basic Types (Keywords)
    /// `bit` keyword
    BitKw,
    /// `bit_vector` keyword
    BitVectorKw,
    /// `boolean` keyword
    BooleanKw,
    /// `character` keyword
    CharacterKw,
    /// `integer` keyword
    IntegerKw,
    /// `natural` keyword
    NaturalKw,
    /// `positive` keyword
    PositiveKw,
    /// `real` keyword
    RealKw,
    /// `string` keyword
    StringKw,
    /// `time` keyword
    TimeKw,
    /// `std_logic` keyword
    StdLogicKw,
    /// `std_logic_vector` keyword
    StdLogicVectorKw,
    /// `unsigned` keyword
    UnsignedKw,
    /// `signed` keyword
    SignedKw,

    // Logical Operators (Keywords)
    /// `and` operator
    And,
    /// `or` operator
    Or,
    /// `nand` operator
    Nand,
    /// `nor` operator
    Nor,
    /// `xor` operator
    Xor,
    /// `xnor` operator
    Xnor,
    /// `not` operator
    Not,
    /// `sll` operator
    Sll,
    /// `srl` operator
    Srl,
    /// `sla` operator
    Sla,
    /// `sra` operator
    Sra,
    /// `rol` operator
    Rol,
    /// `ror` operator
    Ror,
    /// `mod` operator
    Mod,
    /// `rem` operator
    Rem,
    /// `abs` operator
    Abs,

    // Operators
    /// `=>` arrow
    Arrow,
    /// `=` equals
    Eq,
    /// `/=` not equals
    Ne,
    /// `/` slash
    Slash,
    /// `<=` less than or equal
    Le,
    /// `<` less than
    Lt,
    /// `>=` greater than or equal
    Ge,
    /// `>` greater than
    Gt,
    /// `<<` shift left
    ShiftLeft,
    /// `>>` shift right
    ShiftRight,
    /// `+` plus
    Plus,
    /// `-` minus
    Minus,
    /// `*` star
    Star,
    /// `**` power
    Pow,
    /// `:=` assignment
    Assign,
    /// `:` colon
    Colon,
    /// `&` ampersand
    Ampersand,

    // Punctuation
    /// `(` left parenthesis
    LeftParen,
    /// `)` right parenthesis
    RightParen,
    /// `[` left bracket
    LeftBracket,
    /// `]` right bracket
    RightBracket,
    /// `;` semicolon
    Semicolon,
    /// `,` comma
    Comma,
    /// `.` dot
    Dot,
    /// `|` pipe
    Pipe,
    /// `#` hash
    Hash,
    /// `@` at
    At,
    /// `?` question mark
    Question,
    /// `$` dollar sign
    Dollar,
    /// `%` percent sign
    Percent,
    /// `^` caret
    Caret,
    /// `~` tilde
    Tilde,
    /// `\` backslash
    Backslash,
    /// `!` exclamation mark
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

/// Represents the different keywords in the VHDL language.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VhdlKeyword {
    /// `abs` keyword
    Abs,
    /// `access` keyword
    Access,
    /// `after` keyword
    After,
    /// `alias` keyword
    Alias,
    /// `all` keyword
    All,
    /// `and` keyword
    And,
    /// `architecture` keyword
    Architecture,
    /// `array` keyword
    Array,
    /// `assert` keyword
    Assert,
    /// `attribute` keyword
    Attribute,
    /// `begin` keyword
    Begin,
    /// `block` keyword
    Block,
    /// `body` keyword
    Body,
    /// `buffer` keyword
    Buffer,
    /// `bus` keyword
    Bus,
    /// `case` keyword
    Case,
    /// `component` keyword
    Component,
    /// `configuration` keyword
    Configuration,
    /// `constant` keyword
    Constant,
    /// `disconnect` keyword
    Disconnect,
    /// `downto` keyword
    Downto,
    /// `else` keyword
    Else,
    /// `elsif` keyword
    Elsif,
    /// `end` keyword
    End,
    /// `entity` keyword
    Entity,
    /// `exit` keyword
    Exit,
    /// `file` keyword
    File,
    /// `for` keyword
    For,
    /// `function` keyword
    Function,
    /// `generate` keyword
    Generate,
    /// `generic` keyword
    Generic,
    /// `group` keyword
    Group,
    /// `guarded` keyword
    Guarded,
    /// `if` keyword
    If,
    /// `impure` keyword
    Impure,
    /// `in` keyword
    In,
    /// `inout` keyword
    Inout,
    /// `is` keyword
    Is,
    /// `label` keyword
    Label,
    /// `library` keyword
    Library,
    /// `linkage` keyword
    Linkage,
    /// `literal` keyword
    Literal,
    /// `loop` keyword
    Loop,
    /// `map` keyword
    Map,
    /// `mod` keyword
    Mod,
    /// `nand` keyword
    Nand,
    /// `new` keyword
    New,
    /// `next` keyword
    Next,
    /// `nor` keyword
    Nor,
    /// `not` keyword
    Not,
    /// `null` keyword
    Null,
    /// `of` keyword
    Of,
    /// `on` keyword
    On,
    /// `open` keyword
    Open,
    /// `or` keyword
    Or,
    /// `others` keyword
    Others,
    /// `out` keyword
    Out,
    /// `package` keyword
    Package,
    /// `port` keyword
    Port,
    /// `postponed` keyword
    Postponed,
    /// `procedural` keyword
    Procedural,
    /// `procedure` keyword
    Procedure,
    /// `process` keyword
    Process,
    /// `protected` keyword
    Protected,
    /// `pure` keyword
    Pure,
    /// `range` keyword
    Range,
    /// `record` keyword
    Record,
    /// `register` keyword
    Register,
    /// `reject` keyword
    Reject,
    /// `rem` keyword
    Rem,
    /// `report` keyword
    Report,
    /// `return` keyword
    Return,
    /// `rol` keyword
    Rol,
    /// `ror` keyword
    Ror,
    /// `select` keyword
    Select,
    /// `severity` keyword
    Severity,
    /// `shared` keyword
    Shared,
    /// `signal` keyword
    Signal,
    /// `sla` keyword
    Sla,
    /// `sll` keyword
    Sll,
    /// `sra` keyword
    Sra,
    /// `srl` keyword
    Srl,
    /// `subtype` keyword
    Subtype,
    /// `then` keyword
    Then,
    /// `to` keyword
    To,
    /// `transport` keyword
    Transport,
    /// `type` keyword
    Type,
    /// `units` keyword
    Units,
    /// `until` keyword
    Until,
    /// `use` keyword
    Use,
    /// `variable` keyword
    Variable,
    /// `wait` keyword
    Wait,
    /// `when` keyword
    When,
    /// `while` keyword
    While,
    /// `with` keyword
    With,
    /// `xnor` keyword
    Xnor,
    /// `xor` keyword
    Xor,
}
