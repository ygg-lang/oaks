use super::{Identifier, Item, LoopKind, MatchArm, NamePath, Param, Pattern, Span, StringLiteral, Type};

/// An expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expr {
    /// An identifier expression.
    Ident(Identifier),
    /// A name path expression (e.g., `std::collections::HashMap`).
    Path(NamePath),
    /// A string literal expression.
    StringLiteral(StringLiteral),
    /// A boolean literal expression.
    Bool {
        /// The boolean value.
        value: bool,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A binary operation expression.
    Binary {
        /// The left operand.
        left: Box<Expr>,
        /// The binary operator.
        op: crate::lexer::token_type::ValkyrieTokenType,
        /// The right operand.
        right: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A unary operation expression.
    Unary {
        /// The unary operator.
        op: crate::lexer::token_type::ValkyrieTokenType,
        /// The operand expression.
        expr: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A function call expression.
    Call {
        /// The callee expression.
        callee: Box<Expr>,
        /// The call arguments.
        args: Vec<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A field access expression.
    Field {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The field name.
        field: Identifier,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An index expression.
    Index {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An offset expression (pointer arithmetic).
    Offset {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The offset expression.
        offset: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A parenthesized expression.
    Paren {
        /// The inner expression.
        expr: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A block expression.
    Block(Block),
    /// A lambda expression.
    Lambda(LambdaExpr),
    /// An object expression.
    ///
    /// Creates a new object instance with specified field values.
    ///
    /// ```v
    /// let p = Point { x: 10, y: 20 }
    /// let shorthand = Point { x, y }  // shorthand syntax
    /// ```
    Object {
        /// The callee expression.
        callee: Box<Expr>,
        /// The field-value pairs. None for shorthand syntax.
        fields: Vec<(Identifier, Option<Expr>)>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// Anonymous class expression.
    ///
    /// ```v
    /// let obj = class { x: 10, y: 20 }
    /// let impl_trait = class: Trait { ... }
    /// ```
    AnonymousClass {
        /// Parent traits or classes to implement/extend.
        parents: Vec<String>,
        /// Fields and methods defined in the anonymous class.
        items: Vec<Item>,
        /// Variables captured from the enclosing scope.
        captures: Vec<Identifier>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// An if expression.
    If {
        /// Optional pattern for pattern-matching the condition.
        pattern: Option<Pattern>,
        /// The condition expression.
        condition: Box<Expr>,
        /// The then branch block.
        then_branch: Block,
        /// The optional else branch block.
        else_branch: Option<Block>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A match expression.
    Match {
        /// The expression being matched.
        scrutinee: Box<Expr>,
        /// The match arms.
        arms: Vec<MatchArm>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A loop expression.
    Loop {
        /// The loop keyword kind.
        kind: LoopKind,
        /// Optional label for the loop.
        label: Option<String>,
        /// Optional pattern for loop variable binding.
        pattern: Option<Pattern>,
        /// Optional condition for conditional loops.
        condition: Option<Box<Expr>>,
        /// The loop body.
        body: Block,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A return expression.
    Return {
        /// The optional return value expression.
        expr: Option<Box<Expr>>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A break expression.
    Break {
        /// Optional label of the loop to break from.
        label: Option<String>,
        /// Optional value to break with.
        expr: Option<Box<Expr>>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A continue expression.
    Continue {
        /// Optional label of the loop to continue.
        label: Option<String>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A yield expression.
    Yield {
        /// The optional value to yield.
        expr: Option<Box<Expr>>,
        /// Whether this is a yield from expression.
        yield_from: bool,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A raise (throw) expression.
    Raise {
        /// The expression to raise.
        expr: Box<Expr>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// A catch (try-catch) expression.
    Catch {
        /// The expression to try.
        expr: Box<Expr>,
        /// The catch arms.
        arms: Vec<MatchArm>,
        /// The source code span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
    /// With expression for functional record updates.
    ///
    /// Creates a new record by copying an existing one and updating specified fields.
    ///
    /// ```v
    /// let p2 = p1.with { x: 20.0, y: 30.0 }
    /// let updated = config.with { timeout: 60 }
    /// ```
    With {
        /// The base expression to copy from.
        base: Box<Expr>,
        /// Field updates to apply.
        updates: Vec<(Identifier, Expr)>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Span,
    },
}

/// A block of statements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Block {
    /// The statements in the block.
    pub statements: Vec<super::Statement>,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}

/// A lambda expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LambdaExpr {
    /// The lambda parameters.
    pub params: Vec<Param>,
    /// Optional return type annotation.
    pub return_type: Option<Type>,
    /// The lambda body.
    pub body: Block,
    /// The source code span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Span,
}
