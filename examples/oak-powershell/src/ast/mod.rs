#![doc = include_str!("readme.md")]

/// Root node of the PowerShell AST.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellRoot {
    /// Top-level items in the script.
    pub items: Vec<PowerShellItem>,
}

/// A top-level item in a PowerShell script.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellItem {
    /// A statement.
    Statement(PowerShellStatement),
    /// A function definition.
    Function(PowerShellFunction),
    /// A class definition.
    Class(PowerShellClass),
    /// A workflow definition.
    Workflow(PowerShellWorkflow),
}

/// A PowerShell statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellStatement {
    /// An expression statement.
    Expression(Box<PowerShellExpression>),
    /// An assignment statement.
    Assignment(PowerShellAssignment),
    /// An `if` statement.
    If(PowerShellIf),
    /// A `while` statement.
    While(PowerShellWhile),
    /// A `for` statement.
    For(PowerShellFor),
    /// A `foreach` statement.
    ForEach(PowerShellForEach),
    /// A `switch` statement.
    Switch(PowerShellSwitch),
    /// A `try` statement.
    Try(PowerShellTry),
    /// A `return` statement.
    Return(PowerShellReturn),
    /// A `break` statement.
    Break(PowerShellBreak),
    /// A `continue` statement.
    Continue(PowerShellContinue),
    /// An `exit` statement.
    Exit(PowerShellExit),
    /// A `throw` statement.
    Throw(PowerShellThrow),
    /// A code block.
    Block(PowerShellBlock),
}

/// A PowerShell expression.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellExpression {
    /// A literal value.
    Literal(PowerShellLiteral),
    /// A variable.
    Variable(PowerShellVariable),
    /// A command call.
    Command(PowerShellCommand),
    /// A pipeline.
    Pipeline(PowerShellPipeline),
    /// A binary operation.
    Binary(PowerShellBinaryOp),
    /// A unary operation.
    Unary(PowerShellUnaryOp),
    /// A member access.
    Member(PowerShellMemberAccess),
    /// An index access.
    Index(PowerShellIndexAccess),
    /// A subexpression in parentheses.
    Subexpression(Box<PowerShellExpression>),
    /// An array literal.
    Array(PowerShellArray),
    /// A hashtable literal.
    Hashtable(PowerShellHashtable),
    /// A script block.
    ScriptBlock(PowerShellScriptBlock),
}

/// A PowerShell literal value.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellLiteral {
    /// A string literal.
    String(String),
    /// A number literal.
    Number(String),
    /// A boolean literal.
    Boolean(bool),
    /// A null literal.
    Null,
}

/// A PowerShell variable.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellVariable {
    /// Name of the variable.
    pub name: String,
    /// Optional scope of the variable.
    pub scope: Option<String>,
}

/// A PowerShell command.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellCommand {
    /// Name of the command.
    pub name: String,
    /// Arguments passed to the command.
    pub arguments: Vec<PowerShellArgument>,
}

/// A PowerShell command argument.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellArgument {
    /// A positional argument.
    Positional(Box<PowerShellExpression>),
    /// A named argument.
    Named(String, Box<PowerShellExpression>),
    /// A switch parameter.
    Switch(String),
}

/// A PowerShell pipeline.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellPipeline {
    /// Commands in the pipeline.
    pub commands: Vec<PowerShellCommand>,
}

/// A binary operation in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellBinaryOp {
    /// Left operand.
    pub left: Box<PowerShellExpression>,
    /// Binary operator.
    pub operator: PowerShellBinaryOperator,
    /// Right operand.
    pub right: Box<PowerShellExpression>,
}

/// A binary operator in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellBinaryOperator {
    /// Addition `+`.
    Add,
    /// Subtraction `-`.
    Subtract,
    /// Multiplication `*`.
    Multiply,
    /// Division `/`.
    Divide,
    /// Modulo `%`.
    Modulo,
    /// Equality `-eq`.
    Equal,
    /// Inequality `-ne`.
    NotEqual,
    /// Less than `-lt`.
    Less,
    /// Less than or equal `-le`.
    LessEqual,
    /// Greater than `-gt`.
    Greater,
    /// Greater than or equal `-ge`.
    GreaterEqual,
    /// Pattern match `-like`.
    Like,
    /// Pattern non-match `-notlike`.
    NotLike,
    /// Regex match `-match`.
    Match,
    /// Regex non-match `-notmatch`.
    NotMatch,
    /// Collection contains `-contains`.
    Contains,
    /// Collection does not contain `-notcontains`.
    NotContains,
    /// Item in collection `-in`.
    In,
    /// Item not in collection `-notin`.
    NotIn,
    /// Logical AND `-and`.
    And,
    /// Logical OR `-or`.
    Or,
    /// Logical XOR `-xor`.
    Xor,
    /// Bitwise AND `-band`.
    BitwiseAnd,
    /// Bitwise OR `-bor`.
    BitwiseOr,
    /// Bitwise XOR `-bxor`.
    BitwiseXor,
}

/// A unary operation in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellUnaryOp {
    /// Unary operator.
    pub operator: PowerShellUnaryOperator,
    /// Operand.
    pub operand: Box<PowerShellExpression>,
}

/// A unary operator in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellUnaryOperator {
    /// Positive `+`.
    Plus,
    /// Negative `-`.
    Minus,
    /// Logical NOT `-not`.
    Not,
    /// Bitwise NOT `-bnot`.
    BitwiseNot,
    /// Pre-increment `++$x`.
    Increment,
    /// Pre-decrement `--$x`.
    Decrement,
}

/// A member access in PowerShell (e.g., `$obj.Prop`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellMemberAccess {
    /// Object being accessed.
    pub object: Box<PowerShellExpression>,
    /// Name of the member.
    pub member: String,
}

/// An index access in PowerShell (e.g., `$arr[0]`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellIndexAccess {
    /// Object being indexed.
    pub object: Box<PowerShellExpression>,
    /// Index value.
    pub index: Box<PowerShellExpression>,
}

/// An array literal in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellArray {
    /// Elements of the array.
    pub elements: Vec<PowerShellExpression>,
}

/// A hashtable literal in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellHashtable {
    /// Entries in the hashtable.
    pub entries: Vec<PowerShellHashtableEntry>,
}

/// An entry in a PowerShell hashtable.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellHashtableEntry {
    /// Key of the entry.
    pub key: Box<PowerShellExpression>,
    /// Value of the entry.
    pub value: Box<PowerShellExpression>,
}

/// A script block in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellScriptBlock {
    /// Statements in the script block.
    pub statements: Vec<PowerShellStatement>,
}

/// An assignment operation in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellAssignment {
    /// Target of the assignment.
    pub target: Box<PowerShellExpression>,
    /// Assignment operator.
    pub operator: PowerShellAssignmentOperator,
    /// Value being assigned.
    pub value: Box<PowerShellExpression>,
}

/// An assignment operator in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellAssignmentOperator {
    /// Simple assignment `=`.
    Assign,
    /// Addition assignment `+=`.
    PlusAssign,
    /// Subtraction assignment `-=`.
    MinusAssign,
    /// Multiplication assignment `*=`.
    MultiplyAssign,
    /// Division assignment `/=`.
    DivideAssign,
    /// Modulo assignment `%=`.
    ModuloAssign,
}

/// An `if` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellIf {
    /// Condition to check.
    pub condition: Box<PowerShellExpression>,
    /// Block to execute if condition is true.
    pub then_block: PowerShellScriptBlock,
    /// Optional `elseif` blocks.
    pub elseif_blocks: Vec<PowerShellElseIf>,
    /// Optional `else` block.
    pub else_block: Option<PowerShellScriptBlock>,
}

/// An `elseif` clause in a PowerShell `if` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellElseIf {
    /// Condition to check.
    pub condition: Box<PowerShellExpression>,
    /// Block to execute if condition is true.
    pub block: PowerShellScriptBlock,
}

/// A `while` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellWhile {
    /// Condition to check.
    pub condition: Box<PowerShellExpression>,
    /// Block to execute while condition is true.
    pub block: PowerShellScriptBlock,
}

/// A `for` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellFor {
    /// Optional initialization expression.
    pub init: Option<Box<PowerShellExpression>>,
    /// Optional condition expression.
    pub condition: Option<Box<PowerShellExpression>>,
    /// Optional update expression.
    pub update: Option<Box<PowerShellExpression>>,
    /// Block to execute in each iteration.
    pub block: PowerShellScriptBlock,
}

/// A `foreach` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellForEach {
    /// Loop variable.
    pub variable: PowerShellVariable,
    /// Collection to iterate over.
    pub collection: Box<PowerShellExpression>,
    /// Block to execute for each element.
    pub block: PowerShellScriptBlock,
}

/// A `switch` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellSwitch {
    /// Expression to switch on.
    pub expression: Box<PowerShellExpression>,
    /// Cases to check.
    pub cases: Vec<PowerShellSwitchCase>,
    /// Optional default block.
    pub default: Option<PowerShellScriptBlock>,
}

/// A case clause in a PowerShell `switch` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellSwitchCase {
    /// Pattern to match.
    pub pattern: Box<PowerShellExpression>,
    /// Block to execute if pattern matches.
    pub block: PowerShellScriptBlock,
}

/// A `try` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellTry {
    /// Block to try executing.
    pub block: PowerShellScriptBlock,
    /// Optional `catch` blocks.
    pub catch_blocks: Vec<PowerShellCatch>,
    /// Optional `finally` block.
    pub finally_block: Option<PowerShellScriptBlock>,
}

/// A `catch` clause in a PowerShell `try` statement.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellCatch {
    /// Optional exception type to catch.
    pub exception_type: Option<String>,
    /// Block to execute if exception matches.
    pub block: PowerShellScriptBlock,
}

/// A `return` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellReturn {
    /// Optional value to return.
    pub value: Option<Box<PowerShellExpression>>,
}

/// A `break` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellBreak {
    /// Optional label to break to.
    pub label: Option<String>,
}

/// A `continue` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellContinue {
    /// Optional label to continue to.
    pub label: Option<String>,
}

/// An `exit` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellExit {
    /// Optional exit code.
    pub code: Option<Box<PowerShellExpression>>,
}

/// A `throw` statement in PowerShell.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellThrow {
    /// Optional exception expression to throw.
    pub exception: Option<Box<PowerShellExpression>>,
}

/// A PowerShell code block.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellBlock {
    /// Statements in the block.
    pub statements: Vec<PowerShellStatement>,
}

/// A PowerShell function definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellFunction {
    /// Name of the function.
    pub name: String,
    /// Body of the function.
    pub body: PowerShellScriptBlock,
    /// Attributes applied to the function.
    pub attributes: Vec<PowerShellAttribute>,
}

/// A PowerShell parameter block (e.g., `param(...)`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellParamBlock {
    /// Parameters in the block.
    pub parameters: Vec<PowerShellParameter>,
}

/// A PowerShell parameter definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellParameter {
    /// Name of the parameter.
    pub name: String,
    /// Optional type of the parameter.
    pub param_type: Option<String>,
    /// Optional default value of the parameter.
    pub default_value: Option<Box<PowerShellExpression>>,
    /// Attributes applied to the parameter.
    pub attributes: Vec<PowerShellAttribute>,
}

/// A PowerShell attribute (e.g., `[CmdletBinding()]`).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellAttribute {
    /// Name of the attribute.
    pub name: String,
    /// Arguments passed to the attribute.
    pub arguments: Vec<PowerShellExpression>,
}

/// A PowerShell class definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellClass {
    /// Name of the class.
    pub name: String,
    /// Optional base class.
    pub base_class: Option<String>,
    /// Members of the class.
    pub members: Vec<PowerShellClassMember>,
}

/// A member of a PowerShell class.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PowerShellClassMember {
    /// A property.
    Property(PowerShellProperty),
    /// A method.
    Method(PowerShellMethod),
    /// A constructor.
    Constructor(PowerShellConstructor),
}

/// A property in a PowerShell class.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellProperty {
    /// Name of the property.
    pub name: String,
    /// Optional type of the property.
    pub property_type: Option<String>,
    /// Optional default value of the property.
    pub default_value: Option<Box<PowerShellExpression>>,
    /// Attributes applied to the property.
    pub attributes: Vec<PowerShellAttribute>,
}

/// A method in a PowerShell class.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellMethod {
    /// Name of the method.
    pub name: String,
    /// Optional return type of the method.
    pub return_type: Option<String>,
    /// Parameters of the method.
    pub parameters: Vec<PowerShellParameter>,
    /// Body of the method.
    pub body: PowerShellScriptBlock,
    /// Attributes applied to the method.
    pub attributes: Vec<PowerShellAttribute>,
}

/// A constructor in a PowerShell class.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellConstructor {
    /// Parameters of the constructor.
    pub parameters: Vec<PowerShellParameter>,
    /// Body of the constructor.
    pub body: PowerShellScriptBlock,
}

/// A PowerShell workflow definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowerShellWorkflow {
    /// Name of the workflow.
    pub name: String,
    /// Parameters of the workflow.
    pub parameters: Vec<PowerShellParameter>,
    /// Body of the workflow.
    pub body: PowerShellScriptBlock,
}
