#![doc = include_str!("../readme.md")]

use core::range::Range;

/// Root node of the VB.NET AST.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VbNetRoot {
    /// Items in the compilation unit.
    pub items: Vec<Item>,
}

/// Top-level items in a VB.NET program.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    /// Namespace declaration.
    Namespace(NamespaceDeclaration),
    /// Imports directive.
    Imports(ImportsDirective),
    /// Class declaration.
    Class(ClassDeclaration),
    /// Interface declaration.
    Interface(InterfaceDeclaration),
    /// Structure declaration.
    Structure(StructureDeclaration),
    /// Enum declaration.
    Enum(EnumDeclaration),
    /// Module declaration.
    Module(ModuleDeclaration),
    /// Delegate declaration.
    Delegate(DelegateDeclaration),
    /// Event declaration.
    Event(EventDeclaration),
    /// Function declaration.
    Function(FunctionDeclaration),
    /// Subroutine declaration.
    Sub(SubDeclaration),
    /// Property declaration.
    Property(PropertyDeclaration),
    /// Variable declaration.
    Variable(VariableDeclaration),
}

/// Namespace declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NamespaceDeclaration {
    /// The fully qualified name of the namespace.
    pub name: String,
    /// Items defined within this namespace.
    pub items: Vec<Item>,
    /// Source location of the entire namespace declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Imports directive.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportsDirective {
    /// The namespace or type path being imported.
    pub path: String,
    /// An optional alias for the namespace or type.
    pub alias: Option<String>,
    /// Source location of the imports directive.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Class declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// The name of the class.
    pub name: String,
    /// Attributes applied to the class.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Private`, `Shared`, `MustInherit`, `NotInheritable`.
    pub modifiers: Vec<String>,
    /// The base class and any implemented interfaces.
    pub base_types: Vec<String>,
    /// Members of the class.
    pub members: Vec<Member>,
    /// Source location of the class declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDeclaration {
    /// The name of the interface.
    pub name: String,
    /// Attributes applied to the interface.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Friend`.
    pub modifiers: Vec<String>,
    /// Members defined in the interface.
    pub members: Vec<Member>,
    /// Source location of the interface declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Structure declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureDeclaration {
    /// The name of the structure.
    pub name: String,
    /// Attributes applied to the structure.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Private`.
    pub modifiers: Vec<String>,
    /// Members of the structure.
    pub members: Vec<Member>,
    /// Source location of the structure declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Enum declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumDeclaration {
    /// The name of the enum.
    pub name: String,
    /// Attributes applied to the enum.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Friend`.
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

/// Module declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleDeclaration {
    /// The name of the module.
    pub name: String,
    /// Attributes applied to the module.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Friend`.
    pub modifiers: Vec<String>,
    /// Members of the module.
    pub members: Vec<Member>,
    /// Source location of the module declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Delegate declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelegateDeclaration {
    /// The name of the delegate.
    pub name: String,
    /// Attributes applied to the delegate.
    pub attributes: Vec<Attribute>,
    /// Modifiers like `Public`, `Friend`.
    pub modifiers: Vec<String>,
    /// The return type of the method signature.
    pub return_type: String,
    /// The parameters of the delegate method signature.
    pub parameters: Vec<Parameter>,
    /// Source location of the delegate declaration.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Event declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventDeclaration {
    /// Event name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Event type.
    pub event_type: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionDeclaration {
    /// Function name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Return type.
    pub return_type: String,
    /// Parameters.
    pub parameters: Vec<Parameter>,
    /// Function body.
    pub body: Option<Vec<Statement>>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Subroutine declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubDeclaration {
    /// Subroutine name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Parameters.
    pub parameters: Vec<Parameter>,
    /// Subroutine body.
    pub body: Option<Vec<Statement>>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Property declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyDeclaration {
    /// Property name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Property type.
    pub property_type: String,
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

/// Variable declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VariableDeclaration {
    /// Variable name.
    pub name: String,
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Variable type.
    pub variable_type: String,
    /// Modifiers.
    pub modifiers: Vec<String>,
    /// Initializer.
    pub initializer: Option<Expression>,
    /// Source location.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Member declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Member {
    /// Event declaration.
    Event(EventDeclaration),
    /// Function declaration.
    Function(FunctionDeclaration),
    /// Subroutine declaration.
    Sub(SubDeclaration),
    /// Property declaration.
    Property(PropertyDeclaration),
    /// Variable declaration.
    Variable(VariableDeclaration),
}

/// Accessor (Get/Set).
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

/// Parameter.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Parameter {
    /// Attributes.
    pub attributes: Vec<Attribute>,
    /// Parameter name.
    pub name: String,
    /// Parameter type.
    pub parameter_type: String,
    /// Modifiers (ByVal, ByRef, Optional).
    pub modifiers: Vec<String>,
    /// Default value (for Optional parameters).
    pub default_value: Option<Expression>,
}

/// Attribute.
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

/// Statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Statement {
    /// Expression statement.
    Expression(Expression),
    /// Return statement.
    Return(Option<Expression>),
    /// If statement.
    If {
        /// The condition expression.
        condition: Expression,
        /// The then branch statement.
        then_branch: Box<Statement>,
        /// The optional else branch statement.
        else_branch: Option<Box<Statement>>,
    },
    /// For loop.
    For {
        /// The loop variable.
        variable: String,
        /// The start expression.
        start: Expression,
        /// The end expression.
        end: Expression,
        /// The optional step expression.
        step: Option<Expression>,
        /// The loop body.
        body: Box<Statement>,
    },
    /// For Each loop.
    ForEach {
        /// The item variable.
        variable: String,
        /// The iterable expression.
        iterable: Expression,
        /// The loop body.
        body: Box<Statement>,
    },
    /// While loop.
    While {
        /// The condition expression.
        condition: Expression,
        /// The loop body.
        body: Box<Statement>,
    },
    /// Do While loop.
    DoWhile {
        /// The condition expression.
        condition: Expression,
        /// The loop body.
        body: Box<Statement>,
        /// Whether the condition is checked at the end.
        check_at_end: bool,
    },
    /// Select Case statement.
    SelectCase {
        /// The expression to evaluate.
        expression: Expression,
        /// The case clauses.
        cases: Vec<CaseClause>,
        /// The optional default case.
        default_case: Option<Box<Statement>>,
    },
    /// With statement.
    With {
        /// The target expression.
        target: Expression,
        /// The statements within the With block.
        statements: Vec<Statement>,
    },
    /// Try statement.
    Try {
        /// The try block statements.
        try_block: Vec<Statement>,
        /// The catch clauses.
        catch_clauses: Vec<CatchClause>,
        /// The optional finally block.
        finally_block: Option<Vec<Statement>>,
    },
    /// Dim statement (variable declaration).
    Dim(VariableDeclaration),
    /// Const statement (constant declaration).
    Const {
        /// Constant name.
        name: String,
        /// Constant type.
        constant_type: String,
        /// Constant value.
        value: Expression,
    },
    /// Exit statement.
    Exit(String),
    /// Continue statement.
    Continue(String),
    /// Block statement (multiple statements).
    Block(Vec<Statement>),
}

/// Case clause in Select Case statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseClause {
    /// The case expressions.
    expressions: Vec<Expression>,
    /// The case body.
    body: Box<Statement>,
}

/// Catch clause in Try statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatchClause {
    /// The exception variable (optional).
    variable: Option<String>,
    /// The exception type (optional).
    exception_type: Option<String>,
    /// The catch block statements.
    body: Vec<Statement>,
}

/// Expression.
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
    /// Me expression.
    Me,
    /// MyBase expression.
    MyBase,
    /// MyClass expression.
    MyClass,
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
    /// Array expression.
    Array(Vec<Expression>),
    /// Tuple expression.
    Tuple(Vec<Expression>),
    /// Parenthesized expression.
    Parenthesized(Box<Expression>),
    /// TypeOf expression.
    TypeOf(Box<Expression>, String),
    /// Is expression.
    Is(Box<Expression>, Box<Expression>),
    /// Like expression.
    Like(Box<Expression>, Box<Expression>),
    /// If expression.
    If {
        /// The condition expression.
        condition: Box<Expression>,
        /// The true expression.
        true_expression: Box<Expression>,
        /// The false expression.
        false_expression: Box<Expression>,
    },
}

/// Literal.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// Integer.
    Integer(i64),
    /// String.
    String(String),
    /// Boolean.
    Boolean(bool),
    /// Double.
    Double(f64),
    /// Date.
    Date(String),
    /// Nothing.
    Nothing,
}

/// Member access.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MemberAccess {
    /// Target expression.
    pub target: Box<Expression>,
    /// Member name.
    pub name: String,
}

/// Method call.
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
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ElementAccess {
    /// Target expression.
    pub target: Box<Expression>,
    /// Argument list.
    pub arguments: Vec<Expression>,
}

/// New expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewExpression {
    /// Type name.
    pub r#type: String,
    /// Argument list.
    pub arguments: Vec<Expression>,
}
