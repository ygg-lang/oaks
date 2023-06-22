use core::range::Range;
use serde::{Deserialize, Serialize};

use crate::kind::RSyntaxKind;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// R 语言的根节点
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RRoot {
    pub statements: Vec<Statement>,
}

/// R 语句
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    Assignment {
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
    FunctionDef {
        name: Identifier,
        params: Vec<Identifier>,
        body: Vec<Statement>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

/// R 表达式
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Ident(Identifier),
    Literal {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Bool {
        value: bool,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Null {
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Binary {
        left: Box<Expr>,
        op: RSyntaxKind,
        right: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Unary {
        op: RSyntaxKind,
        expr: Box<Expr>,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}
