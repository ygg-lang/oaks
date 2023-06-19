use core::range::Range;

use crate::RustSyntaxKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

/// 强类型 AST 根
#[derive(Debug, PartialEq, Clone)]
pub struct RustRoot {
    pub items: Vec<Item>,
}

/// 顶层项：函数或语句
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Function(Function),
    Statement(Statement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub name: Identifier,
    pub ty: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: Identifier, expr: Expr, span: Range<usize> },
    ExprStmt { expr: Expr, semi: bool, span: Range<usize> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Ident(Identifier),
    Literal { value: String, span: Range<usize> },
    Bool { value: bool, span: Range<usize> },
    Unary { op: RustSyntaxKind, expr: Box<Expr>, span: Range<usize> },
    Binary { left: Box<Expr>, op: RustSyntaxKind, right: Box<Expr>, span: Range<usize> },
    Call { callee: Box<Expr>, args: Vec<Expr>, span: Range<usize> },
    Field { receiver: Box<Expr>, field: Identifier, span: Range<usize> },
    Index { receiver: Box<Expr>, index: Box<Expr>, span: Range<usize> },
    Paren { expr: Box<Expr>, span: Range<usize> },
    Block(Block),
}
