use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

/// Expression node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprNode {
    /// Literal.
    Literal(LiteralNode),
    /// Name.
    Name(String),
    /// Array element.
    ArrayElement(ArrayElementNode),
    /// Function reference.
    FunctionReference(FunctionReferenceNode),
    /// Unary operation.
    UnaryOp(UnaryOpNode),
    /// Binary operation.
    BinaryOp(BinaryOpNode),
    /// Parenthesized expression.
    ParenExpr(Box<ExprNode>),
    /// Structure constructor.
    StructureConstructor(StructureConstructorNode),
}

/// Literal node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiteralNode {
    /// The literal value as a string.
    pub value: String,
    /// The kind of literal (integer, real, complex, character, logical).
    pub kind: LiteralKind,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Literal kind.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiteralKind {
    /// Integer.
    Integer,
    /// Real.
    Real,
    /// Complex.
    Complex,
    /// Character.
    Character,
    /// Logical.
    Logical,
}

/// Array element node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayElementNode {
    /// The name of the array.
    pub name: String,
    /// The subscript expressions.
    pub subscripts: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function reference node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionReferenceNode {
    /// The name of the function being referenced.
    pub name: String,
    /// The arguments passed to the function.
    pub arguments: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operation node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryOpNode {
    /// The unary operator.
    pub operator: UnaryOperator,
    /// The operand expression.
    pub operand: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operator.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperator {
    /// Not.
    Not,
    /// Plus.
    Plus,
    /// Minus.
    Minus,
}

/// Binary operation node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryOpNode {
    /// The binary operator.
    pub operator: BinaryOperator,
    /// The left-hand side expression.
    pub left: Box<ExprNode>,
    /// The right-hand side expression.
    pub right: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Binary operator.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperator {
    /// Add.
    Add,
    /// Subtract.
    Subtract,
    /// Multiply.
    Multiply,
    /// Divide.
    Divide,
    /// Power.
    Power,
    /// Concat.
    Concat,
    /// Equal.
    Equal,
    /// Not equal.
    NotEqual,
    /// Less than.
    LessThan,
    /// Less than or equal.
    LessThanOrEqual,
    /// Greater than.
    GreaterThan,
    /// Greater than or equal.
    GreaterThanOrEqual,
    /// And.
    And,
    /// Or.
    Or,
    /// Eqv.
    Eqv,
    /// Neqv.
    Neqv,
}

/// Structure constructor node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureConstructorNode {
    /// The name of the derived type being constructed.
    pub type_name: String,
    /// The component initializers as (name, value) pairs.
    pub args: Vec<(Option<String>, Box<ExprNode>)>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
