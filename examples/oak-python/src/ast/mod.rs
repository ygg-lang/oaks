#![doc = include_str!("readme.md")]
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Root node of a Python source file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct PythonRoot {
    /// The program structure
    pub program: Program,
    /// Source code span
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// A Python program consisting of a list of statements.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// List of statements in the program
    pub statements: Vec<Statement>,
}

/// Represents a Python statement.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Function definition
    FunctionDef {
        /// Decorators applied to the function
        decorators: Vec<Expression>,
        /// Function name
        name: String,
        /// List of parameters
        parameters: Vec<Parameter>,
        /// Optional return type annotation
        return_type: Option<Type>,
        /// Function body
        body: Vec<Statement>,
    },
    /// Async function definition
    AsyncFunctionDef {
        /// Decorators applied to the function
        decorators: Vec<Expression>,
        /// Function name
        name: String,
        /// List of parameters
        parameters: Vec<Parameter>,
        /// Optional return type annotation
        return_type: Option<Type>,
        /// Function body
        body: Vec<Statement>,
    },
    /// Class definition
    ClassDef {
        /// Decorators applied to the class
        decorators: Vec<Expression>,
        /// Class name
        name: String,
        /// Base classes
        bases: Vec<Expression>,
        /// Class body
        body: Vec<Statement>,
    },
    /// Variable assignment
    Assignment {
        /// Target expression
        target: Expression,
        /// Value expression
        value: Expression,
    },
    /// Augmented assignment (e.g., `+=`, `-=`)
    AugmentedAssignment {
        /// Target expression
        target: Expression,
        /// Augmented operator
        operator: AugmentedOperator,
        /// Value expression
        value: Expression,
    },
    /// Expression statement
    Expression(Expression),
    /// Return statement
    Return(Option<Expression>),
    /// If statement
    If {
        /// Test expression
        test: Expression,
        /// Body of the if block
        body: Vec<Statement>,
        /// Else block (or empty)
        orelse: Vec<Statement>,
    },
    /// For loop
    For {
        /// Loop target
        target: Expression,
        /// Iterable expression
        iter: Expression,
        /// Loop body
        body: Vec<Statement>,
        /// Else block (or empty)
        orelse: Vec<Statement>,
    },
    /// Async for loop
    AsyncFor {
        /// Loop target
        target: Expression,
        /// Iterable expression
        iter: Expression,
        /// Loop body
        body: Vec<Statement>,
        /// Else block (or empty)
        orelse: Vec<Statement>,
    },
    /// While loop
    While {
        /// Test expression
        test: Expression,
        /// Loop body
        body: Vec<Statement>,
        /// Else block (or empty)
        orelse: Vec<Statement>,
    },
    /// Break statement
    Break,
    /// Continue statement
    Continue,
    /// Pass statement
    Pass,
    /// Import statement
    Import {
        /// List of names being imported
        names: Vec<ImportName>,
    },
    /// From-import statement
    ImportFrom {
        /// Optional module name
        module: Option<String>,
        /// List of names being imported
        names: Vec<ImportName>,
    },
    /// Global statement
    Global {
        /// List of global names
        names: Vec<String>,
    },
    /// Nonlocal statement
    Nonlocal {
        /// List of nonlocal names
        names: Vec<String>,
    },
    /// Try statement
    Try {
        /// Try body
        body: Vec<Statement>,
        /// Exception handlers
        handlers: Vec<ExceptHandler>,
        /// Else block
        orelse: Vec<Statement>,
        /// Finally block
        finalbody: Vec<Statement>,
    },
    /// Raise statement
    Raise {
        /// Optional exception
        exc: Option<Expression>,
        /// Optional cause
        cause: Option<Expression>,
    },
    /// With statement
    With {
        /// With items
        items: Vec<WithItem>,
        /// With body
        body: Vec<Statement>,
    },
    /// Async with statement
    AsyncWith {
        /// With items
        items: Vec<WithItem>,
        /// With body
        body: Vec<Statement>,
    },
    /// Assert statement
    Assert {
        /// Test expression
        test: Expression,
        /// Optional error message
        msg: Option<Expression>,
    },
    /// Match statement
    Match {
        /// Subject expression
        subject: Expression,
        /// Match cases
        cases: Vec<MatchCase>,
    },
}

/// Represents a case in a match statement.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    /// Pattern to match
    pub pattern: Pattern,
    /// Optional guard expression
    pub guard: Option<Expression>,
    /// Case body
    pub body: Vec<Statement>,
}

/// Represents a pattern in a match case.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Value pattern
    Value(Expression),
    /// Wildcard pattern
    Wildcard,
    /// As pattern
    As {
        /// Optional sub-pattern
        pattern: Option<Box<Pattern>>,
        /// Target name
        name: String,
    },
    /// Sequence pattern
    Sequence(Vec<Pattern>),
    /// Mapping pattern
    Mapping {
        /// Keys to match
        keys: Vec<Expression>,
        /// Corresponding patterns
        patterns: Vec<Pattern>,
    },
    /// Class pattern
    Class {
        /// Class expression
        cls: Expression,
        /// Positional patterns
        patterns: Vec<Pattern>,
        /// Keyword names
        keywords: Vec<String>,
        /// Keyword patterns
        keyword_patterns: Vec<Pattern>,
    },
    /// Or pattern
    Or(Vec<Pattern>),
}

/// Represents a Python expression.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Literal value
    Literal(Literal),
    /// Identifier name
    Name(String),
    /// Binary operation
    BinaryOp {
        /// Left operand
        left: Box<Expression>,
        /// Binary operator
        operator: BinaryOperator,
        /// Right operand
        right: Box<Expression>,
    },
    /// Unary operation
    UnaryOp {
        /// Unary operator
        operator: UnaryOperator,
        /// Operand
        operand: Box<Expression>,
    },
    /// Boolean operation (and, or)
    BoolOp {
        /// Boolean operator
        operator: BoolOperator,
        /// List of values
        values: Vec<Expression>,
    },
    /// Comparison operation
    Compare {
        /// Leftmost operand
        left: Box<Expression>,
        /// Comparison operators
        ops: Vec<CompareOperator>,
        /// Subsequent operands
        comparators: Vec<Expression>,
    },
    /// Function call
    Call {
        /// Function being called
        func: Box<Expression>,
        /// Positional arguments
        args: Vec<Expression>,
        /// Keyword arguments
        keywords: Vec<Keyword>,
    },
    /// Attribute access
    Attribute {
        /// Base expression
        value: Box<Expression>,
        /// Attribute name
        attr: String,
    },
    /// Subscript access
    Subscript {
        /// Base expression
        value: Box<Expression>,
        /// Slice or index expression
        slice: Box<Expression>,
    },
    /// List literal
    List {
        /// List elements
        elts: Vec<Expression>,
    },
    /// Tuple literal
    Tuple {
        /// Tuple elements
        elts: Vec<Expression>,
    },
    /// Slice expression
    Slice {
        /// Optional lower bound
        lower: Option<Box<Expression>>,
        /// Optional upper bound
        upper: Option<Box<Expression>>,
        /// Optional step
        step: Option<Box<Expression>>,
    },
    /// Dictionary literal
    Dict {
        /// Optional keys
        keys: Vec<Option<Expression>>,
        /// Values
        values: Vec<Expression>,
    },
    /// Set literal
    Set {
        /// Set elements
        elts: Vec<Expression>,
    },
    /// List comprehension
    ListComp {
        /// Result expression
        elt: Box<Expression>,
        /// Generators
        generators: Vec<Comprehension>,
    },
    /// Dictionary comprehension
    DictComp {
        /// Key expression
        key: Box<Expression>,
        /// Value expression
        value: Box<Expression>,
        /// Generators
        generators: Vec<Comprehension>,
    },
    /// Set comprehension
    SetComp {
        /// Result expression
        elt: Box<Expression>,
        /// Generators
        generators: Vec<Comprehension>,
    },
    /// Generator expression
    GeneratorExp {
        /// Result expression
        elt: Box<Expression>,
        /// Generators
        generators: Vec<Comprehension>,
    },
    /// Lambda expression
    Lambda {
        /// Lambda arguments
        args: Vec<Parameter>,
        /// Lambda body
        body: Box<Expression>,
    },
    /// Conditional expression (ternary operator)
    IfExp {
        /// Test expression
        test: Box<Expression>,
        /// Body expression
        body: Box<Expression>,
        /// Else expression
        orelse: Box<Expression>,
    },
    /// f-string
    JoinedStr {
        /// f-string parts
        values: Vec<Expression>,
    },
    /// Formatted value within an f-string
    FormattedValue {
        /// Value to format
        value: Box<Expression>,
        /// Conversion type
        conversion: usize,
        /// Optional format specification
        format_spec: Option<Box<Expression>>,
    },
    /// Yield expression
    Yield(Option<Box<Expression>>),
    /// Yield from expression
    YieldFrom(Box<Expression>),
    /// Await expression
    Await(Box<Expression>),
    /// Starred expression (*args, **kwargs)
    Starred {
        /// Value being starred
        value: Box<Expression>,
        /// Whether it's a double star (**kwargs)
        is_double: bool,
    },
}

/// Represents a literal value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Bytes literal
    Bytes(Vec<u8>),
    /// Boolean literal
    Boolean(bool),
    /// None literal
    None,
}

/// Represents binary operators.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mult,
    /// `/`
    Div,
    /// `//`
    FloorDiv,
    /// `%`
    Mod,
    /// `**`
    Pow,
    /// `<<`
    LShift,
    /// `>>`
    RShift,
    /// `|`
    BitOr,
    /// `^`
    BitXor,
    /// `&`
    BitAnd,
}

/// Represents unary operators.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// `~`
    Invert,
    /// `not`
    Not,
    /// `+`
    UAdd,
    /// `-`
    USub,
}

/// Represents boolean operators.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum BoolOperator {
    /// `and`
    And,
    /// `or`
    Or,
}

/// Represents comparison operators.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum CompareOperator {
    /// `==`
    Eq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `<=`
    LtE,
    /// `>`
    Gt,
    /// `>=`
    GtE,
    /// `is`
    Is,
    /// `is not`
    IsNot,
    /// `in`
    In,
    /// `not in`
    NotIn,
}

/// Represents augmented assignment operators.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum AugmentedOperator {
    /// `+=`
    Add,
    /// `-=`
    Sub,
    /// `*=`
    Mult,
    /// `/=`
    Div,
    /// `//= `
    FloorDiv,
    /// `%=`
    Mod,
    /// `**=`
    Pow,
    /// `<<=`
    LShift,
    /// `>>=`
    RShift,
    /// `|=`
    BitOr,
    /// `^=`
    BitXor,
    /// `&=`
    BitAnd,
}

/// Represents a function parameter.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub annotation: Option<Type>,
    /// Optional default value
    pub default: Option<Expression>,
    /// Whether it's a variable positional argument (*args)
    pub is_vararg: bool,
    /// Whether it's a variable keyword argument (**kwargs)
    pub is_kwarg: bool,
}

/// Represents a type annotation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Basic type name
    Name(String),
    /// Generic type
    Generic {
        /// Type name
        name: String,
        /// Type arguments
        args: Vec<Type>,
    },
    /// Union type
    Union(Vec<Type>),
    /// Optional type
    Optional(Box<Type>),
}

/// Represents a keyword argument.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Keyword {
    /// Optional argument name
    pub arg: Option<String>,
    /// Argument value
    pub value: Expression,
}

/// Represents a comprehension in a list/dict/set/generator.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Comprehension {
    /// Target expression
    pub target: Expression,
    /// Iterable expression
    pub iter: Expression,
    /// Optional conditions
    pub ifs: Vec<Expression>,
    /// Whether it's an async comprehension
    pub is_async: bool,
}

/// Represents a name in an import statement.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ImportName {
    /// Name being imported
    pub name: String,
    /// Optional alias (asname)
    pub asname: Option<String>,
}

/// Represents an exception handler in a try statement.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptHandler {
    /// Optional exception type
    pub type_: Option<Expression>,
    /// Optional name for the exception instance
    pub name: Option<String>,
    /// Handler body
    pub body: Vec<Statement>,
}

/// Represents an item in a with statement.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct WithItem {
    /// Context manager expression
    pub context_expr: Expression,
    /// Optional variables to bind to
    pub optional_vars: Option<Expression>,
}

impl Program {
    /// Creates a new empty program.
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    /// Adds a statement to the program.
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement)
    }
}

impl Default for Program {
    /// Returns a default empty program.
    fn default() -> Self {
        Self::new()
    }
}

impl Expression {
    /// Creates a name expression.
    pub fn name(name: impl Into<String>) -> Self {
        Self::Name(name.into())
    }

    /// Creates a string literal expression.
    pub fn string(value: impl Into<String>) -> Self {
        Self::Literal(Literal::String(value.into()))
    }

    /// Creates an integer literal expression.
    pub fn integer(value: i64) -> Self {
        Self::Literal(Literal::Integer(value))
    }

    /// Creates a float literal expression.
    pub fn float(value: f64) -> Self {
        Self::Literal(Literal::Float(value))
    }

    /// Creates a boolean literal expression.
    pub fn boolean(value: bool) -> Self {
        Self::Literal(Literal::Boolean(value))
    }

    /// Creates a None literal expression.
    pub fn none() -> Self {
        Self::Literal(Literal::None)
    }
}

impl Statement {
    /// Creates a function definition statement.
    pub fn function_def(name: impl Into<String>, parameters: Vec<Parameter>, return_type: Option<Type>, body: Vec<Statement>) -> Self {
        Self::FunctionDef { decorators: Vec::new(), name: name.into(), parameters, return_type, body }
    }

    /// Creates an assignment statement.
    pub fn assignment(target: Expression, value: Expression) -> Self {
        Self::Assignment { target, value }
    }

    /// Creates an expression statement.
    pub fn expression(expr: Expression) -> Self {
        Self::Expression(expr)
    }

    /// Creates a return statement.
    pub fn return_stmt(value: Option<Expression>) -> Self {
        Self::Return(value)
    }
}
