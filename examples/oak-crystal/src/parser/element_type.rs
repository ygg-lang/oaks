//! Crystal element types.

use oak_core::{ElementType, UniversalElementRole};
use std::fmt::{Display, Formatter};

/// Enum representing all possible element types in Crystal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum CrystalElementType {
    /// Whitespace characters.
    Whitespace,
    /// Comments.
    Comment,
    /// Identifiers.
    Identifier,
    /// Numeric literals.
    Number,
    /// String literals.
    String,
    /// Character literals.
    Character,
    /// Symbol literals.
    Symbol,
    /// `class` keyword.
    ClassKeyword,
    /// `module` keyword.
    ModuleKeyword,
    /// `def` keyword.
    DefKeyword,
    /// `end` keyword.
    EndKeyword,
    /// `if` keyword.
    IfKeyword,
    /// `else` keyword.
    ElseKeyword,
    /// `elsif` keyword.
    ElsifKeyword,
    /// `unless` keyword.
    UnlessKeyword,
    /// `case` keyword.
    CaseKeyword,
    /// `when` keyword.
    WhenKeyword,
    /// `then` keyword.
    ThenKeyword,
    /// `while` keyword.
    WhileKeyword,
    /// `until` keyword.
    UntilKeyword,
    /// `for` keyword.
    ForKeyword,
    /// `in` keyword.
    InKeyword,
    /// `do` keyword.
    DoKeyword,
    /// `begin` keyword.
    BeginKeyword,
    /// `rescue` keyword.
    RescueKeyword,
    /// `ensure` keyword.
    EnsureKeyword,
    /// `break` keyword.
    BreakKeyword,
    /// `next` keyword.
    NextKeyword,
    /// `return` keyword.
    ReturnKeyword,
    /// `yield` keyword.
    YieldKeyword,
    /// `super` keyword.
    SuperKeyword,
    /// `self` keyword.
    SelfKeyword,
    /// `true` keyword.
    TrueKeyword,
    /// `false` keyword.
    FalseKeyword,
    /// `nil` keyword.
    NilKeyword,
    /// `and` keyword.
    AndKeyword,
    /// `or` keyword.
    OrKeyword,
    /// `not` keyword.
    NotKeyword,
    /// `+` operator.
    Plus,
    /// `-` operator.
    Minus,
    /// `*` operator.
    Star,
    /// `/` operator.
    Slash,
    /// `%` operator.
    Percent,
    /// `**` operator.
    StarStar,
    /// `=` operator.
    Equal,
    /// `==` operator.
    EqualEqual,
    /// `!=` operator.
    NotEqual,
    /// `<` operator.
    Less,
    /// `<=` operator.
    LessEqual,
    /// `>` operator.
    Greater,
    /// `>=` operator.
    GreaterEqual,
    /// `<=>` operator.
    Spaceship,
    /// `=~` operator.
    Match,
    /// `!~` operator.
    NotMatch,
    /// `&` operator.
    And,
    /// `|` operator.
    Or,
    /// `!` operator.
    Not,
    /// `&` bitwise operator.
    BitwiseAnd,
    /// `|` bitwise operator.
    BitwiseOr,
    /// `^` bitwise operator.
    BitwiseXor,
    /// `~` bitwise operator.
    BitwiseNot,
    /// `<<` operator.
    LeftShift,
    /// `>>` operator.
    RightShift,
    /// `&&` operator.
    LogicalAnd,
    /// `||` operator.
    LogicalOr,
    /// `+=` operator.
    PlusEqual,
    /// `-=` operator.
    MinusEqual,
    /// `*=` operator.
    StarEqual,
    /// `/=` operator.
    SlashEqual,
    /// `%=` operator.
    PercentEqual,
    /// `**=` operator.
    StarStarEqual,
    /// `&=` operator.
    AndEqual,
    /// `|=` operator.
    OrEqual,
    /// `^=` operator.
    XorEqual,
    /// `<<=` operator.
    LeftShiftEqual,
    /// `>>=` operator.
    RightShiftEqual,
    /// `&&=` operator.
    LogicalAndEqual,
    /// `||=` operator.
    LogicalOrEqual,
    /// `(` symbol.
    LeftParen,
    /// `)` symbol.
    RightParen,
    /// `{` symbol.
    LeftBrace,
    /// `}` symbol.
    RightBrace,
    /// `[` symbol.
    LeftBracket,
    /// `]` symbol.
    RightBracket,
    /// `,` symbol.
    Comma,
    /// `;` symbol.
    Semicolon,
    /// `.` symbol.
    Dot,
    /// `..` symbol.
    DotDot,
    /// `...` symbol.
    DotDotDot,
    /// `:` symbol.
    Colon,
    /// `::` symbol.
    DoubleColon,
    /// `->` symbol.
    Arrow,
    /// `=>` symbol.
    FatArrow,
    /// `?` symbol.
    Question,
    /// `@` symbol.
    At,
    /// `@@` symbol.
    DoubleAt,
    /// `$` symbol.
    Dollar,
    /// Newline character.
    Newline,
    /// End of file.
    Eof,
    /// Error element.
    Error,
    /// Root node.
    Root,
    /// Program node.
    Program,
    /// Source file node.
    SourceFile,
    /// Class definition.
    ClassDef,
    /// Module definition.
    ModuleDef,
    /// Method definition.
    MethodDef,
    /// Block node.
    Block,
    /// `if` expression.
    IfExpr,
    /// `unless` expression.
    UnlessExpr,
    /// `case` expression.
    CaseExpr,
    /// `when` clause.
    WhenClause,
    /// `while` expression.
    WhileExpr,
    /// `until` expression.
    UntilExpr,
    /// `for` expression.
    ForExpr,
    /// `begin` expression.
    BeginExpr,
    /// `rescue` clause.
    RescueClause,
    /// `ensure` clause.
    EnsureClause,
    /// Call expression.
    CallExpr,
    /// Index expression.
    IndexExpr,
    /// Member expression.
    MemberExpr,
    /// Binary expression.
    BinaryExpr,
    /// Unary expression.
    UnaryExpr,
    /// Assignment expression.
    AssignExpr,
    /// Literal expression.
    LiteralExpr,
    /// Identifier expression.
    IdentifierExpr,
    /// Array expression.
    ArrayExpr,
    /// Hash expression.
    HashExpr,
    /// Hash pair.
    HashPair,
    /// Block expression.
    BlockExpr,
    /// Lambda expression.
    LambdaExpr,
    /// `yield` expression.
    YieldExpr,
    /// `return` expression.
    ReturnExpr,
    /// `break` expression.
    BreakExpr,
    /// `next` expression.
    NextExpr,
    /// `super` expression.
    SuperExpr,
    /// `self` expression.
    SelfExpr,
    /// Parenthesized expression.
    ParenExpr,
    /// Type expression.
    TypeExpr,
    /// Generic type.
    GenericType,
    /// Union type.
    UnionType,
    /// Tuple type.
    TupleType,
    /// Named tuple type.
    NamedTupleType,
    /// Proc type.
    ProcType,
    /// Pattern node.
    Pattern,
    /// Identifier pattern.
    IdentifierPattern,
    /// Literal pattern.
    LiteralPattern,
    /// Array pattern.
    ArrayPattern,
    /// Hash pattern.
    HashPattern,
    /// Tuple pattern.
    TuplePattern,
    /// Parameter list.
    ParamList,
    /// Parameter node.
    Param,
    /// Splat parameter.
    SplatParam,
    /// Double splat parameter.
    DoubleSplatParam,
    /// Block parameter.
    BlockParam,
    /// Annotation node.
    Annotation,
    /// Macro definition.
    MacroDef,
    /// Macro call.
    MacroCall,
    /// Macro expression.
    MacroExpr,
    /// Alias definition.
    Alias,
    /// `include` statement.
    Include,
    /// `extend` statement.
    Extend,
    /// `require` statement.
    Require,
    /// `private` visibility.
    Private,
    /// `protected` visibility.
    Protected,
    /// `public` visibility.
    Public,
    /// `abstract` modifier.
    Abstract,
    /// `virtual` modifier.
    Virtual,
    /// `override` modifier.
    Override,
    /// Struct definition.
    StructDef,
    /// Enum definition.
    EnumDef,
    /// Union definition.
    UnionDef,
    /// Lib definition.
    LibDef,
    /// `raise` expression.
    RaiseExpr,
    /// Range expression.
    RangeExpr,
    /// Exclusive range expression.
    ExclusiveRangeExpr,
    /// Regex literal.
    RegexLiteral,
    /// String interpolation.
    StringInterpolation,
    /// Interpolation expression.
    InterpolationExpr,
    /// Symbol literal.
    SymbolLiteral,
    /// Constant reference.
    ConstantRef,
    /// Instance variable.
    InstanceVar,
    /// Class variable.
    ClassVar,
    /// Global variable.
    GlobalVar,
    /// Getter method.
    Getter,
    /// Setter method.
    Setter,
    /// Operator definition.
    OperatorDef,
}

impl CrystalElementType {
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

impl Display for CrystalElementType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementType for CrystalElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::CrystalTokenType> for CrystalElementType {
    fn from(token: crate::lexer::token_type::CrystalTokenType) -> Self {
                match token {
            crate::lexer::token_type::CrystalTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::CrystalTokenType::Comment => Self::Comment,
            crate::lexer::token_type::CrystalTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::CrystalTokenType::Number => Self::Number,
            crate::lexer::token_type::CrystalTokenType::String => Self::String,
            crate::lexer::token_type::CrystalTokenType::Character => Self::Character,
            crate::lexer::token_type::CrystalTokenType::Symbol => Self::Symbol,
            crate::lexer::token_type::CrystalTokenType::ClassKeyword => Self::ClassKeyword,
            crate::lexer::token_type::CrystalTokenType::ModuleKeyword => Self::ModuleKeyword,
            crate::lexer::token_type::CrystalTokenType::DefKeyword => Self::DefKeyword,
            crate::lexer::token_type::CrystalTokenType::EndKeyword => Self::EndKeyword,
            crate::lexer::token_type::CrystalTokenType::IfKeyword => Self::IfKeyword,
            crate::lexer::token_type::CrystalTokenType::ElseKeyword => Self::ElseKeyword,
            crate::lexer::token_type::CrystalTokenType::ElsifKeyword => Self::ElsifKeyword,
            crate::lexer::token_type::CrystalTokenType::UnlessKeyword => Self::UnlessKeyword,
            crate::lexer::token_type::CrystalTokenType::CaseKeyword => Self::CaseKeyword,
            crate::lexer::token_type::CrystalTokenType::WhenKeyword => Self::WhenKeyword,
            crate::lexer::token_type::CrystalTokenType::ThenKeyword => Self::ThenKeyword,
            crate::lexer::token_type::CrystalTokenType::WhileKeyword => Self::WhileKeyword,
            crate::lexer::token_type::CrystalTokenType::UntilKeyword => Self::UntilKeyword,
            crate::lexer::token_type::CrystalTokenType::ForKeyword => Self::ForKeyword,
            crate::lexer::token_type::CrystalTokenType::InKeyword => Self::InKeyword,
            crate::lexer::token_type::CrystalTokenType::DoKeyword => Self::DoKeyword,
            crate::lexer::token_type::CrystalTokenType::BeginKeyword => Self::BeginKeyword,
            crate::lexer::token_type::CrystalTokenType::RescueKeyword => Self::RescueKeyword,
            crate::lexer::token_type::CrystalTokenType::EnsureKeyword => Self::EnsureKeyword,
            crate::lexer::token_type::CrystalTokenType::BreakKeyword => Self::BreakKeyword,
            crate::lexer::token_type::CrystalTokenType::NextKeyword => Self::NextKeyword,
            crate::lexer::token_type::CrystalTokenType::ReturnKeyword => Self::ReturnKeyword,
            crate::lexer::token_type::CrystalTokenType::YieldKeyword => Self::YieldKeyword,
            crate::lexer::token_type::CrystalTokenType::SuperKeyword => Self::SuperKeyword,
            crate::lexer::token_type::CrystalTokenType::SelfKeyword => Self::SelfKeyword,
            crate::lexer::token_type::CrystalTokenType::TrueKeyword => Self::TrueKeyword,
            crate::lexer::token_type::CrystalTokenType::FalseKeyword => Self::FalseKeyword,
            crate::lexer::token_type::CrystalTokenType::NilKeyword => Self::NilKeyword,
            crate::lexer::token_type::CrystalTokenType::AndKeyword => Self::AndKeyword,
            crate::lexer::token_type::CrystalTokenType::OrKeyword => Self::OrKeyword,
            crate::lexer::token_type::CrystalTokenType::NotKeyword => Self::NotKeyword,
            crate::lexer::token_type::CrystalTokenType::Plus => Self::Plus,
            crate::lexer::token_type::CrystalTokenType::Minus => Self::Minus,
            crate::lexer::token_type::CrystalTokenType::Star => Self::Star,
            crate::lexer::token_type::CrystalTokenType::Slash => Self::Slash,
            crate::lexer::token_type::CrystalTokenType::Percent => Self::Percent,
            crate::lexer::token_type::CrystalTokenType::StarStar => Self::StarStar,
            crate::lexer::token_type::CrystalTokenType::Equal => Self::Equal,
            crate::lexer::token_type::CrystalTokenType::EqualEqual => Self::EqualEqual,
            crate::lexer::token_type::CrystalTokenType::NotEqual => Self::NotEqual,
            crate::lexer::token_type::CrystalTokenType::Less => Self::Less,
            crate::lexer::token_type::CrystalTokenType::LessEqual => Self::LessEqual,
            crate::lexer::token_type::CrystalTokenType::Greater => Self::Greater,
            crate::lexer::token_type::CrystalTokenType::GreaterEqual => Self::GreaterEqual,
            crate::lexer::token_type::CrystalTokenType::Spaceship => Self::Spaceship,
            crate::lexer::token_type::CrystalTokenType::Match => Self::Match,
            crate::lexer::token_type::CrystalTokenType::NotMatch => Self::NotMatch,
            crate::lexer::token_type::CrystalTokenType::And => Self::And,
            crate::lexer::token_type::CrystalTokenType::Or => Self::Or,
            crate::lexer::token_type::CrystalTokenType::Not => Self::Not,
            crate::lexer::token_type::CrystalTokenType::BitwiseAnd => Self::BitwiseAnd,
            crate::lexer::token_type::CrystalTokenType::BitwiseOr => Self::BitwiseOr,
            crate::lexer::token_type::CrystalTokenType::BitwiseXor => Self::BitwiseXor,
            crate::lexer::token_type::CrystalTokenType::BitwiseNot => Self::BitwiseNot,
            crate::lexer::token_type::CrystalTokenType::LeftShift => Self::LeftShift,
            crate::lexer::token_type::CrystalTokenType::RightShift => Self::RightShift,
            crate::lexer::token_type::CrystalTokenType::LogicalAnd => Self::LogicalAnd,
            crate::lexer::token_type::CrystalTokenType::LogicalOr => Self::LogicalOr,
            crate::lexer::token_type::CrystalTokenType::PlusEqual => Self::PlusEqual,
            crate::lexer::token_type::CrystalTokenType::MinusEqual => Self::MinusEqual,
            crate::lexer::token_type::CrystalTokenType::StarEqual => Self::StarEqual,
            crate::lexer::token_type::CrystalTokenType::SlashEqual => Self::SlashEqual,
            crate::lexer::token_type::CrystalTokenType::PercentEqual => Self::PercentEqual,
            crate::lexer::token_type::CrystalTokenType::StarStarEqual => Self::StarStarEqual,
            crate::lexer::token_type::CrystalTokenType::AndEqual => Self::AndEqual,
            crate::lexer::token_type::CrystalTokenType::OrEqual => Self::OrEqual,
            crate::lexer::token_type::CrystalTokenType::XorEqual => Self::XorEqual,
            crate::lexer::token_type::CrystalTokenType::LeftShiftEqual => Self::LeftShiftEqual,
            crate::lexer::token_type::CrystalTokenType::RightShiftEqual => Self::RightShiftEqual,
            crate::lexer::token_type::CrystalTokenType::LogicalAndEqual => Self::LogicalAndEqual,
            crate::lexer::token_type::CrystalTokenType::LogicalOrEqual => Self::LogicalOrEqual,
            crate::lexer::token_type::CrystalTokenType::LeftParen => Self::LeftParen,
            crate::lexer::token_type::CrystalTokenType::RightParen => Self::RightParen,
            crate::lexer::token_type::CrystalTokenType::LeftBrace => Self::LeftBrace,
            crate::lexer::token_type::CrystalTokenType::RightBrace => Self::RightBrace,
            crate::lexer::token_type::CrystalTokenType::LeftBracket => Self::LeftBracket,
            crate::lexer::token_type::CrystalTokenType::RightBracket => Self::RightBracket,
            crate::lexer::token_type::CrystalTokenType::Comma => Self::Comma,
            crate::lexer::token_type::CrystalTokenType::Semicolon => Self::Semicolon,
            crate::lexer::token_type::CrystalTokenType::Dot => Self::Dot,
            crate::lexer::token_type::CrystalTokenType::DotDot => Self::DotDot,
            crate::lexer::token_type::CrystalTokenType::DotDotDot => Self::DotDotDot,
            crate::lexer::token_type::CrystalTokenType::Colon => Self::Colon,
            crate::lexer::token_type::CrystalTokenType::DoubleColon => Self::DoubleColon,
            crate::lexer::token_type::CrystalTokenType::Arrow => Self::Arrow,
            crate::lexer::token_type::CrystalTokenType::FatArrow => Self::FatArrow,
            crate::lexer::token_type::CrystalTokenType::Question => Self::Question,
            crate::lexer::token_type::CrystalTokenType::At => Self::At,
            crate::lexer::token_type::CrystalTokenType::DoubleAt => Self::DoubleAt,
            crate::lexer::token_type::CrystalTokenType::Dollar => Self::Dollar,
            crate::lexer::token_type::CrystalTokenType::Newline => Self::Newline,
            crate::lexer::token_type::CrystalTokenType::Eof => Self::Eof,
            crate::lexer::token_type::CrystalTokenType::Error => Self::Error,
            crate::lexer::token_type::CrystalTokenType::Root => Self::Root,
            crate::lexer::token_type::CrystalTokenType::Program => Self::Program,
            crate::lexer::token_type::CrystalTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::CrystalTokenType::ClassDef => Self::ClassDef,
            crate::lexer::token_type::CrystalTokenType::ModuleDef => Self::ModuleDef,
            crate::lexer::token_type::CrystalTokenType::MethodDef => Self::MethodDef,
            crate::lexer::token_type::CrystalTokenType::Block => Self::Block,
            crate::lexer::token_type::CrystalTokenType::IfExpr => Self::IfExpr,
            crate::lexer::token_type::CrystalTokenType::UnlessExpr => Self::UnlessExpr,
            crate::lexer::token_type::CrystalTokenType::CaseExpr => Self::CaseExpr,
            crate::lexer::token_type::CrystalTokenType::WhenClause => Self::WhenClause,
            crate::lexer::token_type::CrystalTokenType::WhileExpr => Self::WhileExpr,
            crate::lexer::token_type::CrystalTokenType::UntilExpr => Self::UntilExpr,
            crate::lexer::token_type::CrystalTokenType::ForExpr => Self::ForExpr,
            crate::lexer::token_type::CrystalTokenType::BeginExpr => Self::BeginExpr,
            crate::lexer::token_type::CrystalTokenType::RescueClause => Self::RescueClause,
            crate::lexer::token_type::CrystalTokenType::EnsureClause => Self::EnsureClause,
            crate::lexer::token_type::CrystalTokenType::CallExpr => Self::CallExpr,
            crate::lexer::token_type::CrystalTokenType::IndexExpr => Self::IndexExpr,
            crate::lexer::token_type::CrystalTokenType::MemberExpr => Self::MemberExpr,
            crate::lexer::token_type::CrystalTokenType::BinaryExpr => Self::BinaryExpr,
            crate::lexer::token_type::CrystalTokenType::UnaryExpr => Self::UnaryExpr,
            crate::lexer::token_type::CrystalTokenType::AssignExpr => Self::AssignExpr,
            crate::lexer::token_type::CrystalTokenType::LiteralExpr => Self::LiteralExpr,
            crate::lexer::token_type::CrystalTokenType::IdentifierExpr => Self::IdentifierExpr,
            crate::lexer::token_type::CrystalTokenType::ArrayExpr => Self::ArrayExpr,
            crate::lexer::token_type::CrystalTokenType::HashExpr => Self::HashExpr,
            crate::lexer::token_type::CrystalTokenType::HashPair => Self::HashPair,
            crate::lexer::token_type::CrystalTokenType::BlockExpr => Self::BlockExpr,
            crate::lexer::token_type::CrystalTokenType::LambdaExpr => Self::LambdaExpr,
            crate::lexer::token_type::CrystalTokenType::YieldExpr => Self::YieldExpr,
            crate::lexer::token_type::CrystalTokenType::ReturnExpr => Self::ReturnExpr,
            crate::lexer::token_type::CrystalTokenType::BreakExpr => Self::BreakExpr,
            crate::lexer::token_type::CrystalTokenType::NextExpr => Self::NextExpr,
            crate::lexer::token_type::CrystalTokenType::SuperExpr => Self::SuperExpr,
            crate::lexer::token_type::CrystalTokenType::SelfExpr => Self::SelfExpr,
            crate::lexer::token_type::CrystalTokenType::ParenExpr => Self::ParenExpr,
            crate::lexer::token_type::CrystalTokenType::TypeExpr => Self::TypeExpr,
            crate::lexer::token_type::CrystalTokenType::GenericType => Self::GenericType,
            crate::lexer::token_type::CrystalTokenType::UnionType => Self::UnionType,
            crate::lexer::token_type::CrystalTokenType::TupleType => Self::TupleType,
            crate::lexer::token_type::CrystalTokenType::NamedTupleType => Self::NamedTupleType,
            crate::lexer::token_type::CrystalTokenType::ProcType => Self::ProcType,
            crate::lexer::token_type::CrystalTokenType::Pattern => Self::Pattern,
            crate::lexer::token_type::CrystalTokenType::IdentifierPattern => Self::IdentifierPattern,
            crate::lexer::token_type::CrystalTokenType::LiteralPattern => Self::LiteralPattern,
            crate::lexer::token_type::CrystalTokenType::ArrayPattern => Self::ArrayPattern,
            crate::lexer::token_type::CrystalTokenType::HashPattern => Self::HashPattern,
            crate::lexer::token_type::CrystalTokenType::TuplePattern => Self::TuplePattern,
            crate::lexer::token_type::CrystalTokenType::ParamList => Self::ParamList,
            crate::lexer::token_type::CrystalTokenType::Param => Self::Param,
            crate::lexer::token_type::CrystalTokenType::SplatParam => Self::SplatParam,
            crate::lexer::token_type::CrystalTokenType::DoubleSplatParam => Self::DoubleSplatParam,
            crate::lexer::token_type::CrystalTokenType::BlockParam => Self::BlockParam,
            crate::lexer::token_type::CrystalTokenType::Annotation => Self::Annotation,
            crate::lexer::token_type::CrystalTokenType::MacroDef => Self::MacroDef,
            crate::lexer::token_type::CrystalTokenType::MacroCall => Self::MacroCall,
            crate::lexer::token_type::CrystalTokenType::MacroExpr => Self::MacroExpr,
            crate::lexer::token_type::CrystalTokenType::Alias => Self::Alias,
            crate::lexer::token_type::CrystalTokenType::Include => Self::Include,
            crate::lexer::token_type::CrystalTokenType::Extend => Self::Extend,
            crate::lexer::token_type::CrystalTokenType::Require => Self::Require,
            crate::lexer::token_type::CrystalTokenType::Private => Self::Private,
            crate::lexer::token_type::CrystalTokenType::Protected => Self::Protected,
            crate::lexer::token_type::CrystalTokenType::Public => Self::Public,
            crate::lexer::token_type::CrystalTokenType::Abstract => Self::Abstract,
            crate::lexer::token_type::CrystalTokenType::Virtual => Self::Virtual,
            crate::lexer::token_type::CrystalTokenType::Override => Self::Override,
            crate::lexer::token_type::CrystalTokenType::StructDef => Self::StructDef,
            crate::lexer::token_type::CrystalTokenType::EnumDef => Self::EnumDef,
            crate::lexer::token_type::CrystalTokenType::UnionDef => Self::UnionDef,
            crate::lexer::token_type::CrystalTokenType::LibDef => Self::LibDef,
            crate::lexer::token_type::CrystalTokenType::RaiseExpr => Self::RaiseExpr,
            crate::lexer::token_type::CrystalTokenType::RangeExpr => Self::RangeExpr,
            crate::lexer::token_type::CrystalTokenType::ExclusiveRangeExpr => Self::ExclusiveRangeExpr,
            crate::lexer::token_type::CrystalTokenType::RegexLiteral => Self::RegexLiteral,
            crate::lexer::token_type::CrystalTokenType::StringInterpolation => Self::StringInterpolation,
            crate::lexer::token_type::CrystalTokenType::InterpolationExpr => Self::InterpolationExpr,
            crate::lexer::token_type::CrystalTokenType::SymbolLiteral => Self::SymbolLiteral,
            crate::lexer::token_type::CrystalTokenType::ConstantRef => Self::ConstantRef,
            crate::lexer::token_type::CrystalTokenType::InstanceVar => Self::InstanceVar,
            crate::lexer::token_type::CrystalTokenType::ClassVar => Self::ClassVar,
            crate::lexer::token_type::CrystalTokenType::GlobalVar => Self::GlobalVar,
            crate::lexer::token_type::CrystalTokenType::Getter => Self::Getter,
            crate::lexer::token_type::CrystalTokenType::Setter => Self::Setter,
            crate::lexer::token_type::CrystalTokenType::OperatorDef => Self::OperatorDef,
            _ => Self::Error,
        }
    }
}
