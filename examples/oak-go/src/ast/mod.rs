use core::range::Range;
use std::{boxed::Box, string::String, vec::Vec};

/// Go 语言强类型 AST 根
#[derive(Debug, PartialEq, Clone)]
pub struct GoRoot {
    pub package: Option<String>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Function(Function),
    Variable(Variable),
    Type(TypeDecl),
    Const(Const),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Block,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(Expression),
    Assignment { target: String, value: Expression, span: Range<usize> },
    Return { value: Option<Expression>, span: Range<usize> },
    If { condition: Expression, then_block: Block, else_block: Option<Block>, span: Range<usize> },
    For { condition: Option<Expression>, body: Block, span: Range<usize> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier { name: String, span: Range<usize> },
    Literal { value: String, span: Range<usize> },
    Binary { left: Box<Expression>, op: String, right: Box<Expression>, span: Range<usize> },
    Call { func: Box<Expression>, args: Vec<Expression>, span: Range<usize> },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: Option<String>,
    pub value: Option<Expression>,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeDecl {
    pub name: String,
    pub definition: String,
    pub span: Range<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Const {
    pub name: String,
    pub const_type: Option<String>,
    pub value: Expression,
    pub span: Range<usize>,
}
