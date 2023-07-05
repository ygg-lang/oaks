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
        unsafe { std::mem::transmute(token) }
    }
}
