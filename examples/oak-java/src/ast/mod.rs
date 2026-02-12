#![doc = include_str!("readme.md")]
//! Java AST definitions

use core::range::Range;

/// Root node of a Java program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JavaRoot {
    /// Items in the compilation unit
    pub items: Vec<Item>,
}

/// Annotation/Attribute
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Annotation {
    /// Annotation name
    pub name: String,
    /// Arguments (if any)
    pub arguments: Vec<Expression>,
    /// Source span
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Top-level items in a Java program
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Class declaration
    Class(ClassDeclaration),
    /// Interface declaration
    Interface(InterfaceDeclaration),
    /// Struct declaration
    Struct(StructDeclaration),
    /// Enum declaration
    Enum(EnumDeclaration),
    /// Record declaration
    Record(RecordDeclaration),
    /// Package declaration
    Package(PackageDeclaration),
    /// Import declaration
    Import(ImportDeclaration),
}

/// Class declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// Class name
    pub name: String,
    /// Modifiers
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Superclass
    pub extends: Option<String>,
    /// Implemented interfaces
    pub implements: Vec<String>,
    /// Members
    pub members: Vec<Member>,
    /// Source span
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Class members
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Member {
    /// Method declaration
    Method(MethodDeclaration),
    /// Field declaration
    Field(FieldDeclaration),
    /// Constructor declaration
    Constructor(ConstructorDeclaration),
}

/// Constructor declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstructorDeclaration {
    /// Modifiers
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Name (should match class name)
    pub name: String,
    /// Parameter list
    pub parameters: Vec<Parameter>,
    /// Method body
    pub body: Vec<Statement>,
    /// Source span
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Method declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodDeclaration {
    /// Method name
    pub name: String,
    /// Modifiers
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Return type
    pub return_type: String,
    /// Parameter list
    pub parameters: Vec<Parameter>,
    /// Method body
    pub body: Vec<Statement>,
    /// Thrown exceptions
    pub throws: Vec<String>,
    /// Whether it is static
    pub is_static: bool,
    /// Source span
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Parameter
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub r#type: String,
}

/// Field declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldDeclaration {
    /// Field name.
    pub name: String,
    /// Field type.
    pub r#type: String,
    /// Field modifiers (e.g., public, static).
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// The span of the field declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Expression statement.
    Expression(Expression),
    /// Return statement.
    Return(Option<Expression>),
    /// Block statement.
    Block(Vec<Statement>),
    /// Try statement.
    Try(TryStatement),
    /// Throw statement.
    Throw(Expression),
    /// If statement.
    If {
        /// Condition expression.
        condition: Expression,
        /// Then branch statement.
        then_branch: Box<Statement>,
        /// Optional else branch statement.
        else_branch: Option<Box<Statement>>,
    },
    /// While loop.
    While {
        /// Condition expression.
        condition: Expression,
        /// Loop body.
        body: Box<Statement>,
    },
    /// Do-while loop.
    DoWhile {
        /// Condition expression.
        condition: Expression,
        /// Loop body.
        body: Box<Statement>,
    },
    /// For loop.
    For {
        /// Initializer statement.
        init: Option<Box<Statement>>,
        /// Condition expression.
        condition: Option<Expression>,
        /// Update expression.
        update: Option<Expression>,
        /// Loop body.
        body: Box<Statement>,
    },
    /// For-each loop.
    ForEach {
        /// Item type.
        item_type: String,
        /// Item name.
        item_name: String,
        /// Iterable expression.
        iterable: Expression,
        /// Loop body.
        body: Box<Statement>,
    },
    /// Switch statement.
    Switch {
        /// Selector expression.
        selector: Expression,
        /// Switch cases.
        cases: Vec<SwitchCase>,
        /// Optional default case statements.
        default: Option<Vec<Statement>>,
    },
    /// Break statement.
    Break,
    /// Continue statement.
    Continue,
    /// Local variable declaration statement.
    LocalVariable {
        /// Variable type.
        r#type: String,
        /// Variable name.
        name: String,
        /// Optional initializer expression.
        initializer: Option<Expression>,
    },
}

/// Switch case.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwitchCase {
    /// Case label expression.
    pub label: Expression,
    /// Case body statements.
    pub body: Vec<Statement>,
}

/// Try statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TryStatement {
    /// Try block.
    pub block: Vec<Statement>,
    /// Catch clauses.
    pub catches: Vec<CatchClause>,
    /// Optional finally block.
    pub finally: Option<Vec<Statement>>,
}

/// Catch clause.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatchClause {
    /// Catch parameter.
    pub parameter: Parameter,
    /// Catch block.
    pub block: Vec<Statement>,
}

/// Expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Literal expression.
    Literal(Literal),
    /// Identifier expression (e.g., variable reference).
    Identifier(String),
    /// Method call expression.
    MethodCall(MethodCall),
    /// Field access expression.
    FieldAccess(FieldAccess),
    /// Array access expression.
    ArrayAccess(ArrayAccess),
    /// Array creation expression.
    ArrayCreation(ArrayCreation),
    /// New expression (object creation).
    New(NewExpression),
    /// This expression.
    This,
    /// Super expression.
    Super,
    /// Binary operation.
    Binary {
        /// Left operand.
        left: Box<Expression>,
        /// Operator.
        op: String,
        /// Right operand.
        right: Box<Expression>,
    },
    /// Unary operation.
    Unary {
        /// Operator.
        op: String,
        /// Operand expression.
        expression: Box<Expression>,
    },
    /// Assignment operation.
    Assignment {
        /// Left operand.
        left: Box<Expression>,
        /// Operator.
        op: String,
        /// Right operand.
        right: Box<Expression>,
    },
    /// Update operation (increment/decrement).
    Update {
        /// Operand expression.
        expression: Box<Expression>,
        /// Operator.
        op: String,
        /// Whether the operator is a prefix.
        is_prefix: bool,
    },
    /// Ternary operation.
    Ternary {
        /// Condition expression.
        condition: Box<Expression>,
        /// Then branch expression.
        then_branch: Box<Expression>,
        /// Else branch expression.
        else_branch: Box<Expression>,
    },
    /// Cast operation.
    Cast {
        /// Target type.
        target_type: String,
        /// Operand expression.
        expression: Box<Expression>,
    },
}

/// New expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewExpression {
    /// Object type.
    pub r#type: String,
    /// Constructor arguments.
    pub arguments: Vec<Expression>,
}

/// Literal.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// Integer literal.
    Integer(i64),
    /// Float literal.
    Float(f64),
    /// String literal.
    String(String),
    /// Boolean literal.
    Boolean(bool),
}

/// Field access.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldAccess {
    /// Target expression.
    pub target: Box<Expression>,
    /// Field name.
    pub name: String,
}

/// Method call.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodCall {
    /// Call target (optional, e.g., System.out).
    pub target: Option<Box<Expression>>,
    /// Method name.
    pub name: String,
    /// Arguments.
    pub arguments: Vec<Expression>,
}

/// Array access.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayAccess {
    /// Target array expression.
    pub target: Box<Expression>,
    /// Index expression.
    pub index: Box<Expression>,
}

/// Array creation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayCreation {
    /// Element type.
    pub element_type: String,
    /// Array dimensions.
    pub dimensions: Vec<Expression>,
}

/// Interface declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDeclaration {
    /// Interface name.
    pub name: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Extended interfaces.
    pub extends: Vec<String>,
    /// Interface members.
    pub members: Vec<Member>,
    /// The span of the interface declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Struct declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructDeclaration {
    /// Struct name.
    pub name: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Implemented interfaces.
    pub implements: Vec<String>,
    /// Struct members.
    pub members: Vec<Member>,
    /// The span of the struct declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Enum declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumDeclaration {
    /// Enum name
    pub name: String,
    /// Modifiers
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Implemented interfaces
    pub implements: Vec<String>,
    /// Enum variants
    pub variants: Vec<String>,
    /// Members
    pub members: Vec<Member>,
    /// Source span
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Record declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecordDeclaration {
    /// Record name.
    pub name: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Annotations
    pub annotations: Vec<Annotation>,
    /// Record parameters (e.g., primary constructor parameters).
    pub parameters: Vec<Parameter>,
    /// Implemented interfaces.
    pub implements: Vec<String>,
    /// Record members.
    pub members: Vec<Member>,
    /// The span of the record declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Package declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageDeclaration {
    /// Package name.
    pub name: String,
    /// The span of the package declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Import declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportDeclaration {
    /// Import path.
    pub path: String,
    /// Whether it's a static import.
    pub is_static: bool,
    /// The span of the import declaration in the source file.
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
