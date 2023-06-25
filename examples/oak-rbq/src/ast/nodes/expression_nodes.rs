use crate::{language::RbqLanguage, lexer::token_type::RbqTokenType, parser::element_type::RbqElementType};
use oak_core::{
    Range,
    tree::{RedNode, RedTree},
};

/// Represents an expression in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqExpr {
    /// The kind of expression.
    pub kind: RbqExprKind,
    /// The source range of the expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents the kind of an RBQ expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqExprKind {
    /// A literal value.
    Literal(RbqLiteral),
    /// An identifier.
    Identifier(String),
    /// A binary operation.
    Binary {
        /// The left operand.
        left: Box<RbqExpr>,
        /// The operator string.
        op: String,
        /// The right operand.
        right: Box<RbqExpr>,
    },
    /// A unary operation.
    Unary {
        /// The operator string.
        op: String,
        /// The operand expression.
        expr: Box<RbqExpr>,
    },
    /// A function or method call.
    Call {
        /// The expression being called.
        callee: Box<RbqExpr>,
        /// List of arguments for the call.
        args: Vec<RbqExpr>,
    },
    /// A member access (e.g., `object.property`).
    Member {
        /// The object being accessed.
        object: Box<RbqExpr>,
        /// The name of the property.
        property: String,
    },
    /// A query pipeline expression.
    Pipeline {
        /// The base expression of the pipeline.
        base: Box<RbqExpr>,
        /// List of pipeline steps.
        steps: Vec<RbqPipelineStep>,
    },
    /// A closure definition.
    Closure {
        /// List of argument names.
        args: Vec<String>,
        /// List of expressions in the closure body.
        body: Vec<RbqExpr>,
    },
    /// A block of expressions.
    Block(Vec<RbqExpr>),
    /// A magic variable (e.g., `$`, `$key`, `$group`).
    MagicVar(String),
}

/// Represents a literal value in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RbqLiteral {
    /// A string literal.
    String(String),
    /// A numeric literal.
    Number(String),
    /// A boolean literal.
    Boolean(bool),
}

/// Represents a step in a query pipeline in RBQ.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RbqPipelineStep {
    /// The name of the pipeline step.
    pub name: String,
    /// List of arguments for the step.
    pub args: Vec<RbqExpr>,
    /// The source range of the pipeline step.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl RbqExpr {
    /// Lowers a red node into an `RbqExpr` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let kind = match red.kind::<RbqElementType>() {
            RbqElementType::Literal => {
                let text = source[span.clone()].trim().to_string();
                if let Some(leaf) = red.children().find_map(|c| {
                    let t = c.as_token()?;
                    let k = t.kind();
                    match k {
                        RbqTokenType::StringLiteral | RbqTokenType::NumberLiteral | RbqTokenType::TrueKw | RbqTokenType::FalseKw => Some(t),
                        _ => None,
                    }
                }) {
                    match leaf.kind() {
                        RbqTokenType::StringLiteral => {
                            // Strip quotes
                            let s = if text.starts_with('"') && text.ends_with('"') {
                                text[1..text.len() - 1].to_string()
                            }
                            else if text.starts_with('\'') && text.ends_with('\'') {
                                text[1..text.len() - 1].to_string()
                            }
                            else {
                                text
                            };
                            RbqExprKind::Literal(RbqLiteral::String(s))
                        }
                        RbqTokenType::NumberLiteral => RbqExprKind::Literal(RbqLiteral::Number(text)),
                        RbqTokenType::TrueKw => RbqExprKind::Literal(RbqLiteral::Boolean(true)),
                        RbqTokenType::FalseKw => RbqExprKind::Literal(RbqLiteral::Boolean(false)),
                        _ => RbqExprKind::Literal(RbqLiteral::String(text)),
                    }
                }
                else {
                    RbqExprKind::Literal(RbqLiteral::String(text))
                }
            }
            RbqElementType::Ident => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::Identifier(text)
            }
            RbqElementType::MagicVar => {
                let text = source[span.clone()].trim().to_string();
                RbqExprKind::MagicVar(text)
            }
            RbqElementType::BinaryExpr => {
                let mut left = None;
                let mut op = String::new();
                let mut right = None;
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            if left.is_none() {
                                left = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                            else {
                                right = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                        }
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind();
                            if k == RbqTokenType::Plus
                                || k == RbqTokenType::Minus
                                || k == RbqTokenType::Star
                                || k == RbqTokenType::Slash
                                || k == RbqTokenType::EqEq
                                || k == RbqTokenType::NotEq
                                || k == RbqTokenType::Lt
                                || k == RbqTokenType::Gt
                                || k == RbqTokenType::LtEq
                                || k == RbqTokenType::GtEq
                                || k == RbqTokenType::AndAnd
                                || k == RbqTokenType::OrOr
                                || k == RbqTokenType::Eq
                            {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let (Some(left), Some(right)) = (left, right) { RbqExprKind::Binary { left, op, right } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::UnaryExpr => {
                let mut op = String::new();
                let mut expr = None;
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => expr = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Leaf(leaf) => {
                            let k = leaf.kind();
                            if k == RbqTokenType::Not || k == RbqTokenType::Minus {
                                op = source[leaf.span()].trim().to_string()
                            }
                        }
                    }
                }
                if let Some(expr) = expr { RbqExprKind::Unary { op, expr } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::CallExpr => {
                let mut callee = None;
                let mut args = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            if callee.is_none() {
                                callee = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                            else {
                                args.push(RbqExpr::lower(node, source))
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(callee) = callee { RbqExprKind::Call { callee, args } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::MemberExpr => {
                let mut object = None;
                let mut property = String::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => object = Some(Box::new(RbqExpr::lower(node, source))),
                        RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident => property = source[leaf.span()].trim().to_string(),
                        _ => {}
                    }
                }
                if let Some(object) = object { RbqExprKind::Member { object, property } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::QueryPipeline => {
                let mut base = None;
                let mut steps = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => {
                            let k = node.kind::<RbqElementType>();
                            if k == RbqElementType::PipelineStep {
                                steps.push(RbqPipelineStep::lower(node, source))
                            }
                            else if base.is_none() {
                                base = Some(Box::new(RbqExpr::lower(node, source)))
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(base) = base { RbqExprKind::Pipeline { base, steps } } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            RbqElementType::Closure => {
                let mut args = Vec::new();
                let mut body = Vec::new();
                for child in red.children() {
                    match child {
                        RedTree::Node(node) => match node.kind::<RbqElementType>() {
                            RbqElementType::ClosureArgs => {
                                for arg in node.children() {
                                    match arg {
                                        RedTree::Leaf(leaf) => {
                                            if leaf.kind() == RbqTokenType::Ident {
                                                args.push(source[leaf.span()].trim().to_string())
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {
                                body.push(RbqExpr::lower(node, source));
                            }
                        },
                        _ => {}
                    }
                }
                RbqExprKind::Closure { args, body }
            }
            RbqElementType::Block => {
                let mut expressions = Vec::new();
                for child in red.children() {
                    if let RedTree::Node(node) = child {
                        expressions.push(RbqExpr::lower(node, source));
                    }
                }
                RbqExprKind::Block(expressions)
            }
            RbqElementType::Expression => {
                let first_node = red.children().find_map(|c| c.as_node());
                if let Some(node) = first_node { return Self::lower(node, source) } else { RbqExprKind::Identifier(source[span.clone()].to_string()) }
            }
            _ => RbqExprKind::Identifier(source[span.clone()].to_string()),
        };

        Self { kind, span }
    }
}

impl RbqPipelineStep {
    /// Lowers a red node into an `RbqPipelineStep` AST node.
    pub fn lower(red: RedNode<RbqLanguage>, source: &str) -> Self {
        let span = red.span();
        let mut name = String::new();
        let mut args = Vec::new();
        for child in red.children() {
            match child {
                RedTree::Node(node) => args.push(RbqExpr::lower(node, source)),
                RedTree::Leaf(leaf) if leaf.kind() == RbqTokenType::Ident && name.is_empty() => name = source[leaf.span()].trim().to_string(),
                _ => {}
            }
        }
        Self { name, args, span }
    }
}
