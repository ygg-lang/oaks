/// Mojo statement types.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MojoStatement {
    /// Function definition statement.
    Function {
        /// Function name.
        name: String,
        /// Function parameters as (name, type) pairs.
        params: Vec<(String, Option<String>)>,
        /// Return type annotation.
        return_type: Option<String>,
        /// Function body statements.
        body: Vec<MojoStatement>,
    },
    /// Variable declaration statement.
    Variable {
        /// Variable name.
        name: String,
        /// Type annotation.
        ty: Option<String>,
        /// Initial value expression.
        value: Option<MojoExpression>,
        /// Whether this is a let binding (immutable).
        is_let: bool,
    },
    /// Assignment statement.
    Assignment {
        /// Assignment target expression.
        target: MojoExpression,
        /// Value to assign.
        value: MojoExpression,
    },
    /// If statement.
    If {
        /// Condition expression.
        condition: MojoExpression,
        /// Then branch statements.
        then_body: Vec<MojoStatement>,
        /// Optional else branch statements.
        else_body: Option<Vec<MojoStatement>>,
    },
    /// While loop statement.
    While {
        /// Loop condition expression.
        condition: MojoExpression,
        /// Loop body statements.
        body: Vec<MojoStatement>,
    },
    /// For loop statement.
    For {
        /// Loop variable name.
        variable: String,
        /// Iterable expression.
        iterable: MojoExpression,
        /// Loop body statements.
        body: Vec<MojoStatement>,
    },
    /// Return statement.
    Return(Option<MojoExpression>),
    /// Expression statement.
    Expression(MojoExpression),
}

/// Mojo expression types.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MojoExpression {
    /// Literal expression.
    Literal(MojoLiteral),
    /// Identifier expression.
    Identifier(String),
    /// Binary expression with left operand, operator, and right operand.
    Binary {
        /// Left operand.
        left: Box<MojoExpression>,
        /// Operator.
        op: String,
        /// Right operand.
        right: Box<MojoExpression>,
    },
    /// Unary expression with operator and operand.
    Unary {
        /// Operator.
        op: String,
        /// Operand.
        right: Box<MojoExpression>,
    },
    /// Function call expression.
    Call {
        /// Callee expression.
        callee: Box<MojoExpression>,
        /// Call arguments.
        args: Vec<MojoExpression>,
    },
}

/// Mojo literal types.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MojoLiteral {
    /// Integer literal.
    Int(i64),
    /// Float literal.
    Float(f64),
    /// String literal.
    String(String),
    /// Boolean literal.
    Bool(bool),
    /// None literal.
    None,
}
