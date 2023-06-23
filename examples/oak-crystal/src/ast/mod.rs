#![doc = include_str!("readme.md")]
use core::range::Range;

/// Identifier in the Crystal language
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Range<usize>,
}

/// Crystal AST root
#[derive(Debug, PartialEq, Clone)]
pub struct CrystalRoot {
    pub items: Vec<Item>,
}

/// Top-level items: classes, modules, methods, etc.
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Class(ClassDeclaration),
    Module(ModuleDeclaration),
    Def(MethodDeclaration),
    Expression(Expression),
}

/// Class declaration in Crystal
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    pub name: Identifier,
    pub body: Vec<Item>,
    pub span: Range<usize>,
}

/// Module declaration in Crystal
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration {
    pub name: Identifier,
    pub body: Vec<Item>,
    pub span: Range<usize>,
}

/// Method declaration in Crystal
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDeclaration {
    pub name: Identifier,
    pub params: Vec<Parameter>,
    pub body: Vec<Item>,
    pub span: Range<usize>,
}

/// Parameter in a method declaration
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub type_name: Option<Identifier>,
}

/// Basic expression
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Call(Call),
}

/// Literals in Crystal
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Nil,
}

/// Method call
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub receiver: Option<Box<Expression>>,
    pub name: Identifier,
    pub args: Vec<Expression>,
}
