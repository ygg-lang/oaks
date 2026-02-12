#![doc = include_str!("readme.md")]
use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

use crate::lexer::token_type::ErlangTokenType;

/// Erlang AST root.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ErlangRoot {
    /// Items in the module.
    pub items: Vec<Item>,
}

/// An identifier in Erlang.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    /// Name of the identifier.
    pub name: String,
    /// Source span of the identifier.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Top-level items: modules, functions, attributes, etc.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    /// Module declaration.
    Module(Module),
    /// Function declaration.
    Function(Function),
    /// Attribute declaration.
    Attribute(Attribute),
    /// Export declaration.
    Export(Export),
}

/// A module declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    /// Name of the module.
    pub name: Identifier,
    /// Source span of the module declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An export declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Export {
    /// Exported functions.
    pub functions: Vec<Identifier>,
    /// Source span of the export declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An attribute declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    /// Name of the attribute.
    pub name: Identifier,
    /// Value of the attribute.
    pub value: String,
    /// Source span of the attribute declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A function declaration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    /// Name of the function.
    pub name: Identifier,
    /// Arity of the function.
    pub arity: usize,
    /// Clauses of the function.
    pub clauses: Vec<FunctionClause>,
    /// Source span of the function declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A clause of a function.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionClause {
    /// Patterns for the clause.
    pub patterns: Vec<Pattern>,
    /// Optional guard for the clause.
    pub guard: Option<Expr>,
    /// Body of the clause.
    pub body: Vec<Statement>,
    /// Source span of the function clause.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A pattern matching expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    /// Variable pattern.
    Variable(Identifier),
    /// Atom pattern.
    Atom(String),
    /// Number pattern.
    Number(String),
    /// String pattern.
    String(String),
    /// List pattern.
    List(Vec<Pattern>),
    /// Tuple pattern.
    Tuple(Vec<Pattern>),
    /// Record pattern.
    Record(RecordPattern),
    /// Wildcard pattern.
    Wildcard,
}

/// A record pattern.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern {
    /// Name of the record.
    pub name: Identifier,
    /// Fields of the record pattern.
    pub fields: Vec<(Identifier, Pattern)>,
    /// Source span of the record pattern.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A statement in Erlang.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Expression statement.
    Expr(Expr),
    /// Match statement.
    Match {
        /// Pattern to match.
        pattern: Pattern,
        /// Expression to match against.
        expr: Expr,
        /// Source span of the match statement.
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// Case statement.
    Case(CaseExpr),
    /// If statement.
    If(IfExpr),
    /// Try statement.
    Try(TryExpr),
}

/// An expression in Erlang.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// Variable expression.
    Variable(Identifier),
    /// Atom expression.
    Atom(String),
    /// Number expression.
    Number(String),
    /// String expression.
    String(String),
    /// Binary expression.
    Binary(BinaryExpr),
    /// List expression.
    List(Vec<Expr>),
    /// Tuple expression.
    Tuple(Vec<Expr>),
    /// Record expression.
    Record(RecordExpr),
    /// Function call expression.
    Call(CallExpr),
    /// Fun expression.
    Fun(FunExpr),
    /// Case expression.
    Case(CaseExpr),
    /// If expression.
    If(IfExpr),
    /// Try expression.
    Try(TryExpr),
    /// Receive expression.
    Receive(ReceiveExpr),
}

/// A binary expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpr {
    /// Left-hand side expression.
    pub left: Box<Expr>,
    /// Operator.
    pub op: ErlangTokenType,
    /// Right-hand side expression.
    pub right: Box<Expr>,
    /// Source span of the binary expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A function call expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    /// Callee expression.
    pub callee: Box<Expr>,
    /// Arguments for the call.
    pub args: Vec<Expr>,
    /// Source span of the call expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A fun expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct FunExpr {
    /// Clauses of the fun expression.
    pub clauses: Vec<FunctionClause>,
    /// Source span of the fun expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A case expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CaseExpr {
    /// Expression to match.
    pub expr: Box<Expr>,
    /// Clauses of the case expression.
    pub clauses: Vec<CaseClause>,
    /// Source span of the case expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A clause in a case expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CaseClause {
    /// Pattern for the clause.
    pub pattern: Pattern,
    /// Optional guard for the clause.
    pub guard: Option<Expr>,
    /// Body of the clause.
    pub body: Vec<Statement>,
    /// Source span of the case clause.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An if expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct IfExpr {
    /// Clauses of the if expression.
    pub clauses: Vec<IfClause>,
    /// Source span of the if expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A clause in an if expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct IfClause {
    /// Guard for the clause.
    pub guard: Expr,
    /// Body of the clause.
    pub body: Vec<Statement>,
    /// Source span of the if clause.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A try expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct TryExpr {
    /// Body of the try expression.
    pub body: Vec<Statement>,
    /// Catch clauses.
    pub catch_clauses: Vec<CatchClause>,
    /// Optional after clause.
    pub after_clause: Option<Box<Expr>>,
    /// Source span of the try expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A catch clause.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    /// Pattern for the clause.
    pub pattern: Pattern,
    /// Optional guard for the clause.
    pub guard: Option<Expr>,
    /// Body of the clause.
    pub body: Vec<Statement>,
    /// Source span of the catch clause.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A receive expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ReceiveExpr {
    /// Clauses of the receive expression.
    pub clauses: Vec<ReceiveClause>,
    /// Optional after clause.
    pub after_clause: Option<Box<Expr>>,
    /// Source span of the receive expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A clause in a receive expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ReceiveClause {
    /// Pattern for the clause.
    pub pattern: Pattern,
    /// Optional guard for the clause.
    pub guard: Option<Expr>,
    /// Body of the clause.
    pub body: Vec<Statement>,
    /// Source span of the receive clause.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A record expression.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordExpr {
    /// Name of the record.
    pub name: Identifier,
    /// Fields of the record expression.
    pub fields: Vec<(Identifier, Expr)>,
    /// Source span of the record expression.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
