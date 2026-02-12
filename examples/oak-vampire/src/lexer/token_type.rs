use oak_core::TokenType;

/// Vampire token types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VampireTokenType {
    /// End of file.
    Eof,
    /// Whitespace.
    Whitespace,
    /// Line comment.
    LineComment,
    /// Block comment.
    BlockComment,
    /// String literal.
    StringLiteral,
    /// Integer literal.
    IntegerLiteral,
    /// Real literal.
    RealLiteral,
    /// Identifier.
    Identifier,

    // Keywords
    FofKw,
    CnfKw,
    TffKw,
    ThfKw,
    TpiKw,
    IncludeKw,
    AxiomKw,
    HypothesisKw,
    DefinitionKw,
    AssumptionKw,
    LemmaKw,
    TheoremKw,
    ConjectureKw,
    NegatedConjectureKw,
    PlainKw,
    TypeKw,
    FiDomainKw,
    FiFunctorsKw,
    FiPredicatesKw,
    UnknownKw,
    ForallKw,
    ExistsKw,
    AndKw,
    OrKw,
    NotKw,
    ImpliesKw,
    IffKw,
    XorKw,
    NorKw,
    NandKw,
    BoolKw,
    IndividualKw,
    IntKw,
    RealKw,
    RatKw,
    TTypeKw,
    OTypeKw,
    ITypeKw,
    BoolLiteral,

    // Operators
    DoubleEq,
    NotEq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    PlusPlus,
    MinusMinus,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    LeftShift,
    RightShift,
    Arrow,

    // Single char tokens
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Question,
    Bang,
    At,
    Hash,
    Dollar,
    Percent,
    Caret,
    Ampersand,
    Star,
    Plus,
    Minus,
    Eq,
    LessThan,
    GreaterThan,
    Slash,
    Backslash,
    Pipe,
    Tilde,
}

impl TokenType for VampireTokenType {
    const END_OF_STREAM: Self = VampireTokenType::Eof;
    type Role = oak_core::UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            VampireTokenType::Eof => oak_core::UniversalTokenRole::Eof,
            VampireTokenType::Whitespace => oak_core::UniversalTokenRole::Whitespace,
            VampireTokenType::LineComment | VampireTokenType::BlockComment => oak_core::UniversalTokenRole::Comment,
            VampireTokenType::StringLiteral => oak_core::UniversalTokenRole::Literal,
            VampireTokenType::IntegerLiteral | VampireTokenType::RealLiteral => oak_core::UniversalTokenRole::Literal,
            VampireTokenType::Identifier => oak_core::UniversalTokenRole::Name,

            VampireTokenType::FofKw
            | VampireTokenType::CnfKw
            | VampireTokenType::TffKw
            | VampireTokenType::ThfKw
            | VampireTokenType::TpiKw
            | VampireTokenType::IncludeKw
            | VampireTokenType::AxiomKw
            | VampireTokenType::HypothesisKw
            | VampireTokenType::DefinitionKw
            | VampireTokenType::AssumptionKw
            | VampireTokenType::LemmaKw
            | VampireTokenType::TheoremKw
            | VampireTokenType::ConjectureKw
            | VampireTokenType::NegatedConjectureKw
            | VampireTokenType::PlainKw
            | VampireTokenType::TypeKw
            | VampireTokenType::FiDomainKw
            | VampireTokenType::FiFunctorsKw
            | VampireTokenType::FiPredicatesKw
            | VampireTokenType::UnknownKw
            | VampireTokenType::ForallKw
            | VampireTokenType::ExistsKw
            | VampireTokenType::AndKw
            | VampireTokenType::OrKw
            | VampireTokenType::NotKw
            | VampireTokenType::ImpliesKw
            | VampireTokenType::IffKw
            | VampireTokenType::XorKw
            | VampireTokenType::NorKw
            | VampireTokenType::NandKw
            | VampireTokenType::BoolKw
            | VampireTokenType::IndividualKw
            | VampireTokenType::IntKw
            | VampireTokenType::RealKw
            | VampireTokenType::RatKw
            | VampireTokenType::TTypeKw
            | VampireTokenType::OTypeKw
            | VampireTokenType::ITypeKw
            | VampireTokenType::BoolLiteral => oak_core::UniversalTokenRole::Keyword,

            _ => oak_core::UniversalTokenRole::Operator,
        }
    }
}

impl From<VampireTokenType> for u16 {
    fn from(t: VampireTokenType) -> u16 {
        t as u16
    }
}

impl From<u16> for VampireTokenType {
    fn from(i: u16) -> Self {
        unsafe { std::mem::transmute(i as u8) }
    }
}
