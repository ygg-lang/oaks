#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HaskellRoot {
    pub module_name: Option<Identifier>,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    Function(Function),
    DataDeclaration(DataDeclaration),
    TypeAlias(TypeAlias),
    Import(Import),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Function {
    pub name: Identifier,
    pub type_signature: Option<Type>,
    pub equations: Vec<Equation>,
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Equation {
    pub patterns: Vec<Pattern>,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pattern {
    Wildcard,
    Variable(Identifier),
    Constructor(Identifier, Vec<Pattern>),
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    Variable(Identifier),
    Literal(Literal),
    Application(Box<Expression>, Box<Expression>),
    Lambda(Vec<Pattern>, Box<Expression>),
    Let(Vec<Item>, Box<Expression>),
    Case(Box<Expression>, Vec<CaseArm>),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CaseArm {
    pub pattern: Pattern,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    Variable(Identifier),
    Constructor(Identifier, Vec<Type>),
    Function(Box<Type>, Box<Type>),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DataDeclaration {
    pub name: Identifier,
    pub type_params: Vec<Identifier>,
    pub constructors: Vec<ConstructorDef>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstructorDef {
    pub name: Identifier,
    pub fields: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeAlias {
    pub name: Identifier,
    pub type_params: Vec<Identifier>,
    pub target: Type,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Import {
    pub module: Identifier,
    pub qualified: bool,
    pub as_name: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
}
