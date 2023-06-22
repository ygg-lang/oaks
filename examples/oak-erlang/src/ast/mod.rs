use core::range::Range;
use serde::{Deserialize, Serialize};
use std::{boxed::Box, string::String, vec::Vec};

use crate::kind::ErlangSyntaxKind;

/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErlangRoot {
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// 顶层项：模块、函数、属性等
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Item {
    Module(Module),
    Function(Function),
    Attribute(Attribute),
    Export(Export),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: Identifier,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Export {
    pub functions: Vec<Identifier>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Identifier,
    pub value: String,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: Identifier,
    pub arity: usize,
    pub clauses: Vec<FunctionClause>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionClause {
    pub patterns: Vec<Pattern>,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    Variable(Identifier),
    Atom(String),
    Number(String),
    String(String),
    List(Vec<Pattern>),
    Tuple(Vec<Pattern>),
    Record(RecordPattern),
    Wildcard,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordPattern {
    pub name: Identifier,
    pub fields: Vec<(Identifier, Pattern)>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    Expr(Expr),
    Match {
        pattern: Pattern,
        expr: Expr,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    Case(CaseExpr),
    If(IfExpr),
    Try(TryExpr),
    Receive(ReceiveExpr),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    Variable(Identifier),
    Atom(String),
    Number(String),
    String(String),
    Binary(BinaryExpr),
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
    Record(RecordExpr),
    Call(CallExpr),
    Fun(FunExpr),
    Case(CaseExpr),
    If(IfExpr),
    Try(TryExpr),
    Receive(ReceiveExpr),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: ErlangSyntaxKind,
    pub right: Box<Expr>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunExpr {
    pub clauses: Vec<FunctionClause>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CaseExpr {
    pub expr: Box<Expr>,
    pub clauses: Vec<CaseClause>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CaseClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IfExpr {
    pub clauses: Vec<IfClause>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IfClause {
    pub guard: Expr,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TryExpr {
    pub body: Vec<Statement>,
    pub catch_clauses: Vec<CatchClause>,
    pub after_clause: Option<Box<Expr>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceiveExpr {
    pub clauses: Vec<ReceiveClause>,
    pub after_clause: Option<Box<Expr>>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceiveClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordExpr {
    pub name: Identifier,
    pub fields: Vec<(Identifier, Expr)>,
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
