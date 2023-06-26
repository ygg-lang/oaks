#![doc = include_str!("readme.md")]

use core::range::Range;

/// Root node of the C# AST.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CSharpRoot {
    /// Items in the compilation unit.
    pub items: Vec<Item>,
}

/// Top-level items in a C# program.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Namespace declaration.
    Namespace(NamespaceDeclaration),
    /// Using directive.
    Using(UsingDirective),
    /// Class declaration.
    Class(ClassDeclaration),
    /// Interface declaration.
    Interface(InterfaceDeclaration),
    /// Struct declaration.
    Struct(StructDeclaration),
    /// Enum declaration.
    Enum(EnumDeclaration),
    /// Record declaration.
    Record(RecordDeclaration),
    /// Delegate declaration.
    Delegate(DelegateDeclaration),
}

/// Namespace declaration.
///
/// Represents a `namespace` block in C#, which groups related classes and other types.
/// Supports both block-scoped and file-scoped namespaces (C# 10+).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclaration {
    /// The fully qualified name of the namespace (e.g., "System.Collections.Generic").
    pub name: String,
    /// Attributes applied to the namespace declaration.
    pub attributes: Vec<Attribute>,
    /// Types and nested namespaces defined within this namespace.
    pub items: Vec<Item>,
    /// Source location of the entire namespace declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Using directive.
///
/// Represents a `using` statement used to import types from a namespace or to create aliases.
/// Supports `using`, `using static`, and `global using`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UsingDirective {
    /// The namespace or type path being imported.
    pub path: String,
    /// Indicates if this is a `using static` directive.
    pub is_static: bool,
    /// An optional alias for the namespace or type (e.g., `using Project = MyCompany.Project;`).
    pub alias: Option<String>,
    /// Indicates if this is a `global using` directive (C# 10+).
    pub is_global: bool,
    /// Source location of the using directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Class declaration.
///
/// Represents a `class` definition in C#. Classes are the primary reference types
/// in C# and support inheritance, interfaces, and generics.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// The name of the class.
    pub name: String,
    /// Attributes applied to the class.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `private`, `static`, `abstract`, `sealed`, `partial`.
    pub modifiers: Vec<String>,
    /// The base class and any implemented interfaces.
    pub base_types: Vec<String>,
    /// Generic type parameters (e.g., `T` in `List<T>`).
    pub type_parameters: Vec<TypeParameter>,
    /// Constraints on generic type parameters (e.g., `where T : class`).
    pub constraints: Vec<TypeParameterConstraint>,
    /// Members of the class, including fields, properties, methods, and nested types.
    pub members: Vec<Member>,
    /// Source location of the class declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Struct declaration.
///
/// Represents a `struct` definition in C#. Structs are value types
/// and are typically used for small, data-centric structures.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructDeclaration {
    /// The name of the struct.
    pub name: String,
    /// Attributes applied to the struct.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `private`, `readonly`, `ref`, `partial`.
    pub modifiers: Vec<String>,
    /// Members of the struct.
    pub members: Vec<Member>,
    /// Generic type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// Source location of the struct declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface declaration.
///
/// Represents an `interface` definition in C#. Interfaces define a contract
/// that classes or structs must implement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDeclaration {
    /// The name of the interface.
    pub name: String,
    /// Attributes applied to the interface.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `internal`, `partial`.
    pub modifiers: Vec<String>,
    /// Members defined in the interface (methods, properties, etc.).
    pub members: Vec<Member>,
    /// Generic type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// Source location of the interface declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Enum declaration.
///
/// Represents an `enum` definition in C#. Enums are value types that
/// consist of a set of named constants.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumDeclaration {
    /// The name of the enum.
    pub name: String,
    /// Attributes applied to the enum.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `internal`.
    pub modifiers: Vec<String>,
    /// The individual members (constants) of the enum.
    pub members: Vec<EnumMember>,
    /// Source location of the enum declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Enum member.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumMember {
    /// Member name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Member value.
    pub value: Option<Expression>,
}

/// Record declaration.
///
/// Represents a `record` definition in C# (C# 9+). Records provide built-in
/// functionality for encapsulating data and supporting value-based equality.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RecordDeclaration {
    /// The name of the record.
    pub name: String,
    /// Attributes applied to the record.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `private`, `sealed`, `partial`.
    pub modifiers: Vec<String>,
    /// Members of the record.
    pub members: Vec<Member>,
    /// Generic type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// Source location of the record declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Delegate declaration.
///
/// Represents a `delegate` definition in C#. Delegates are reference types
/// that represent a method with a particular parameter list and return type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelegateDeclaration {
    /// The name of the delegate.
    pub name: String,
    /// Attributes applied to the delegate.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `public`, `internal`.
    pub modifiers: Vec<String>,
    /// The return type of the method signature.
    pub return_type: String,
    /// Generic type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// The parameters of the delegate method signature.
    pub parameters: Vec<Parameter>,
    /// Source location of the delegate declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Member declaration.
///
/// Represents various members that can be declared within a class, struct,
/// or interface.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Member {
    /// Method declaration.
    Method(MethodDeclaration),
    /// Field declaration.
    Field(FieldDeclaration),
    /// Property declaration.
    Property(PropertyDeclaration),
    /// Indexer declaration.
    Indexer(IndexerDeclaration),
    /// Constructor declaration.
    Constructor(MethodDeclaration),
    /// Event declaration.
    Event(EventDeclaration),
}

/// Method declaration.
///
/// Represents a method or constructor declaration, including its signature,
/// modifiers, and optional body.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodDeclaration {
    /// Method name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Return type.
    pub return_type: String,
    /// Type parameters.
    pub type_parameters: Vec<TypeParameter>,
    /// Parameters.
    pub parameters: Vec<Parameter>,
    /// Method body.
    pub body: Option<Vec<Statement>>,
    /// Whether it's an async method.
    pub is_async: bool,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Property declaration.
///
/// Represents a C# property with optional get and set accessors.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyDeclaration {
    /// Property name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Property type.
    pub r#type: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Get accessor.
    pub get_accessor: Option<Accessor>,
    /// Set accessor.
    pub set_accessor: Option<Accessor>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Accessor (get/set).
///
/// Represents a property or indexer accessor, which can contain a body
/// of statements or be auto-implemented.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Accessor {
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Accessor body.
    pub body: Option<Vec<Statement>>,
    /// Modifiers.
    pub modifiers: Vec<String>,
}

/// Indexer declaration.
///
/// Represents a C# indexer (`this[...]`), which allows objects to be indexed
/// like arrays.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexerDeclaration {
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Indexer type.
    pub r#type: String,
    /// Parameters.
    pub parameters: Vec<Parameter>,
    /// Get accessor.
    pub get_accessor: Option<Accessor>,
    /// Set accessor.
    pub set_accessor: Option<Accessor>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Event declaration.
///
/// Represents a C# `event` member, which provides a way for a class to notify
/// other classes when something of interest occurs.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventDeclaration {
    /// Event name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Event type.
    pub r#type: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter.
///
/// Represents a parameter in a method, constructor, or delegate signature.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub r#type: String,
    /// Modifiers (ref, out, params).
    pub modifiers: Vec<String>,
    /// Default value.
    pub default_value: Option<Expression>,
}

/// Field declaration.
///
/// Represents a field within a class or struct.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldDeclaration {
    /// Field name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Field type.
    pub r#type: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Initializer.
    pub initializer: Option<Expression>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Attribute.
///
/// Represents a C# attribute applied to a program element (class, method, etc.).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attribute {
    /// Attribute name.
    pub name: String,
    /// Argument list.
    pub arguments: Vec<Expression>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Type parameter.
///
/// Represents a generic type parameter (e.g., `T` in `List<T>`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParameter {
    /// Parameter name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Variance (in, out).
    pub variance: Option<String>,
}

/// Type parameter constraint.
///
/// Represents a constraint on a generic type parameter (e.g., `where T : class`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeParameterConstraint {
    /// Type parameter name.
    pub parameter_name: String,
    /// Constraints.
    pub constraints: Vec<String>,
}

/// Statement.
///
/// Represents various C# statements that can appear within a method body or block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Expression statement.
    Expression(Expression),
    /// Return statement.
    Return(Option<Expression>),
    /// Block statement.
    Block(Vec<Statement>),
    /// If statement.
    If {
        /// The condition expression.
        condition: Expression,
        /// The then branch statement.
        then_branch: Box<Statement>,
        /// The optional else branch statement.
        else_branch: Option<Box<Statement>>,
    },
    /// While loop.
    While {
        /// The condition expression.
        condition: Expression,
        /// The loop body.
        body: Box<Statement>,
    },
    /// For loop.
    For {
        /// The initializer statement.
        init: Option<Box<Statement>>,
        /// The condition expression.
        condition: Option<Expression>,
        /// The update expression.
        update: Option<Expression>,
        /// The loop body.
        body: Box<Statement>,
    },
    /// Foreach loop.
    Foreach {
        /// The item type name.
        item_type: String,
        /// The item variable name.
        item_name: String,
        /// The iterable expression.
        iterable: Expression,
        /// The loop body.
        body: Box<Statement>,
    },
    /// Local variable declaration.
    LocalVariable {
        /// The variable type name.
        r#type: String,
        /// The variable name.
        name: String,
        /// The optional initializer expression.
        initializer: Option<Expression>,
    },
    /// Break.
    Break,
    /// Continue.
    Continue,
}

/// Expression.
///
/// Represents various C# expressions that can be evaluated to a value.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    /// Literal.
    Literal(Literal),
    /// Identifier.
    Identifier(String),
    /// Method call.
    MethodCall(MethodCall),
    /// Member access.
    MemberAccess(MemberAccess),
    /// Element access.
    ElementAccess(ElementAccess),
    /// New expression.
    New(NewExpression),
    /// This expression.
    This,
    /// Base expression.
    Base,
    /// Binary expression.
    Binary {
        /// The left operand.
        left: Box<Expression>,
        /// The operator string.
        op: String,
        /// The right operand.
        right: Box<Expression>,
    },
    /// Unary expression.
    Unary {
        /// The operator string.
        op: String,
        /// The operand expression.
        expression: Box<Expression>,
    },
    /// Assignment expression.
    Assignment {
        /// The left-hand side expression.
        left: Box<Expression>,
        /// The operator string.
        op: String,
        /// The right-hand side expression.
        right: Box<Expression>,
    },
    /// Await expression.
    Await(Box<Expression>),
    /// LINQ query expression.
    Query(Box<QueryExpression>),
}

/// LINQ query expression.
///
/// Represents a LINQ query (e.g., `from x in items where x > 0 select x`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryExpression {
    /// From clause.
    pub from_clause: FromClause,
    /// Query body.
    pub body: QueryBody,
}

/// From clause.
///
/// Represents a `from` clause in a LINQ query.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FromClause {
    /// Identifier.
    pub identifier: String,
    /// Expression.
    pub expression: Box<Expression>,
}

/// Query body.
///
/// Represents the body of a LINQ query, containing clauses and a select/group clause.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryBody {
    /// Query clauses.
    pub clauses: Vec<QueryClause>,
    /// Select or group clause.
    pub select_or_group: SelectOrGroupClause,
    /// Continuation (into).
    pub continuation: Option<String>,
}

/// Query clause.
///
/// Represents a clause within a LINQ query body.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryClause {
    /// From clause.
    From(FromClause),
    /// Let clause.
    Let(LetClause),
    /// Where clause.
    Where(Expression),
    /// Join clause.
    Join(JoinClause),
    /// OrderBy clause.
    OrderBy(Vec<Ordering>),
}

/// Query clause extension.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryClauseExt {
    /// GroupBy clause.
    GroupBy(Expression),
}

/// Let clause.
///
/// Represents a `let` clause in a LINQ query, used to store sub-expression results.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetClause {
    /// Identifier.
    pub identifier: String,
    /// Expression.
    pub expression: Expression,
}

/// Join clause.
///
/// Represents a `join` clause in a LINQ query.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JoinClause {
    /// Identifier.
    pub identifier: String,
    /// In expression.
    pub in_expression: Expression,
    /// On expression.
    pub on_expression: Expression,
    /// Equals expression.
    pub equals_expression: Expression,
    /// Into identifier.
    pub into_identifier: Option<String>,
}

/// Ordering.
///
/// Represents an `orderby` criterion in a LINQ query.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ordering {
    /// Expression.
    pub expression: Expression,
    /// Whether it's ascending.
    pub ascending: bool,
}

/// Select or group clause.
///
/// Represents the final `select` or `group` clause of a LINQ query.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelectOrGroupClause {
    /// Select clause.
    Select(Expression),
    /// Group clause.
    Group {
        /// Expression.
        expression: Expression,
        /// By expression.
        by_expression: Expression,
    },
}

/// New expression.
///
/// Represents an object creation expression (e.g., `new MyClass(args)`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewExpression {
    /// Type name.
    pub r#type: String,
    /// Argument list.
    pub arguments: Vec<Expression>,
}

/// Literal.
///
/// Represents a constant value of a primitive type.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// Integer.
    Integer(i64),
    /// String.
    String(String),
    /// Boolean.
    Boolean(bool),
    /// Null.
    Null,
}

/// Member access.
///
/// Represents accessing a member of an object (e.g., `obj.Member`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemberAccess {
    /// Target expression.
    pub target: Box<Expression>,
    /// Member name.
    pub name: String,
}

/// Method call.
///
/// Represents a method invocation (e.g., `target.Method(args)` or `Method(args)`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodCall {
    /// Target expression.
    pub target: Option<Box<Expression>>,
    /// Method name.
    pub name: String,
    /// Argument list.
    pub arguments: Vec<Expression>,
}

/// Element access (indexer).
///
/// Represents an element access via indexers (e.g., `array[index]`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElementAccess {
    /// Target expression.
    pub target: Box<Expression>,
    /// Argument list.
    pub arguments: Vec<Expression>,
}
