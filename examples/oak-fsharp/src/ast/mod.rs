#![doc = include_str!("readme.md")]

use core::range::Range;

/// The root node of an F# program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FSharpRoot {
    /// Items in the compilation unit
    pub items: Vec<Item>,
}

/// Top-level items in an F# program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Namespace declaration
    Namespace(NamespaceDeclaration),
    /// Module declaration
    Module(ModuleDeclaration),
    /// Open directive (open)
    Open(OpenDirective),
    /// Binding (let)
    Binding(Binding),
    /// Type definition (type)
    Type(TypeDefinition),
    /// Class definition (type ... = class)
    Class(ClassDefinition),
    /// Interface definition (type ... = interface)
    Interface(InterfaceDefinition),
    /// Exception definition (exception)
    Exception(ExceptionDefinition),
}

/// Namespace declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclaration {
    /// Namespace name
    pub name: String,
    /// Members
    pub items: Vec<Item>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleDeclaration {
    /// Module name
    pub name: String,
    /// Whether it is a top-level module
    pub is_top_level: bool,
    /// Whether it is a nested module
    pub is_nested: bool,
    /// Members
    pub items: Vec<Item>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Open directive (open)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpenDirective {
    /// Import path
    pub path: String,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Binding (let)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Binding {
    /// Binding name
    pub name: String,
    /// Whether it is a recursive binding (rec)
    pub is_rec: bool,
    /// Whether it is mutable (mutable)
    pub is_mutable: bool,
    /// Parameter list
    pub parameters: Vec<Parameter>,
    /// Type annotation
    pub type_annotation: Option<String>,
    /// Bound expression
    pub expression: Expression,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Type annotation
    pub type_annotation: Option<String>,
}

/// Type definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDefinition {
    /// Type name
    pub name: String,
    /// Type parameters
    pub type_parameters: Vec<String>,
    /// Type body
    pub body: TypeBody,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Type body
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeBody {
    /// Record type
    Record(Vec<RecordField>),
    /// Union type
    Union(Vec<UnionCase>),
    /// Alias
    Alias(String),
    /// Struct
    Struct(Vec<RecordField>),
}

/// Record field
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecordField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
}

/// Union case
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionCase {
    /// Case name
    pub name: String,
    /// Case fields
    pub fields: Vec<RecordField>,
}

/// Class definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDefinition {
    /// Class name
    pub name: String,
    /// Type parameters
    pub type_parameters: Vec<String>,
    /// Base class
    pub base_class: Option<String>,
    /// Interfaces
    pub interfaces: Vec<String>,
    /// Members
    pub members: Vec<ClassMember>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Class member
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassMember {
    /// Constructor
    Constructor(Constructor),
    /// Method
    Method(Method),
    /// Property
    Property(Property),
    /// Field
    Field(Field),
}

/// Constructor
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Constructor {
    /// Parameters
    pub parameters: Vec<Parameter>,
    /// Body
    pub body: Expression,
}

/// Method
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Method {
    /// Method name
    pub name: String,
    /// Parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<String>,
    /// Body
    pub body: Expression,
}

/// Property
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Property {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: String,
    /// Getter
    pub getter: Option<Expression>,
    /// Setter
    pub setter: Option<Expression>,
}

/// Field
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Initial value
    pub initial_value: Option<Expression>,
}

/// Interface definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDefinition {
    /// Interface name
    pub name: String,
    /// Type parameters
    pub type_parameters: Vec<String>,
    /// Base interfaces
    pub base_interfaces: Vec<String>,
    /// Members
    pub members: Vec<InterfaceMember>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface member
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterfaceMember {
    /// Method signature
    MethodSignature(Method),
    /// Property signature
    PropertySignature(Property),
}

/// Exception definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExceptionDefinition {
    /// Exception name
    pub name: String,
    /// Fields
    pub fields: Vec<RecordField>,
    /// Source span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// F# expression
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Literal
    Literal(Literal),
    /// Identifier
    Identifier(String),
    /// If expression
    If {
        /// Condition expression
        condition: Box<Expression>,
        /// Then branch
        then_branch: Box<Expression>,
        /// Else branch
        else_branch: Option<Box<Expression>>,
    },
    /// Match expression
    Match {
        /// Expression to match
        expression: Box<Expression>,
        /// Match cases
        cases: Vec<MatchCase>,
    },
    /// Lambda expression
    Lambda {
        /// Parameters
        parameters: Vec<Parameter>,
        /// Body
        body: Box<Expression>,
    },
    /// Function application
    Application {
        /// Function
        function: Box<Expression>,
        /// Argument
        argument: Box<Expression>,
    },
    /// Binary expression
    Binary {
        /// Left operand
        left: Box<Expression>,
        /// Operator
        op: String,
        /// Right operand
        right: Box<Expression>,
    },
    /// Unary expression
    Unary {
        /// Operator
        op: String,
        /// Operand
        operand: Box<Expression>,
    },
    /// Let expression
    Let {
        /// Bindings
        bindings: Vec<Binding>,
        /// Body
        body: Box<Expression>,
    },
    /// Pipe forward operator (|>)
    PipeForward {
        /// Expression
        expression: Box<Expression>,
        /// Function
        function: Box<Expression>,
    },
    /// Pipe backward operator (<|)
    PipeBackward {
        /// Function
        function: Box<Expression>,
        /// Expression
        expression: Box<Expression>,
    },
    /// Tuple
    Tuple(Vec<Expression>),
    /// List
    List(Vec<Expression>),
    /// Array
    Array(Vec<Expression>),
    /// Record expression
    Record {
        /// Type name
        type_name: Option<String>,
        /// Fields
        fields: Vec<(String, Expression)>,
    },
    /// Union case expression
    UnionCase {
        /// Case name
        case_name: String,
        /// Arguments
        arguments: Vec<Expression>,
    },
    /// Sequential expressions
    Sequential(Vec<Expression>),
    /// Parenthesized expression
    Parenthesized(Box<Expression>),
    /// Async expression
    Async(Box<Expression>),
    /// Computation expression
    Computation {
        /// Builder name
        builder: String,
        /// Body
        body: Vec<ComputationItem>,
    },
}

/// Literal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// Integer
    Integer(i64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Char
    Char(char),
    /// Boolean
    Boolean(bool),
    /// Unit
    Unit,
}

/// Match case
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MatchCase {
    /// Pattern
    pub pattern: Pattern,
    /// Guard
    pub guard: Option<Expression>,
    /// Body
    pub body: Expression,
}

/// Pattern
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pattern {
    /// Wildcard (_)
    Wildcard,
    /// Identifier
    Identifier(String),
    /// Literal
    Literal(Literal),
    /// Tuple pattern
    Tuple(Vec<Pattern>),
    /// List pattern
    List(Vec<Pattern>),
    /// Union case pattern
    UnionCase {
        /// Case name
        case_name: String,
        /// Patterns
        patterns: Vec<Pattern>,
    },
    /// Record pattern
    Record(Vec<String>),
    /// As pattern
    As {
        /// Pattern
        pattern: Box<Pattern>,
        /// Identifier
        identifier: String,
    },
    /// Or pattern
    Or(Box<Pattern>, Box<Pattern>),
}

/// Computation item
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ComputationItem {
    /// Let binding
    Let(Binding),
    /// Yield
    Yield(Expression),
    /// Return
    Return(Expression),
    /// Expression
    Expression(Expression),
    /// Custom operation
    CustomOperation {
        /// Operation name
        name: String,
        /// Arguments
        arguments: Vec<Expression>,
    },
}
