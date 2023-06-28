#![doc = include_str!("readme.md")]

use core::range::Range;

/// Identifier in the Crystal language
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    /// Name of the identifier.
    pub name: String,
    /// Range of the identifier in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Crystal AST root
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct CrystalRoot {
    /// Items in the root.
    pub items: Vec<Item>,
}

/// Top-level items: classes, modules, methods, etc.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    /// Class declaration.
    Class(ClassDeclaration),
    /// Module declaration.
    Module(ModuleDeclaration),
    /// Method declaration.
    Def(MethodDeclaration),
    /// An expression.
    Expression(Expression),
}

/// Class declaration in Crystal
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ClassDeclaration {
    /// Name of the class.
    pub name: Identifier,
    /// Body of the class.
    pub body: Vec<Item>,
    /// Range of the class declaration in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module declaration in Crystal
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct ModuleDeclaration {
    /// Name of the module.
    pub name: Identifier,
    /// Body of the module.
    pub body: Vec<Item>,
    /// Range of the module declaration in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Method declaration in Crystal
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDeclaration {
    /// Name of the method.
    pub name: Identifier,
    /// Parameters of the method.
    pub params: Vec<Parameter>,
    /// Body of the method.
    pub body: Vec<Item>,
    /// Range of the method declaration in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter in a method declaration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    /// Name of the parameter.
    pub name: Identifier,
    /// Type of the parameter.
    pub type_name: Option<Identifier>,
}

/// Basic expression
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A literal.
    Literal(Literal),
    /// A method call.
    Call(Call),
}

/// Literals in Crystal
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// Numeric literal.
    Number(String),
    /// String literal.
    String(String),
    /// Boolean literal.
    Boolean(bool),
    /// `nil` literal.
    Nil,
}

/// Method call
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    /// Receiver of the call.
    pub receiver: Option<Box<Expression>>,
    /// Name of the method called.
    pub name: Identifier,
    /// Arguments passed to the call.
    pub args: Vec<Expression>,
}
