//! Surface syntax → FullForm helpers for owned Wolfram expressions.

use crate::{
    ast::{
        expression_nodes::{BinaryExpr, Expression, UnaryExpr},
        root_nodes::{Identifier, Span, WolframRoot},
    },
    lexer::token_type::WolframTokenType,
};

impl WolframRoot {
    /// Rewrite every top-level expression into FullForm (`Call` / `Symbol` / `Literal` / `List`).
    pub fn full_form(self) -> Self {
        Self { expressions: self.expressions.into_iter().map(Expression::full_form).collect(), span: self.span }
    }
}

impl Expression {
    /// Rewrite surface syntax into FullForm.
    ///
    /// Operators, `Part`, blanks, and patterns become ordinary `Call` nodes
    /// (`Plus`, `Part`, `Blank`, `Pattern`, …). Parentheses unwrap. Atoms and
    /// lists keep their shape. Nested children are rewritten recursively.
    pub fn full_form(self) -> Self {
        match self {
            Self::Symbol(_) | Self::Literal { .. } | Self::Error { .. } => self,
            Self::Grouped { expression, .. } => expression.full_form(),
            Self::List { elements, span } => Self::List { elements: elements.into_iter().map(Self::full_form).collect(), span },
            Self::Call { head, arguments, span } => Self::Call {
                head: Box::new(head.full_form()),
                arguments: arguments.into_iter().map(Self::full_form).collect(),
                span,
            },
            Self::Part { expression, indices, span } => {
                let mut args = vec![expression.full_form()];
                args.extend(indices.into_iter().map(Self::full_form));
                call("Part", args, span)
            }
            Self::Binary(bin) => full_form_binary(*bin),
            Self::Prefix(u) => full_form_prefix(*u),
            Self::Postfix(u) => full_form_postfix(*u),
            Self::Blank { kind, head, span } => {
                let blank_head = blank_head_name(kind);
                match head {
                    Some(h) => call(blank_head, vec![h.full_form()], span),
                    None => call(blank_head, vec![], span),
                }
            }
            Self::Pattern { name, blank, span } => {
                let blank_expr = call(blank_head_name(blank), vec![], span.clone());
                call("Pattern", vec![name.full_form(), blank_expr], span)
            }
        }
    }
}

fn full_form_binary(bin: BinaryExpr) -> Expression {
    let span = bin.span.clone();
    let lhs = bin.lhs.full_form();
    let rhs = bin.rhs.full_form();
    match bin.operator {
        WolframTokenType::Plus => call("Plus", vec![lhs, rhs], span),
        WolframTokenType::Minus => call("Subtract", vec![lhs, rhs], span),
        WolframTokenType::Times => call("Times", vec![lhs, rhs], span),
        WolframTokenType::Divide => call("Divide", vec![lhs, rhs], span),
        WolframTokenType::Power => call("Power", vec![lhs, rhs], span),
        WolframTokenType::At => Expression::Call { head: Box::new(lhs), arguments: vec![rhs], span },
        WolframTokenType::SlashSlash => Expression::Call { head: Box::new(rhs), arguments: vec![lhs], span },
        WolframTokenType::Arrow | WolframTokenType::Rule => call("Rule", vec![lhs, rhs], span),
        WolframTokenType::RuleDelayedOp | WolframTokenType::RuleDelayed | WolframTokenType::DoubleArrow => {
            call("RuleDelayed", vec![lhs, rhs], span)
        }
        WolframTokenType::MapOperator => call("Map", vec![lhs, rhs], span),
        WolframTokenType::ApplyOperator => call("Apply", vec![lhs, rhs], span),
        WolframTokenType::ApplyLevelOperator => {
            call("Apply", vec![lhs, rhs, Expression::List { elements: vec![int_lit(1, span.clone())], span: span.clone() }], span)
        }
        WolframTokenType::MapAllOperator => call("MapAll", vec![lhs, rhs], span),
        WolframTokenType::Semicolon => call("CompoundExpression", vec![lhs, rhs], span),
        WolframTokenType::Assign | WolframTokenType::Set => call("Set", vec![lhs, rhs], span),
        WolframTokenType::SetDelayed => call("SetDelayed", vec![lhs, rhs], span),
        WolframTokenType::Equal => call("Equal", vec![lhs, rhs], span),
        WolframTokenType::NotEqual => call("Unequal", vec![lhs, rhs], span),
        WolframTokenType::Less => call("Less", vec![lhs, rhs], span),
        WolframTokenType::Greater => call("Greater", vec![lhs, rhs], span),
        WolframTokenType::LessEqual => call("LessEqual", vec![lhs, rhs], span),
        WolframTokenType::GreaterEqual => call("GreaterEqual", vec![lhs, rhs], span),
        WolframTokenType::And => call("And", vec![lhs, rhs], span),
        WolframTokenType::Or => call("Or", vec![lhs, rhs], span),
        other => call(&format!("{other:?}"), vec![lhs, rhs], span),
    }
}

fn full_form_prefix(u: UnaryExpr) -> Expression {
    let span = u.span.clone();
    let operand = u.operand.full_form();
    match u.operator {
        WolframTokenType::Minus => call("Times", vec![int_lit(-1, span.clone()), operand], span),
        WolframTokenType::Factorial => call("Not", vec![operand], span),
        other => call(&format!("{other:?}"), vec![operand], span),
    }
}

fn full_form_postfix(u: UnaryExpr) -> Expression {
    let span = u.span.clone();
    let operand = u.operand.full_form();
    match u.operator {
        WolframTokenType::Ampersand => call("Function", vec![operand], span),
        WolframTokenType::Factorial => call("Factorial", vec![operand], span),
        WolframTokenType::Underscore => call("Pattern", vec![operand, call("Blank", vec![], span.clone())], span),
        WolframTokenType::DoubleUnderscore => {
            call("Pattern", vec![operand, call("BlankSequence", vec![], span.clone())], span)
        }
        WolframTokenType::TripleUnderscore => {
            call("Pattern", vec![operand, call("BlankNullSequence", vec![], span.clone())], span)
        }
        other => call(&format!("{other:?}"), vec![operand], span),
    }
}

fn blank_head_name(kind: WolframTokenType) -> &'static str {
    match kind {
        WolframTokenType::DoubleUnderscore => "BlankSequence",
        WolframTokenType::TripleUnderscore => "BlankNullSequence",
        _ => "Blank",
    }
}

fn call(head: &str, arguments: Vec<Expression>, span: Span) -> Expression {
    Expression::Call {
        head: Box::new(Expression::Symbol(Identifier { name: head.to_string(), span: span.clone() })),
        arguments,
        span,
    }
}

fn int_lit(value: i64, span: Span) -> Expression {
    Expression::Literal { value: value.to_string(), span }
}
