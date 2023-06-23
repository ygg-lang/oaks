#![doc = include_str!("readme.md")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// PHP AST root node.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpRoot {
    /// Top-level items in the PHP file.
    pub items: Vec<PhpItem>,
}

/// PHP top-level items.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpItem {
    /// Opening tag (`<?php`).
    OpenTag,
    /// Closing tag (`?>`).
    CloseTag,
    /// A PHP statement.
    Statement(PhpStatement),
    /// A function declaration.
    Function(PhpFunction),
    /// A class declaration.
    Class(PhpClass),
    /// An interface declaration.
    Interface(PhpInterface),
    /// A trait declaration.
    Trait(PhpTrait),
    /// A namespace declaration.
    Namespace(PhpNamespace),
    /// A use statement for importing symbols.
    Use(PhpUse),
}

/// PHP statements.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpStatement {
    /// An expression statement.
    Expression(PhpExpression),
    /// An `if` statement.
    If(PhpIf),
    /// A `while` loop.
    While(PhpWhile),
    /// A `for` loop.
    For(PhpFor),
    /// A `foreach` loop.
    Foreach(PhpForeach),
    /// A `switch` statement.
    Switch(PhpSwitch),
    /// A `try-catch-finally` block.
    Try(PhpTry),
    /// A `return` statement.
    Return(Option<PhpExpression>),
    /// A `break` statement.
    Break(Option<PhpExpression>),
    /// A `continue` statement.
    Continue(Option<PhpExpression>),
    /// An `echo` statement.
    Echo(Vec<PhpExpression>),
    /// A `print` statement.
    Print(PhpExpression),
    /// A `global` variable declaration.
    Global(Vec<String>),
    /// A `static` variable declaration.
    Static(Vec<PhpVariable>),
    /// An `unset` statement.
    Unset(Vec<PhpExpression>),
    /// A `declare` statement.
    Declare(Vec<PhpDeclareItem>),
    /// A block of statements enclosed in braces.
    Block(Vec<PhpStatement>),
}

/// PHP expressions.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpExpression {
    /// A literal value.
    Literal(PhpLiteral),
    /// A variable.
    Variable(PhpVariable),
    /// An array creation.
    Array(Vec<PhpArrayElement>),
    /// A function call.
    FunctionCall(PhpFunctionCall),
    /// A method call.
    MethodCall(PhpMethodCall),
    /// A property access.
    PropertyAccess(PhpPropertyAccess),
    /// An array element access.
    ArrayAccess(PhpArrayAccess),
    /// An assignment operation.
    Assignment(PhpAssignment),
    /// A binary operation.
    BinaryOp(PhpBinaryOp),
    /// A unary operation.
    UnaryOp(PhpUnaryOp),
    /// A ternary operation.
    TernaryOp(PhpTernaryOp),
    /// A type cast.
    Cast(PhpCast),
    /// An object instantiation.
    New(PhpNew),
    /// An object clone operation.
    Clone(Box<PhpExpression>),
    /// An `instanceof` check.
    Instanceof(PhpInstanceof),
    /// An `include` or `include_once` statement.
    Include(PhpInclude),
    /// A `require` or `require_once` statement.
    Require(PhpRequire),
    /// An `eval` call.
    Eval(Box<PhpExpression>),
    /// An `exit` or `die` call.
    Exit(Option<Box<PhpExpression>>),
    /// An `empty` check.
    Empty(Box<PhpExpression>),
    /// An `isset` check.
    Isset(Vec<PhpExpression>),
    /// A `list` or short array assignment.
    List(Vec<Option<PhpExpression>>),
    /// A `yield` expression for generators.
    Yield(PhpYield),
}

/// PHP literals.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpLiteral {
    /// A string literal.
    String(String),
    /// A numeric literal.
    Number(String),
    /// A boolean literal.
    Boolean(bool),
    /// The `null` literal.
    Null,
}

/// PHP variables.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpVariable {
    /// The name of the variable (including the `$` prefix).
    pub name: String,
}

/// PHP array elements.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpArrayElement {
    /// Optional key for the array element.
    pub key: Option<PhpExpression>,
    /// The value of the array element.
    pub value: PhpExpression,
}

/// PHP function call
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpFunctionCall {
    pub name: Box<PhpExpression>,
    pub arguments: Vec<PhpExpression>,
}

/// PHP method call
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpMethodCall {
    pub object: Box<PhpExpression>,
    pub method: String,
    pub arguments: Vec<PhpExpression>,
}

/// PHP property access
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpPropertyAccess {
    pub object: Box<PhpExpression>,
    pub property: String,
}

/// PHP array access
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpArrayAccess {
    pub array: Box<PhpExpression>,
    pub index: Box<PhpExpression>,
}

/// PHP assignment
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpAssignment {
    pub left: Box<PhpExpression>,
    pub operator: PhpAssignmentOp,
    pub right: Box<PhpExpression>,
}

/// PHP assignment operators.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpAssignmentOp {
    /// Standard assignment (`=`).
    Assign,
    /// Addition assignment (`+=`).
    PlusAssign,
    /// Subtraction assignment (`-=`).
    MinusAssign,
    /// Multiplication assignment (`*=`).
    MultiplyAssign,
    /// Division assignment (`/=`).
    DivideAssign,
    /// Modulo assignment (`%=`).
    ModuloAssign,
    /// Power assignment (`**=`).
    PowerAssign,
    /// Concatenation assignment (`.=`).
    ConcatAssign,
    /// Bitwise AND assignment (`&=`).
    BitwiseAndAssign,
    /// Bitwise OR assignment (`|=`).
    BitwiseOrAssign,
    /// Bitwise XOR assignment (`^=`).
    BitwiseXorAssign,
    /// Left shift assignment (`<<=`).
    LeftShiftAssign,
    /// Right shift assignment (`>>=`).
    RightShiftAssign,
    /// Null coalesce assignment (`??=`).
    NullCoalesceAssign,
}

/// PHP binary operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpBinaryOp {
    /// Left operand.
    pub left: Box<PhpExpression>,
    /// Binary operator.
    pub operator: PhpBinaryOperator,
    /// Right operand.
    pub right: Box<PhpExpression>,
}

/// PHP binary operators.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpBinaryOperator {
    /// Addition (`+`).
    Plus,
    /// Subtraction (`-`).
    Minus,
    /// Multiplication (`*`).
    Multiply,
    /// Division (`/`).
    Divide,
    /// Modulo (`%`).
    Modulo,
    /// Power (`**`).
    Power,
    /// Concatenation (`.`).
    Concat,
    /// Equality (`==`).
    Equal,
    /// Inequality (`!=`).
    NotEqual,
    /// Identity (`===`).
    Identical,
    /// Non-identity (`!==`).
    NotIdentical,
    /// Less than (`<`).
    Less,
    /// Less than or equal to (`<=`).
    LessEqual,
    /// Greater than (`>`).
    Greater,
    /// Greater than or equal to (`>=`).
    GreaterEqual,
    /// Spaceship operator (`<=>`).
    Spaceship,
    /// Logical AND (`&&` or `and`).
    LogicalAnd,
    /// Logical OR (`||` or `or`).
    LogicalOr,
    /// Logical XOR (`xor`).
    LogicalXor,
    /// Bitwise AND (`&`).
    BitwiseAnd,
    /// Bitwise OR (`|`).
    BitwiseOr,
    /// Bitwise XOR (`^`).
    BitwiseXor,
    /// Left shift (`<<`).
    LeftShift,
    /// Right shift (`>>`).
    RightShift,
    /// Null coalesce (`??`).
    NullCoalesce,
}

/// PHP unary operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpUnaryOp {
    /// Unary operator.
    pub operator: PhpUnaryOperator,
    /// Operand.
    pub operand: Box<PhpExpression>,
}

/// PHP unary operators.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpUnaryOperator {
    /// Unary plus (`+`).
    Plus,
    /// Unary minus (`-`).
    Minus,
    /// Logical NOT (`!`).
    LogicalNot,
    /// Bitwise NOT (`~`).
    BitwiseNot,
    /// Pre-increment (`++$a`).
    PreIncrement,
    /// Post-increment (`$a++`).
    PostIncrement,
    /// Pre-decrement (`--$a`).
    PreDecrement,
    /// Post-decrement (`$a--`).
    PostDecrement,
    /// Error suppression (`@`).
    ErrorSuppression,
}

/// PHP ternary operations.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpTernaryOp {
    /// Condition expression.
    pub condition: Box<PhpExpression>,
    /// Expression for true branch (optional for shorthand `?:`).
    pub true_expr: Option<Box<PhpExpression>>,
    /// Expression for false branch.
    pub false_expr: Box<PhpExpression>,
}

/// PHP type casts.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpCast {
    /// Target type for the cast.
    pub cast_type: PhpCastType,
    /// Expression to be cast.
    pub expression: Box<PhpExpression>,
}

/// PHP cast types.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpCastType {
    /// Cast to integer (`(int)`).
    Int,
    /// Cast to float (`(float)`).
    Float,
    /// Cast to string (`(string)`).
    String,
    /// Cast to boolean (`(bool)`).
    Bool,
    /// Cast to array (`(array)`).
    Array,
    /// Cast to object (`(object)`).
    Object,
    /// Cast to unset (`(unset)`).
    Unset,
}

/// PHP `new` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpNew {
    /// Class name or expression.
    pub class: Box<PhpExpression>,
    /// Constructor arguments.
    pub arguments: Vec<PhpExpression>,
}

/// PHP `instanceof` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpInstanceof {
    /// Expression to check.
    pub expression: Box<PhpExpression>,
    /// Class name or expression.
    pub class: Box<PhpExpression>,
}

/// PHP `include` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpInclude {
    /// Whether it is `include_once`.
    pub once: bool,
    /// Path to the file.
    pub path: Box<PhpExpression>,
}

/// PHP `require` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpRequire {
    /// Whether it is `require_once`.
    pub once: bool,
    /// Path to the file.
    pub path: Box<PhpExpression>,
}

/// PHP `yield` expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpYield {
    /// Optional key for yield (e.g., `yield $key => $value`).
    pub key: Option<Box<PhpExpression>>,
    /// Optional value for yield.
    pub value: Option<Box<PhpExpression>>,
    /// Whether it is a `yield from` expression.
    pub from: bool,
}

/// PHP `if` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpIf {
    /// Condition expression.
    pub condition: Box<PhpExpression>,
    /// The `then` block.
    pub then_stmt: Box<PhpStatement>,
    /// Any `elseif` blocks.
    pub elseif_stmts: Vec<PhpElseif>,
    /// Optional `else` block.
    pub else_stmt: Option<Box<PhpStatement>>,
}

/// PHP `elseif` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpElseif {
    /// Condition expression.
    pub condition: Box<PhpExpression>,
    /// The `elseif` block.
    pub statement: Box<PhpStatement>,
}

/// PHP `while` loop.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpWhile {
    /// Loop condition.
    pub condition: Box<PhpExpression>,
    /// Loop body.
    pub statement: Box<PhpStatement>,
}

/// PHP `for` loop.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpFor {
    /// Loop initialization expressions.
    pub init: Vec<PhpExpression>,
    /// Loop conditions.
    pub condition: Vec<PhpExpression>,
    /// Loop update expressions.
    pub update: Vec<PhpExpression>,
    /// Loop body.
    pub statement: Box<PhpStatement>,
}

/// PHP `foreach` loop.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpForeach {
    /// Iterable expression.
    pub iterable: Box<PhpExpression>,
    /// Optional key variable.
    pub key: Option<Box<PhpExpression>>,
    /// Value variable.
    pub value: Box<PhpExpression>,
    /// Loop body.
    pub statement: Box<PhpStatement>,
}

/// PHP `switch` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpSwitch {
    /// Expression to switch on.
    pub expression: Box<PhpExpression>,
    /// Switch cases.
    pub cases: Vec<PhpCase>,
}

/// PHP `case` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpCase {
    /// Case value (None for the `default` case).
    pub value: Option<Box<PhpExpression>>,
    /// Statements within the case.
    pub statements: Vec<PhpStatement>,
}

/// PHP `try-catch-finally` block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpTry {
    /// The `try` block.
    pub statement: Box<PhpStatement>,
    /// Catch clauses.
    pub catches: Vec<PhpCatch>,
    /// Optional `finally` block.
    pub finally_stmt: Option<Box<PhpStatement>>,
}

/// PHP `catch` clause.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpCatch {
    /// Exception types to catch.
    pub types: Vec<String>,
    /// Variable to hold the exception.
    pub variable: PhpVariable,
    /// Catch block body.
    pub statement: Box<PhpStatement>,
}

/// PHP `declare` item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpDeclareItem {
    /// Directive name.
    pub name: String,
    /// Directive value.
    pub value: Box<PhpExpression>,
}

/// PHP function declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpFunction {
    /// Function name.
    pub name: String,
    /// Function parameters.
    pub parameters: Vec<PhpParameter>,
    /// Optional return type.
    pub return_type: Option<PhpType>,
    /// Function body.
    pub body: Box<PhpStatement>,
    /// Whether the function returns by reference.
    pub by_ref: bool,
}

/// PHP function parameter.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpParameter {
    /// Parameter name.
    pub name: String,
    /// Optional type hint.
    pub param_type: Option<PhpType>,
    /// Optional default value.
    pub default: Option<Box<PhpExpression>>,
    /// Whether the parameter is passed by reference.
    pub by_ref: bool,
    /// Whether the parameter is variadic.
    pub variadic: bool,
}

/// PHP type specification.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpType {
    /// A simple named type.
    Named(String),
    /// A nullable type (e.g., `?int`).
    Nullable(Box<PhpType>),
    /// A union type (e.g., `int|string`).
    Union(Vec<PhpType>),
}

/// PHP class declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpClass {
    /// Class name.
    pub name: String,
    /// Optional parent class.
    pub extends: Option<String>,
    /// Implemented interfaces.
    pub implements: Vec<String>,
    /// Class members (properties, methods, constants, etc.).
    pub members: Vec<PhpClassMember>,
    /// Class modifiers (abstract, final, etc.).
    pub modifiers: Vec<PhpModifier>,
}

/// PHP class members.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpClassMember {
    /// A class property.
    Property(PhpProperty),
    /// A class method.
    Method(PhpMethod),
    /// A class constant.
    Constant(PhpConstant),
    /// A trait use statement.
    Use(PhpTraitUse),
}

/// PHP class property.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpProperty {
    /// Property name.
    pub name: String,
    /// Optional type hint.
    pub property_type: Option<PhpType>,
    /// Optional default value.
    pub default: Option<Box<PhpExpression>>,
    /// Access modifiers (public, private, etc.).
    pub modifiers: Vec<PhpModifier>,
}

/// PHP class method.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpMethod {
    /// Method name.
    pub name: String,
    /// Method parameters.
    pub parameters: Vec<PhpParameter>,
    /// Optional return type.
    pub return_type: Option<PhpType>,
    /// Method body (None for abstract methods).
    pub body: Option<Box<PhpStatement>>,
    /// Access and behavior modifiers.
    pub modifiers: Vec<PhpModifier>,
    /// Whether the method returns by reference.
    pub by_ref: bool,
}

/// PHP constant declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpConstant {
    /// Constant name.
    pub name: String,
    /// Constant value.
    pub value: Box<PhpExpression>,
    /// Optional access modifiers.
    pub modifiers: Vec<PhpModifier>,
}

/// PHP trait use statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpTraitUse {
    /// Traits being used.
    pub traits: Vec<String>,
    /// Adaptation rules for trait members (aliasing, precedence).
    pub adaptations: Vec<PhpTraitAdaptation>,
}

/// PHP trait adaptation rule.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpTraitAdaptation {
    /// Precedence rule (`insteadof`).
    Precedence {
        /// Method name.
        method: String,
        /// Trait name.
        trait_name: String,
        /// Traits that this method replaces.
        insteadof: Vec<String>,
    },
    /// Alias rule (`as`).
    Alias {
        /// Method name.
        method: String,
        /// Optional trait name.
        trait_name: Option<String>,
        /// New alias name.
        alias: String,
        /// Optional visibility modifier.
        modifier: Option<PhpModifier>,
    },
}

/// PHP access and behavior modifiers.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpModifier {
    /// Public visibility.
    Public,
    /// Protected visibility.
    Protected,
    /// Private visibility.
    Private,
    /// Static member.
    Static,
    /// Abstract member or class.
    Abstract,
    /// Final member or class.
    Final,
}

/// PHP interface declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpInterface {
    /// Interface name.
    pub name: String,
    /// Parent interfaces.
    pub extends: Vec<String>,
    /// Interface members (methods and constants).
    pub members: Vec<PhpInterfaceMember>,
}

/// PHP interface members.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpInterfaceMember {
    /// An interface method.
    Method(PhpMethod),
    /// An interface constant.
    Constant(PhpConstant),
}

/// PHP trait declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpTrait {
    /// Trait name.
    pub name: String,
    /// Trait members.
    pub members: Vec<PhpClassMember>,
}

/// PHP namespace declaration.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpNamespace {
    /// Optional namespace name (None for global namespace).
    pub name: Option<String>,
    /// Items within the namespace.
    pub items: Vec<PhpItem>,
}

/// PHP `use` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpUse {
    /// List of symbols being imported.
    pub uses: Vec<PhpUseItem>,
    /// Type of the `use` statement (normal, function, or constant).
    pub use_type: PhpUseType,
}

/// PHP `use` item.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhpUseItem {
    /// Fully qualified name of the symbol.
    pub name: String,
    /// Optional alias (`as`).
    pub alias: Option<String>,
}

/// PHP `use` types.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhpUseType {
    /// Normal import (class, interface, trait, or namespace).
    Normal,
    /// Function import (`use function`).
    Function,
    Const,
}
