use core::range::Range;
use serde::{Deserialize, Serialize};
use std::{boxed::Box, string::String, vec::Vec};

/// Go 语言强类型 AST 根
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GoRoot {
    pub package: Option<String>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Declaration {
    Function(Function),
    Variable(Variable),
    Type(TypeDecl),
    Const(Const),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Block,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Assignment {
        target: String,
        value: Expression,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Return {
        value: Option<Expression>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    If {
        condition: Expression,
        then_block: Block,
        else_block: Option<Block>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    For {
        condition: Option<Expression>,
        body: Block,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expression {
    Identifier {
        name: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Literal {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Binary {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: Option<String>,
    pub value: Option<Expression>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeDecl {
    pub name: String,
    pub definition: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Const {
    pub name: String,
    pub const_type: Option<String>,
    pub value: Expression,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
