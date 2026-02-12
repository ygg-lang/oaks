#![doc = include_str!("readme.md")]

/// Root node of the Julia AST.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaRoot {
    /// The statements in the root.
    pub statements: Vec<JuliaStatement>,
}

/// A Julia statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStatement {
    /// A function definition.
    Function(JuliaFunction),
    /// An if statement.
    If {
        /// The condition expression.
        condition: JuliaExpression,
        /// The then branch body.
        then_body: Vec<JuliaStatement>,
        /// The optional else branch body.
        else_body: Option<Vec<JuliaStatement>>,
    },
    /// A for loop.
    For {
        /// The loop variable name.
        variable: String,
        /// The iterable expression.
        iterable: JuliaExpression,
        /// The loop body.
        body: Vec<JuliaStatement>,
    },
    /// An expression statement.
    Expression(JuliaExpression),
    /// An error statement.
    Error,
}

/// A Julia function definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaFunction {
    /// The function name.
    pub name: String,
    /// The function body.
    pub body: Vec<JuliaStatement>,
}

/// A Julia expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaExpression {
    /// An identifier.
    Identifier(String),
    /// A literal value.
    Literal(String),
    /// A binary operation.
    Binary {
        /// The left operand.
        left: Box<JuliaExpression>,
        /// The operator.
        op: String,
        /// The right operand.
        right: Box<JuliaExpression>,
    },
    /// A function call.
    Call {
        /// The callee expression.
        callee: Box<JuliaExpression>,
        /// The arguments to the call.
        arguments: Vec<JuliaExpression>,
    },
}
