#[doc = include_str!("readme.md")]
use crate::kind::ValkyrieSyntaxKind;
use core::range::Range;
use serde::{self, Deserialize, Serialize};

/// Represents an identifier in Valkyrie source code.
///
/// Identifiers are names used for variables, functions, types, and other named entities.
/// Each identifier carries its textual representation and source location information.
///
/// # Examples
///
/// ```rust
/// #![feature(new_range_api)]
/// use core::range::Range;
/// use oak_valkyrie::ast::Identifier;
///
/// let ident = Identifier { name: "main".to_string(), span: (0..4).into() };
/// assert_eq!(ident.name, "main");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Identifier {
    /// The textual name of the identifier
    pub name: String,
    /// Source code span where this identifier appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root node representing the entire Valkyrie source file.
///
/// This is the top-level structure that contains all items (functions, statements, etc.)
/// parsed from a Valkyrie source file.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValkyrieRoot {
    /// Collection of top-level items in the Valkyrie file
    pub items: Vec<Item>,
}

/// Top-level items that can appear in a Valkyrie source file.
///
/// These represent the main constructs that can exist at the module level,
/// such as function definitions and statements.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Item {
    /// A standalone statement
    Statement(Statement),
    /// A namespace definition
    Namespace(Namespace),
    /// A class definition
    Class(Class),
    /// A widget definition
    Widget(Widget),
    /// A type function definition
    TypeFunction(TypeFunction),
    /// A micro definition
    Micro(MicroDefinition),
}

/// Represents a type function definition (mezzo) in Valkyrie source code.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeFunction {
    /// The name identifier of the type function
    pub name: Identifier,
    /// Parameters of the type function
    pub params: Vec<Param>,
    /// The body of the type function
    pub body: Block,
    /// Source code span where this type function appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a class definition in Valkyrie source code.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Class {
    /// The name identifier of the class
    pub name: Identifier,
    /// List of items within the class
    pub items: Vec<Item>,
    /// Source code span where this class appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a widget definition in Valkyrie source code.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Widget {
    /// The name identifier of the widget
    pub name: Identifier,
    /// List of items within the widget
    pub items: Vec<Item>,
    /// Source code span where this widget appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a namespace definition in Valkyrie source code.
///
/// Namespaces are used to organize code and prevent naming conflicts.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Namespace {
    /// The name identifier of the namespace
    pub name: Identifier,
    /// List of items within the namespace
    pub items: Vec<Item>,
    /// Source code span where this namespace appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a micro definition in Valkyrie source code.
///
/// Micro definitions are specialized constructs in the Valkyrie language.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MicroDefinition {
    /// The name identifier of the micro definition
    pub name: Identifier,
    /// List of function parameters
    pub params: Vec<Param>,
    /// The optional return type of the function
    pub return_type: Option<String>,
    /// The function body containing executable statements
    pub body: Block,
    /// Source code span where this micro definition appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a function parameter with its type annotation.
///
/// Parameters define the inputs that a function can accept, with their respective types.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Param {
    /// The parameter name identifier
    pub name: Identifier,
    /// The type annotation for this parameter
    pub ty: String,
    /// Source code span where this parameter appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a block of statements enclosed in braces.
///
/// Blocks are fundamental control structures in Valkyrie that group statements together.
/// They create new scopes and can be used as expressions.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Collection of statements within the block
    pub statements: Vec<Statement>,
    /// Source code span where this block appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents different types of statements in Valkyrie source code.
///
/// Statements are executable instructions that don't return values.
/// They form the body of functions and other code blocks.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// A variable binding statement with `let` keyword
    ///
    /// Contains the variable name, initialization expression, and source location
    Let {
        /// Whether the variable is mutable
        is_mutable: bool,
        /// The variable name identifier
        name: Identifier,
        /// The initialization expression
        expr: Expr,
        /// Source code span where this statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An expression statement that may end with a semicolon
    ///
    /// Contains the expression, whether it ends with semicolon, and source location
    ExprStmt {
        /// The expression being evaluated
        expr: Expr,
        /// Whether this statement ends with a semicolon
        semi: bool,
        /// Source code span where this statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

/// Represents different types of expressions in Valkyrie source code.
///
/// Expressions are code constructs that evaluate to values. They can be simple
/// like identifiers and literals, or complex like function calls and binary operations.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Expr {
    /// An identifier that refers to a variable, function, or other named entity
    Ident(Identifier),
    /// A string or numeric literal value
    ///
    /// Contains the literal value as a string and its source location
    Literal {
        /// The string representation of the literal value
        value: String,
        /// Source code span where this literal appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A boolean literal (true or false)
    ///
    /// Contains the boolean value and its source location
    Bool {
        /// The boolean value
        value: bool,
        /// Source code span where this boolean appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A unary operation expression (e.g., !x, -y)
    ///
    /// Contains the operator, operand expression, and source location
    Unary {
        /// The unary operator kind
        op: ValkyrieSyntaxKind,
        /// The operand expression
        expr: Box<Expr>,
        /// Source code span where this unary expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A binary operation expression (e.g., x + y, a == b)
    ///
    /// Contains the left operand, operator, right operand, and source location
    Binary {
        /// The left operand expression
        left: Box<Expr>,
        /// The binary operator kind
        op: ValkyrieSyntaxKind,
        /// The right operand expression
        right: Box<Expr>,
        /// Source code span where this binary expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A function call expression
    ///
    /// Contains the function being called, argument expressions, and source location
    Call {
        /// The expression that evaluates to the function being called
        callee: Box<Expr>,
        /// List of argument expressions
        args: Vec<Expr>,
        /// Source code span where this function call appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A field access expression (e.g., obj.field)
    ///
    /// Contains the object expression, field name, and source location
    Field {
        /// The expression that evaluates to the object containing the field
        receiver: Box<Expr>,
        /// The field name identifier
        field: Identifier,
        /// Source code span where this field access appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An array/slice indexing expression (e.g., arr[i])
    ///
    /// Contains the array expression, index expression, and source location
    Index {
        /// The expression that evaluates to the array or slice
        receiver: Box<Expr>,
        /// The index expression
        index: Box<Expr>,
        /// Source code span where this indexing expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A parenthesized expression
    ///
    /// Contains the wrapped expression and source location
    Paren {
        /// The expression wrapped in parentheses
        expr: Box<Expr>,
        /// Source code span where this parenthesized expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A block expression that can be used as a value
    Block(Block),
    /// An anonymous class definition
    AnonymousClass {
        /// List of parent types/traits
        parents: Vec<String>,
        /// List of fields and methods
        items: Vec<Item>,
        /// Source code span where this anonymous class appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An if-else expression
    If {
        /// The condition expression
        condition: Box<Expr>,
        /// The then-branch block
        then_branch: Block,
        /// The optional else-branch block
        else_branch: Option<Block>,
        /// Source code span where this if expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A match expression
    Match {
        /// The expression being matched
        scrutinee: Box<Expr>,
        /// List of match arms
        arms: Vec<MatchArm>,
        /// Source code span where this match expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A loop expression
    Loop {
        /// Optional loop label
        label: Option<String>,
        /// The loop body block
        body: Block,
        /// Source code span where this loop expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A return expression
    Return {
        /// The optional return value
        expr: Option<Box<Expr>>,
        /// Source code span where this return expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A break expression
    Break {
        /// Optional break label
        label: Option<String>,
        /// Optional break value
        expr: Option<Box<Expr>>,
        /// Source code span where this break expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A continue expression
    Continue {
        /// Optional continue label
        label: Option<String>,
        /// Source code span where this continue expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A yield expression
    Yield {
        /// Optional yield value
        expr: Option<Box<Expr>>,
        /// Whether this is a yield-from expression
        yield_from: bool,
        /// Source code span where this yield expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A raise/throw expression
    Raise {
        /// The exception expression to raise
        expr: Box<Expr>,
        /// Source code span where this raise expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A catch expression
    Catch {
        /// The expression to try
        expr: Box<Expr>,
        /// List of catch arms
        arms: Vec<MatchArm>,
        /// Source code span where this catch expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An object creation or trailing closure call (e.g., Point { x: 1 })
    Object {
        /// The name or expression being "called" with a block
        callee: Box<Expr>,
        /// The block contents
        block: Block,
        /// Source code span
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

/// Represents an arm in a match or catch expression.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    /// The pattern to match
    pub pattern: Pattern,
    /// Optional guard expression
    pub guard: Option<Expr>,
    /// The body expression
    pub body: Expr,
    /// Source code span where this match arm appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents different types of patterns in Valkyrie.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    /// A wildcard pattern (_)
    Wildcard {
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A variable pattern
    Variable {
        name: Identifier,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A literal pattern
    Literal {
        value: String,
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}
