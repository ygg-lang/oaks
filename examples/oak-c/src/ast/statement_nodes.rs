use super::*;

/// Statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Labeled statement.
    Labeled(LabeledStatement),
    /// Compound statement.
    Compound(CompoundStatement),
    /// Expression statement.
    Expression(ExpressionStatement),
    /// Selection statement (if, switch).
    Selection(SelectionStatement),
    /// Iteration statement (while, do, for).
    Iteration(IterationStatement),
    /// Jump statement (goto, continue, break, return).
    Jump(JumpStatement),
}

impl Statement {
    /// Returns the source span of the statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Labeled(n) => n.span(),
            Self::Compound(n) => n.span.clone(),
            Self::Expression(n) => n.span.clone(),
            Self::Selection(n) => n.span(),
            Self::Iteration(n) => n.span(),
            Self::Jump(n) => n.span(),
        }
    }
}

/// Labeled statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum LabeledStatement {
    /// `identifier: statement`
    Label {
        /// Label name.
        identifier: String,
        /// Labeled statement.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `case constant-expression: statement`
    Case {
        /// Case expression.
        constant_expression: Expression,
        /// Statement.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `default: statement`
    Default {
        /// Statement.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
}

impl LabeledStatement {
    /// Returns the source span of the labeled statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Label { span, .. } => span.clone(),
            Self::Case { span, .. } => span.clone(),
            Self::Default { span, .. } => span.clone(),
        }
    }
}

/// Compound statement (block).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundStatement {
    /// List of block items (declarations or statements).
    pub block_items: Vec<BlockItem>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Block item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum BlockItem {
    /// Declaration.
    Declaration(Declaration),
    /// Statement.
    Statement(Statement),
}

impl BlockItem {
    /// Returns the source span of the block item.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Declaration(n) => n.span.clone(),
            Self::Statement(n) => n.span(),
        }
    }
}

/// Expression statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    /// Optional expression.
    pub expression: Option<Expression>,
    /// Source span.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: core::range::Range<usize>,
}

/// Selection statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionStatement {
    /// `if (condition) then_statement else else_statement?`
    If {
        /// Condition.
        condition: Expression,
        /// Then branch.
        then_statement: Box<Statement>,
        /// Optional else branch.
        else_statement: Option<Box<Statement>>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `switch (expression) statement`
    Switch {
        /// Switch expression.
        expression: Expression,
        /// Switch body.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
}

impl SelectionStatement {
    /// Returns the source span of the selection statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::If { span, .. } => span.clone(),
            Self::Switch { span, .. } => span.clone(),
        }
    }
}

/// Iteration statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum IterationStatement {
    /// `while (condition) statement`
    While {
        /// Condition.
        condition: Expression,
        /// Loop body.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `do statement while (condition);`
    DoWhile {
        /// Loop body.
        statement: Box<Statement>,
        /// Condition.
        condition: Expression,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
    /// `for (init; condition; update) statement`
    For {
        /// Optional initializer expression.
        init: Option<Expression>,
        /// Optional condition expression.
        condition: Option<Expression>,
        /// Optional update expression.
        update: Option<Expression>,
        /// Loop body.
        statement: Box<Statement>,
        /// Source span.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: core::range::Range<usize>,
    },
}

impl IterationStatement {
    /// Returns the source span of the iteration statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::While { span, .. } => span.clone(),
            Self::DoWhile { span, .. } => span.clone(),
            Self::For { span, .. } => span.clone(),
        }
    }
}

/// Jump statement.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum JumpStatement {
    /// `goto identifier;`
    Goto(String, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// `continue;`
    Continue(#[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// `break;`
    Break(#[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
    /// `return expression?;`
    Return(Option<Expression>, #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))] core::range::Range<usize>),
}

impl JumpStatement {
    /// Returns the source span of the jump statement.
    pub fn span(&self) -> core::range::Range<usize> {
        match self {
            Self::Goto(_, span) => span.clone(),
            Self::Continue(span) => span.clone(),
            Self::Break(span) => span.clone(),
            Self::Return(_, span) => span.clone(),
        }
    }
}
