use serde::{Deserialize, Serialize};

/// Julia 根节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JuliaRoot {
    pub statements: Vec<JuliaStatement>,
}

/// Julia 语句
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JuliaStatement {
    Function(JuliaFunction),
    Expression(JuliaExpression),
    Error,
}

/// Julia 函数定义
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JuliaFunction {
    pub name: String,
    pub body: Vec<JuliaStatement>,
}

/// Julia 表达式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JuliaExpression {
    Identifier(String),
    Literal(String),
    Binary { left: Box<JuliaExpression>, op: String, right: Box<JuliaExpression> },
}
