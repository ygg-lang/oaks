#![doc = include_str!("readme.md")]

use core::range::Range;

/// The root node of a Rhombus program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RhombusRoot {
    /// Expressions in the program
    pub expressions: Vec<Expression>,
}

/// Rhombus expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// List expression
    List(Vec<Expression>),
    /// Identifier
    Identifier(String),
    /// Number literal
    NumberLiteral(String),
    /// String literal
    StringLiteral(String),
    /// Character literal
    CharacterLiteral(char),
    /// Boolean literal
    BooleanLiteral(bool),
    /// Symbol
    Symbol(String),
    /// Keyword
    Keyword(String),
    /// Quotation
    Quotation(Box<Expression>),
    /// Define expression
    Define(Define),
    /// Lambda expression
    Lambda(Lambda),
    /// If expression
    If(If),
    /// Cond expression
    Cond(Vec<CondClause>),
    /// Case expression
    Case(Case),
    /// Let expression
    Let(Let),
    /// Let* expression
    LetStar(Let),
    /// Letrec expression
    Letrec(Let),
    /// Begin expression
    Begin(Vec<Expression>),
    /// Do expression
    Do(Do),
    /// Set! expression
    Set(Set),
    /// Require expression
    Require(Require),
    /// Provide expression
    Provide(Provide),
    /// And expression
    And(Vec<Expression>),
    /// Or expression
    Or(Vec<Expression>),
    /// Not expression
    Not(Box<Expression>),
    /// Error expression
    Error(String),
}

/// Define expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Define {
    /// Name
    pub name: String,
    /// Parameters (for functions)
    pub parameters: Vec<String>,
    /// Body
    pub body: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Lambda expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Lambda {
    /// Parameters
    pub parameters: Vec<String>,
    /// Body
    pub body: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct If {
    /// Condition
    pub condition: Box<Expression>,
    /// Then branch
    pub then_branch: Box<Expression>,
    /// Else branch
    pub else_branch: Option<Box<Expression>>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Cond clause
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CondClause {
    /// Condition
    pub condition: Expression,
    /// Body
    pub body: Vec<Expression>,
}

/// Case expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Case {
    /// Expression to match
    pub expression: Box<Expression>,
    /// Cases
    pub cases: Vec<(Vec<Expression>, Vec<Expression>)>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Let expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Let {
    /// Bindings
    pub bindings: Vec<(String, Expression)>,
    /// Body
    pub body: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Do {
    /// Bindings
    pub bindings: Vec<(String, Expression, Expression)>,
    /// Conditions
    pub conditions: Vec<(Expression, Vec<Expression>)>,
    /// Body
    pub body: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Set! expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set {
    /// Name
    pub name: String,
    /// Value
    pub value: Box<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Require expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Require {
    /// Modules or identifiers to require
    pub requirements: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Provide expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Provide {
    /// Identifiers to provide
    pub exports: Vec<Expression>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
