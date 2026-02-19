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

    /// The `fof` keyword (first-order formula).
    FofKw,
    /// The `cnf` keyword (conjunctive normal form).
    CnfKw,
    /// The `tff` keyword (typed first-order formula).
    TffKw,
    /// The `thf` keyword (typed higher-order formula).
    ThfKw,
    /// The `tpi` keyword (typed predicate inference).
    TpiKw,
    /// The `include` keyword.
    IncludeKw,
    /// The `axiom` keyword.
    AxiomKw,
    /// The `hypothesis` keyword.
    HypothesisKw,
    /// The `definition` keyword.
    DefinitionKw,
    /// The `assumption` keyword.
    AssumptionKw,
    /// The `lemma` keyword.
    LemmaKw,
    /// The `theorem` keyword.
    TheoremKw,
    /// The `conjecture` keyword.
    ConjectureKw,
    /// The `negated_conjecture` keyword.
    NegatedConjectureKw,
    /// The `plain` keyword.
    PlainKw,
    /// The `type` keyword.
    TypeKw,
    /// The `fi_domain` keyword.
    FiDomainKw,
    /// The `fi_functors` keyword.
    FiFunctorsKw,
    /// The `fi_predicates` keyword.
    FiPredicatesKw,
    /// The `unknown` keyword.
    UnknownKw,
    /// The `!` (forall) quantifier keyword.
    ForallKw,
    /// The `?` (exists) quantifier keyword.
    ExistsKw,
    /// The `&` (and) logical operator keyword.
    AndKw,
    /// The `|` (or) logical operator keyword.
    OrKw,
    /// The `~` (not) logical operator keyword.
    NotKw,
    /// The `=>` (implies) logical operator keyword.
    ImpliesKw,
    /// The `<=>` (if and only if) logical operator keyword.
    IffKw,
    /// The `<~>` (xor) logical operator keyword.
    XorKw,
    /// The `~|` (nor) logical operator keyword.
    NorKw,
    /// The `~&` (nand) logical operator keyword.
    NandKw,
    /// The `$bool` type keyword.
    BoolKw,
    /// The `$i` (individual) type keyword.
    IndividualKw,
    /// The `$int` type keyword.
    IntKw,
    /// The `$real` type keyword.
    RealKw,
    /// The `$rat` type keyword.
    RatKw,
    /// The `$tType` type keyword.
    TTypeKw,
    /// The `$o` type keyword.
    OTypeKw,
    /// The `$iType` type keyword.
    ITypeKw,
    /// Boolean literal (`$true` or `$false`).
    BoolLiteral,

    /// The `==` equality operator.
    DoubleEq,
    /// The `!=` inequality operator.
    NotEq,
    /// The `<=` less-than-or-equal operator.
    LessEq,
    /// The `>=` greater-than-or-equal operator.
    GreaterEq,
    /// The `&&` logical and operator.
    AndAnd,
    /// The `||` logical or operator.
    OrOr,
    /// The `++` increment operator.
    PlusPlus,
    /// The `--` decrement operator.
    MinusMinus,
    /// The `+=` addition assignment operator.
    PlusEq,
    /// The `-=` subtraction assignment operator.
    MinusEq,
    /// The `*=` multiplication assignment operator.
    StarEq,
    /// The `/=` division assignment operator.
    SlashEq,
    /// The `%=` modulo assignment operator.
    PercentEq,
    /// The `<<` left shift operator.
    LeftShift,
    /// The `>>` right shift operator.
    RightShift,
    /// The `->` arrow operator.
    Arrow,

    /// Left parenthesis `(`.
    LeftParen,
    /// Right parenthesis `)`.
    RightParen,
    /// Left bracket `[`.
    LeftBracket,
    /// Right bracket `]`.
    RightBracket,
    /// Left brace `{`.
    LeftBrace,
    /// Right brace `}`.
    RightBrace,
    /// Colon `:`.
    Colon,
    /// Semicolon `;`.
    Semicolon,
    /// Dot `.`.
    Dot,
    /// Comma `,`.
    Comma,
    /// Question mark `?`.
    Question,
    /// Exclamation mark `!`.
    Bang,
    /// At sign `@`.
    At,
    /// Hash `#`.
    Hash,
    /// Dollar sign `$`.
    Dollar,
    /// Percent `%`.
    Percent,
    /// Caret `^`.
    Caret,
    /// Ampersand `&`.
    Ampersand,
    /// Asterisk `*`.
    Star,
    /// Plus `+`.
    Plus,
    /// Minus `-`.
    Minus,
    /// Equals `=`.
    Eq,
    /// Less-than `<`.
    LessThan,
    /// Greater-than `>`.
    GreaterThan,
    /// Slash `/`.
    Slash,
    /// Backslash `\`.
    Backslash,
    /// Pipe `|`.
    Pipe,
    /// Tilde `~`.
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
