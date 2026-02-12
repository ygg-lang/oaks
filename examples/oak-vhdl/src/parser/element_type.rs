use oak_core::{ElementType, UniversalElementRole};

/// Represents the different types of elements in a VHDL source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VhdlElementType {
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

    // Composite Elements
    /// Root node
    Root,
    /// Entity declaration
    EntityDeclaration,
    /// Architecture body
    ArchitectureBody,
    /// Package declaration
    PackageDeclaration,
    /// Package body
    PackageBody,
    /// Configuration declaration
    ConfigurationDeclaration,
    /// Library clause
    LibraryClause,
    /// Use clause
    UseClause,
    /// Generic clause
    GenericClause,
    /// Port clause
    PortClause,
    /// Port declaration
    PortDeclaration,
    /// Signal declaration
    SignalDeclaration,
    /// Variable declaration
    VariableDeclaration,
    /// Constant declaration
    ConstantDeclaration,
    /// Process statement
    ProcessStatement,
    /// Component declaration
    ComponentDeclaration,
    /// Function declaration
    FunctionDeclaration,
    /// Procedure declaration
    ProcedureDeclaration,
    /// Type declaration
    TypeDeclaration,
    /// Subtype declaration
    SubtypeDeclaration,
    /// If statement
    IfStatement,
    /// Case statement
    CaseStatement,
    /// Loop statement
    LoopStatement,
    /// Assignment statement
    AssignmentStatement,
    /// Wait statement
    WaitStatement,
    /// Assert statement
    AssertStatement,
    /// Report statement
    ReportStatement,
    /// Attribute declaration
    AttributeDeclaration,
    /// Alias declaration
    AliasDeclaration,
    /// File declaration
    FileDeclaration,
    /// Access declaration
    AccessDeclaration,
    /// Generate statement
    GenerateStatement,
    /// Block statement
    BlockStatement,
    /// Component instantiation
    ComponentInstantiation,
}

impl ElementType for VhdlElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::VhdlTokenType> for VhdlElementType {
    fn from(token: crate::lexer::token_type::VhdlTokenType) -> Self {
        match token {
            crate::lexer::token_type::VhdlTokenType::Error => Self::Error,
            crate::lexer::token_type::VhdlTokenType::Eof => Self::Eof,
            crate::lexer::token_type::VhdlTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::VhdlTokenType::Comment => Self::Comment,
            crate::lexer::token_type::VhdlTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::VhdlTokenType::StringLiteral => Self::StringLiteral,
            crate::lexer::token_type::VhdlTokenType::CharLiteral => Self::CharLiteral,
            crate::lexer::token_type::VhdlTokenType::BitStringLiteral => Self::BitStringLiteral,
            crate::lexer::token_type::VhdlTokenType::BasedLiteral => Self::BasedLiteral,
            crate::lexer::token_type::VhdlTokenType::RealLiteral => Self::RealLiteral,
            crate::lexer::token_type::VhdlTokenType::IntegerLiteral => Self::IntegerLiteral,
            crate::lexer::token_type::VhdlTokenType::EntityKw => Self::EntityKw,
            crate::lexer::token_type::VhdlTokenType::ArchitectureKw => Self::ArchitectureKw,
            crate::lexer::token_type::VhdlTokenType::BeginKw => Self::BeginKw,
            crate::lexer::token_type::VhdlTokenType::EndKw => Self::EndKw,
            crate::lexer::token_type::VhdlTokenType::ProcessKw => Self::ProcessKw,
            crate::lexer::token_type::VhdlTokenType::SignalKw => Self::SignalKw,
            crate::lexer::token_type::VhdlTokenType::VariableKw => Self::VariableKw,
            crate::lexer::token_type::VhdlTokenType::ConstantKw => Self::ConstantKw,
            crate::lexer::token_type::VhdlTokenType::ComponentKw => Self::ComponentKw,
            crate::lexer::token_type::VhdlTokenType::PortKw => Self::PortKw,
            crate::lexer::token_type::VhdlTokenType::MapKw => Self::MapKw,
            crate::lexer::token_type::VhdlTokenType::GenericKw => Self::GenericKw,
            crate::lexer::token_type::VhdlTokenType::LibraryKw => Self::LibraryKw,
            crate::lexer::token_type::VhdlTokenType::UseKw => Self::UseKw,
            crate::lexer::token_type::VhdlTokenType::PackageKw => Self::PackageKw,
            crate::lexer::token_type::VhdlTokenType::BodyKw => Self::BodyKw,
            crate::lexer::token_type::VhdlTokenType::FunctionKw => Self::FunctionKw,
            crate::lexer::token_type::VhdlTokenType::ProcedureKw => Self::ProcedureKw,
            crate::lexer::token_type::VhdlTokenType::TypeKw => Self::TypeKw,
            crate::lexer::token_type::VhdlTokenType::SubtypeKw => Self::SubtypeKw,
            crate::lexer::token_type::VhdlTokenType::RecordKw => Self::RecordKw,
            crate::lexer::token_type::VhdlTokenType::ArrayKw => Self::ArrayKw,
            crate::lexer::token_type::VhdlTokenType::IfKw => Self::IfKw,
            crate::lexer::token_type::VhdlTokenType::ThenKw => Self::ThenKw,
            crate::lexer::token_type::VhdlTokenType::ElseKw => Self::ElseKw,
            crate::lexer::token_type::VhdlTokenType::ElsifKw => Self::ElsifKw,
            crate::lexer::token_type::VhdlTokenType::CaseKw => Self::CaseKw,
            crate::lexer::token_type::VhdlTokenType::WhenKw => Self::WhenKw,
            crate::lexer::token_type::VhdlTokenType::LoopKw => Self::LoopKw,
            crate::lexer::token_type::VhdlTokenType::ForKw => Self::ForKw,
            crate::lexer::token_type::VhdlTokenType::WhileKw => Self::WhileKw,
            crate::lexer::token_type::VhdlTokenType::ExitKw => Self::ExitKw,
            crate::lexer::token_type::VhdlTokenType::NextKw => Self::NextKw,
            crate::lexer::token_type::VhdlTokenType::ReturnKw => Self::ReturnKw,
            crate::lexer::token_type::VhdlTokenType::WaitKw => Self::WaitKw,
            crate::lexer::token_type::VhdlTokenType::UntilKw => Self::UntilKw,
            crate::lexer::token_type::VhdlTokenType::InKw => Self::InKw,
            crate::lexer::token_type::VhdlTokenType::OutKw => Self::OutKw,
            crate::lexer::token_type::VhdlTokenType::InoutKw => Self::InoutKw,
            crate::lexer::token_type::VhdlTokenType::BufferKw => Self::BufferKw,
            crate::lexer::token_type::VhdlTokenType::LinkageKw => Self::LinkageKw,
            crate::lexer::token_type::VhdlTokenType::DowntoKw => Self::DowntoKw,
            crate::lexer::token_type::VhdlTokenType::ToKw => Self::ToKw,
            crate::lexer::token_type::VhdlTokenType::GenerateKw => Self::GenerateKw,
            crate::lexer::token_type::VhdlTokenType::WithKw => Self::WithKw,
            crate::lexer::token_type::VhdlTokenType::SelectKw => Self::SelectKw,
            crate::lexer::token_type::VhdlTokenType::AllKw => Self::AllKw,
            crate::lexer::token_type::VhdlTokenType::OthersKw => Self::OthersKw,
            crate::lexer::token_type::VhdlTokenType::NullKw => Self::NullKw,
            crate::lexer::token_type::VhdlTokenType::OpenKw => Self::OpenKw,
            crate::lexer::token_type::VhdlTokenType::IsKw => Self::IsKw,
            crate::lexer::token_type::VhdlTokenType::OfKw => Self::OfKw,
            crate::lexer::token_type::VhdlTokenType::RangeKw => Self::RangeKw,
            crate::lexer::token_type::VhdlTokenType::ReverseRangeKw => Self::ReverseRangeKw,
            crate::lexer::token_type::VhdlTokenType::AttributeKw => Self::AttributeKw,
            crate::lexer::token_type::VhdlTokenType::AliasKw => Self::AliasKw,
            crate::lexer::token_type::VhdlTokenType::FileKw => Self::FileKw,
            crate::lexer::token_type::VhdlTokenType::AccessKw => Self::AccessKw,
            crate::lexer::token_type::VhdlTokenType::AfterKw => Self::AfterKw,
            crate::lexer::token_type::VhdlTokenType::AssertKw => Self::AssertKw,
            crate::lexer::token_type::VhdlTokenType::ReportKw => Self::ReportKw,
            crate::lexer::token_type::VhdlTokenType::SeverityKw => Self::SeverityKw,
            crate::lexer::token_type::VhdlTokenType::BitKw => Self::BitKw,
            crate::lexer::token_type::VhdlTokenType::BitVectorKw => Self::BitVectorKw,
            crate::lexer::token_type::VhdlTokenType::BooleanKw => Self::BooleanKw,
            crate::lexer::token_type::VhdlTokenType::CharacterKw => Self::CharacterKw,
            crate::lexer::token_type::VhdlTokenType::IntegerKw => Self::IntegerKw,
            crate::lexer::token_type::VhdlTokenType::NaturalKw => Self::NaturalKw,
            crate::lexer::token_type::VhdlTokenType::PositiveKw => Self::PositiveKw,
            crate::lexer::token_type::VhdlTokenType::RealKw => Self::RealKw,
            crate::lexer::token_type::VhdlTokenType::StringKw => Self::StringKw,
            crate::lexer::token_type::VhdlTokenType::TimeKw => Self::TimeKw,
            crate::lexer::token_type::VhdlTokenType::StdLogicKw => Self::StdLogicKw,
            crate::lexer::token_type::VhdlTokenType::StdLogicVectorKw => Self::StdLogicVectorKw,
            crate::lexer::token_type::VhdlTokenType::UnsignedKw => Self::UnsignedKw,
            crate::lexer::token_type::VhdlTokenType::SignedKw => Self::SignedKw,
            crate::lexer::token_type::VhdlTokenType::And => Self::And,
            crate::lexer::token_type::VhdlTokenType::Or => Self::Or,
            crate::lexer::token_type::VhdlTokenType::Nand => Self::Nand,
            crate::lexer::token_type::VhdlTokenType::Nor => Self::Nor,
            crate::lexer::token_type::VhdlTokenType::Xor => Self::Xor,
            crate::lexer::token_type::VhdlTokenType::Xnor => Self::Xnor,
            crate::lexer::token_type::VhdlTokenType::Not => Self::Not,
            crate::lexer::token_type::VhdlTokenType::Sll => Self::Sll,
            crate::lexer::token_type::VhdlTokenType::Srl => Self::Srl,
            crate::lexer::token_type::VhdlTokenType::Sla => Self::Sla,
            crate::lexer::token_type::VhdlTokenType::Sra => Self::Sra,
            crate::lexer::token_type::VhdlTokenType::Rol => Self::Rol,
            crate::lexer::token_type::VhdlTokenType::Ror => Self::Ror,
            crate::lexer::token_type::VhdlTokenType::Mod => Self::Mod,
            crate::lexer::token_type::VhdlTokenType::Rem => Self::Rem,
            crate::lexer::token_type::VhdlTokenType::Abs => Self::Abs,
            crate::lexer::token_type::VhdlTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::VhdlTokenType::Eq => Self::Eq,
            crate::lexer::token_type::VhdlTokenType::Ne => Self::Ne,
            crate::lexer::token_type::VhdlTokenType::Slash => Self::Slash,
            crate::lexer::token_type::VhdlTokenType::Le => Self::Le,
            crate::lexer::token_type::VhdlTokenType::Lt => Self::Lt,
            crate::lexer::token_type::VhdlTokenType::Ge => Self::Ge,
            crate::lexer::token_type::VhdlTokenType::Gt => Self::Gt,
            crate::lexer::token_type::VhdlTokenType::ShiftLeft => Self::ShiftLeft,
            crate::lexer::token_type::VhdlTokenType::ShiftRight => Self::ShiftRight,
            crate::lexer::token_type::VhdlTokenType::Plus => Self::Plus,
            crate::lexer::token_type::VhdlTokenType::Minus => Self::Minus,
            crate::lexer::token_type::VhdlTokenType::Star => Self::Star,
            crate::lexer::token_type::VhdlTokenType::Pow => Self::Pow,
            crate::lexer::token_type::VhdlTokenType::Assign => Self::Assign,
            crate::lexer::token_type::VhdlTokenType::Colon => Self::Colon,
            crate::lexer::token_type::VhdlTokenType::Ampersand => Self::Ampersand,
            crate::lexer::token_type::VhdlTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::VhdlTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::VhdlTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::VhdlTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::VhdlTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::VhdlTokenType::Comma => Self::Comma,
            crate::lexer::token_type::VhdlTokenType::Dot => Self::Dot,
            crate::lexer::token_type::VhdlTokenType::Pipe => Self::Pipe,
            crate::lexer::token_type::VhdlTokenType::Hash => Self::Hash,
            crate::lexer::token_type::VhdlTokenType::At => Self::At,
            crate::lexer::token_type::VhdlTokenType::Question => Self::Question,
            crate::lexer::token_type::VhdlTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::VhdlTokenType::Percent => Self::Percent,
            crate::lexer::token_type::VhdlTokenType::Caret => Self::Caret,
            crate::lexer::token_type::VhdlTokenType::Tilde => Self::Tilde,
            crate::lexer::token_type::VhdlTokenType::Backslash => Self::Backslash,
            crate::lexer::token_type::VhdlTokenType::Exclamation => Self::Exclamation,
        }
    }
}
