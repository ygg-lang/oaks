use core::range::Range;

use crate::kind::RSyntaxKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

/// R 语言的根节点
#[derive(Debug, PartialEq, Clone)]
pub struct RRoot {
    pub statements: Vec<Statement>,
}

/// R 语句
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assignment { name: Identifier, expr: Expr, span: Range<usize> },
    ExprStmt { expr: Expr, span: Range<usize> },
    FunctionDef { name: Identifier, params: Vec<Identifier>, body: Vec<Statement>, span: Range<usize> },
}

/// R 表达式
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(Identifier),
    Literal { value: String, span: Range<usize> },
    Bool { value: bool, span: Range<usize> },
    Null { span: Range<usize> },
    Call { callee: Box<Expr>, args: Vec<Expr>, span: Range<usize> },
    Binary { left: Box<Expr>, op: RSyntaxKind, right: Box<Expr>, span: Range<usize> },
    Unary { op: RSyntaxKind, expr: Box<Expr>, span: Range<usize> },
}
