#![doc = include_str!("readme.md")]
use core::range::Range;

/// An identifier in Haskell.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,
    /// The span of the identifier in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// The root of a Haskell Abstract Syntax Tree.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HaskellRoot {
    /// The name of the module, if present.
    pub module_name: Option<Identifier>,
    /// The items in the module.
    pub items: Vec<Item>,
}

/// A top-level item in a Haskell module.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// A function definition.
    Function(Function),
    /// A data type declaration.
    DataDeclaration(DataDeclaration),
    /// A type alias declaration.
    TypeAlias(TypeAlias),
    /// An import declaration.
    Import(Import),
}

/// A function definition in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Function {
    /// The name of the function.
    pub name: Identifier,
    /// The type signature of the function, if present.
    pub type_signature: Option<Type>,
    /// The equations defining the function.
    pub equations: Vec<Equation>,
    /// The span of the function in the source file.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// An equation in a function definition.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equation {
    /// The patterns in the equation.
    pub patterns: Vec<Pattern>,
    /// The body of the equation.
    pub body: Expression,
}

/// A pattern in a function definition or case expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern {
    /// A wildcard pattern `_`.
    Wildcard,
    /// A variable pattern.
    Variable(Identifier),
    /// A constructor pattern.
    Constructor(Identifier, Vec<Pattern>),
    /// A literal pattern.
    Literal(Literal),
}

/// An expression in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// A variable expression.
    Variable(Identifier),
    /// A literal expression.
    Literal(Literal),
    /// A function application.
    Application(Box<Expression>, Box<Expression>),
    /// A lambda expression.
    Lambda(Vec<Pattern>, Box<Expression>),
    /// A let expression.
    Let(Vec<Item>, Box<Expression>),
    /// A case expression.
    Case(Box<Expression>, Vec<CaseArm>),
}

/// A case arm in a case expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseArm {
    /// The pattern to match.
    pub pattern: Pattern,
    /// The body of the case arm.
    pub body: Expression,
}

/// A type in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// A type variable.
    Variable(Identifier),
    /// A type constructor.
    Constructor(Identifier, Vec<Type>),
    /// A function type.
    Function(Box<Type>, Box<Type>),
}

/// A data type declaration in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataDeclaration {
    /// The name of the data type.
    pub name: Identifier,
    /// The type parameters of the data type.
    pub type_params: Vec<Identifier>,
    /// The constructors of the data type.
    pub constructors: Vec<ConstructorDef>,
}

/// A constructor definition in a data declaration.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstructorDef {
    /// The name of the constructor.
    pub name: Identifier,
    /// The fields of the constructor.
    pub fields: Vec<Type>,
}

/// A type alias declaration in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeAlias {
    /// The name of the type alias.
    pub name: Identifier,
    /// The type parameters of the type alias.
    pub type_params: Vec<Identifier>,
    /// The target type of the type alias.
    pub target: Type,
}

/// An import declaration in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Import {
    /// The module to import.
    pub module: Identifier,
    /// Whether the import is qualified.
    pub qualified: bool,
    /// The name to import the module as, if present.
    pub as_name: Option<Identifier>,
}

/// A literal in Haskell.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// An integer literal.
    Integer(i64),
    /// A float literal.
    Float(f64),
    /// A string literal.
    String(String),
    /// A character literal.
    Char(char),
}
