#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Julia 根节点
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaRoot {
    pub statements: Vec<JuliaStatement>,
}

/// Julia 语句
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStatement {
    Function(JuliaFunction),
    If { condition: JuliaExpression, then_body: Vec<JuliaStatement>, else_body: Option<Vec<JuliaStatement>> },
    For { variable: String, iterable: JuliaExpression, body: Vec<JuliaStatement> },
    Expression(JuliaExpression),
    Error,
}

/// Julia 函数定义
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaFunction {
    pub name: String,
    pub body: Vec<JuliaStatement>,
}

/// Julia 表达式
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaExpression {
    Identifier(String),
    Literal(String),
    Binary { left: Box<JuliaExpression>, op: String, right: Box<JuliaExpression> },
    Call { callee: Box<JuliaExpression>, arguments: Vec<JuliaExpression> },
}
