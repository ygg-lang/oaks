use std::{boxed::Box, range::Range, string::String, vec::Vec};

use crate::kind::ErlangSyntaxKind;

/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone)]
pub struct ErlangRoot {
    pub items: Vec<Item>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

/// 顶层项：模块、函数、属性等
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Module(Module),
    Function(Function),
    Attribute(Attribute),
    Export(Export),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub name: Identifier,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Export {
    pub functions: Vec<Identifier>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: Identifier,
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Identifier,
    pub arity: usize,
    pub clauses: Vec<FunctionClause>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionClause {
    pub patterns: Vec<Pattern>,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern {
    pub name: Identifier,
    pub fields: Vec<(Identifier, Pattern)>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expr(Expr),
    Match { pattern: Pattern, expr: Expr, span: Range<usize> },
    Case(CaseExpr),
    If(IfExpr),
    Try(TryExpr),
    Receive(ReceiveExpr),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: ErlangSyntaxKind,
    pub right: Box<Expr>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunExpr {
    pub clauses: Vec<FunctionClause>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseExpr {
    pub expr: Box<Expr>,
    pub clauses: Vec<CaseClause>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaseClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    pub clauses: Vec<IfClause>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfClause {
    pub guard: Expr,
    pub body: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryExpr {
    pub body: Vec<Statement>,
    pub catch_clauses: Vec<CatchClause>,
    pub after_clause: Option<Box<Expr>>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReceiveExpr {
    pub clauses: Vec<ReceiveClause>,
    pub after_clause: Option<Box<Expr>>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReceiveClause {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RecordExpr {
    pub name: Identifier,
    pub fields: Vec<(Identifier, Expr)>,
    pub span: Range<usize>,
}
