/// Expression types in the Racket AST.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    /// Identifier expression.
    Identifier(String),
    /// Number literal expression.
    Number(String),
    /// String literal expression.
    String(String),
    /// Boolean literal expression.
    Boolean(bool),

    /// Binary expression with left operand, operator, and right operand.
    BinaryExpression(Box<BinaryExpression>),
    /// Unary expression with operator and operand.
    UnaryExpression(Box<UnaryExpression>),
    /// Function call expression.
    Call(Box<Call>),
    /// Index access expression.
    Index(Box<Index>),
    /// Tuple expression.
    Tuple(Vec<Expression>),
    /// List expression.
    List(Vec<Expression>),
    /// Map/dictionary expression.
    Map(Vec<(Expression, Expression)>),

    /// For loop expression.
    For(Box<For>),

    /// List comprehension expression.
    ListComprehension(Box<ListComprehension>),
}

/// Binary expression with left operand, operator, and right operand.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpression {
    /// Left operand of the binary expression.
    pub left: Expression,
    /// Operator of the binary expression.
    pub operator: String,
    /// Right operand of the binary expression.
    pub right: Expression,
}

/// Unary expression with operator and operand.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    /// Operator of the unary expression.
    pub operator: String,
    /// Operand of the unary expression.
    pub expression: Expression,
}

/// Function call expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Call {
    /// Function being called.
    pub function: Expression,
    /// Arguments passed to the function.
    pub arguments: Vec<Expression>,
}

/// Index access expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Index {
    /// Expression being indexed.
    pub expression: Expression,
    /// Index value.
    pub index: Expression,
}

/// For loop expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct For {
    /// Loop variable name.
    pub variable: String,
    /// Iterable expression.
    pub iterable: Expression,
    /// Loop body expressions.
    pub body: Vec<Expression>,
}

/// List comprehension expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListComprehension {
    /// Output expression.
    pub expression: Expression,
    /// Loop variable name.
    pub variable: String,
    /// Iterable expression.
    pub iterable: Expression,
    /// Optional filter condition.
    pub condition: Option<Expression>,
}
