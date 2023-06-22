use core::range::Range;
use serde::{Deserialize, Serialize};
use std::{boxed::Box, string::String, vec::Vec};

use crate::ElixirSyntaxKind;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ElixirRoot {
    pub items: Vec<Item>,
}

/// 顶层项：模块、函数或语句
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Item {
    Module(Module),
    Function(Function),
    Statement(Statement),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Identifier,
    pub items: Vec<Item>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub body: Block,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: Identifier,
    pub ty: Option<String>,
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
    Let {
        name: Identifier,
        expr: Expr,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    ExprStmt {
        expr: Expr,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Identifier),
    Atom {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Number {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    String {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Bool {
        value: bool,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Unary {
        op: ElixirSyntaxKind,
        expr: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Binary {
        left: Box<Expr>,
        op: ElixirSyntaxKind,
        right: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Field {
        receiver: Box<Expr>,
        field: Identifier,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Index {
        receiver: Box<Expr>,
        index: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Paren {
        expr: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Block(Block),
}
