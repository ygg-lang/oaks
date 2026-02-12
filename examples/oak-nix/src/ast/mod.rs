#![doc = include_str!("readme.md")]
use core::range::Range;

/// A Nix expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NixExpr {
    /// A literal value (string, number, boolean, null).
    Literal(NixLiteral),
    /// An identifier.
    Identifier(NixIdentifier),
    /// A list of expressions.
    List(NixList),
    /// An attribute set.
    AttrSet(NixAttrSet),
    /// A let expression.
    Let(NixLet),
    /// An if expression.
    If(NixIf),
    /// A binary operation.
    BinaryOp(NixBinaryOp),
    /// A unary operation.
    UnaryOp(NixUnaryOp),
    /// A function call.
    Apply(NixApply),
    /// A function definition.
    Lambda(NixLambda),
}

/// A literal value in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NixLiteral {
    /// A string literal.
    String(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// A numeric literal.
    Number(f64, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// A boolean literal.
    Boolean(bool, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
    /// A null literal.
    Null(#[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] Range<usize>),
}

/// An identifier in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixIdentifier {
    /// The name of the identifier.
    pub name: String,
    /// The source range of the identifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A list of expressions in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixList {
    /// The elements of the list.
    pub elements: Vec<NixExpr>,
    /// The source range of the list.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An attribute set in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixAttrSet {
    /// The attributes in the set.
    pub attrs: Vec<NixAttr>,
    /// Whether the set is recursive.
    pub recursive: bool,
    /// The source range of the attribute set.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An attribute in an attribute set.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixAttr {
    /// The name of the attribute.
    pub name: String,
    /// The value of the attribute.
    pub value: NixExpr,
    /// The source range of the attribute.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A let expression in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixLet {
    /// The attributes defined in the let block.
    pub attrs: Vec<NixAttr>,
    /// The expression following the 'in' keyword.
    pub body: Box<NixExpr>,
    /// The source range of the let expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An if expression in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixIf {
    /// The condition of the if expression.
    pub condition: Box<NixExpr>,
    /// The then branch.
    pub then_branch: Box<NixExpr>,
    /// The else branch.
    pub else_branch: Box<NixExpr>,
    /// The source range of the if expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A binary operation in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixBinaryOp {
    /// The left operand.
    pub left: Box<NixExpr>,
    /// The operator.
    pub op: String,
    /// The right operand.
    pub right: Box<NixExpr>,
    /// The source range of the binary operation.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A unary operation in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixUnaryOp {
    /// The operator.
    pub op: String,
    /// The operand.
    pub expr: Box<NixExpr>,
    /// The source range of the unary operation.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A function application in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixApply {
    /// The function being applied.
    pub func: Box<NixExpr>,
    /// The argument.
    pub arg: Box<NixExpr>,
    /// The source range of the application.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A function definition in Nix.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixLambda {
    /// The parameter (either a single identifier or a pattern).
    pub param: String,
    /// The body of the function.
    pub body: Box<NixExpr>,
    /// The source range of the lambda.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root of a Nix file.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NixRoot {
    /// The top-level expression.
    pub expr: NixExpr,
}
