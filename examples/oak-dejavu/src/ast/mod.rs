#![doc = include_str!("readme.md")]

use crate::lexer::token_type::DejavuSyntaxKind;
use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an identifier in Dejavu source code.
///
/// Identifiers are names used for variables, functions, types, and other named entities.
/// Each identifier carries its textual representation and source location information.
///
/// # Examples
///
/// ```rust
/// #![feature(new_range_api)]
/// use core::range::Range;
/// use oak_dejavu::ast::Identifier;
///
/// let ident = Identifier { name: "main".to_string(), span: (0..4).into() }
/// assert_eq!(ident.name, "main");
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Identifier {
    /// The textual name of the identifier
    pub name: String,
    /// Source code span where this identifier appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a sequence of identifiers separated by double colons.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NamePath {
    /// The list of identifiers in the path
    pub parts: Vec<Identifier>,
    /// Source code span where this name path appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an attribute/annotation in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribute {
    /// The name of the attribute
    pub name: Identifier,
    /// Optional arguments to the attribute
    pub args: Vec<Expr>,
    /// Source code span where this attribute appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Strongly-typed AST root node representing the entire Dejavu source file.
///
/// This is the top-level structure that contains all items (functions, statements, etc.)
/// parsed from a Dejavu source file.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DejavuRoot {
    /// Collection of top-level items in the Dejavu file
    pub items: Vec<Item>,
}

/// Top-level items that can appear in a Dejavu source file.
///
/// These represent the main constructs that can exist at the module level,
/// such as function definitions and statements.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    /// A standalone statement
    Statement(Statement),
    /// A namespace definition
    Namespace(Namespace),
    /// An import statement
    Using(Using),
    /// A class definition
    Class(Class),
    /// A flags definition
    Flags(Flags),
    /// An enums definition
    Enums(Enums),
    /// A trait definition
    Trait(Trait),
    /// A widget definition
    Widget(Widget),
    /// A type function definition
    TypeFunction(TypeFunction),
    /// A micro definition
    Micro(MicroDefinition),
    /// An effect definition
    Effect(EffectDefinition),
    /// A variant definition (for enums and flags)
    Variant(Variant),
    /// A template text block
    TemplateText {
        /// The text content
        content: String,
        /// Source code span where this text appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A template control block (<% ... %>)
    TemplateControl {
        /// The items within the control block
        items: Vec<Item>,
        /// Source code span where this control block appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A template interpolation block ({ ... })
    TemplateInterpolation {
        /// The expression to interpolate
        expr: Expr,
        /// Source code span where this interpolation block appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl Item {
    /// Returns the source code span of this item.
    pub fn span(&self) -> Range<usize> {
        match self {
            Item::Statement(s) => s.span(),
            Item::Namespace(n) => n.span,
            Item::Using(u) => u.span,
            Item::Class(c) => c.span,
            Item::Flags(f) => f.span,
            Item::Enums(e) => e.span,
            Item::Trait(t) => t.span,
            Item::Widget(w) => w.span,
            Item::TypeFunction(t) => t.span,
            Item::Micro(m) => m.span,
            Item::Effect(e) => e.span,
            Item::Variant(v) => v.span,
            Item::TemplateText { span, .. } => *span,
            Item::TemplateControl { span, .. } => *span,
            Item::TemplateInterpolation { span, .. } => *span,
        }
    }
}

/// Represents a variant definition (for enums and flags) in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Variant {
    /// The name identifier of the variant
    pub name: Identifier,
    /// The list of annotations for this variant
    pub annotations: Vec<Attribute>,
    /// Optional value for the variant (e.g., in flags)
    pub value: Option<Expr>,
    /// Source code span where this variant appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a type function definition (mezzo) in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeFunction {
    /// The name identifier of the type function
    pub name: Identifier,
    /// The list of annotations for this type function
    pub annotations: Vec<Attribute>,
    /// Parameters of the type function
    pub params: Vec<Param>,
    /// The body of the type function
    pub body: Block,
    /// Source code span where this type function appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a class definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class {
    /// The name identifier of the class
    pub name: Identifier,
    /// The list of annotations for this class
    pub annotations: Vec<Attribute>,
    /// Parent classes/traits
    pub parents: Vec<NamePath>,
    /// List of items within the class
    pub items: Vec<Item>,
    /// Source code span where this class appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a flags definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Flags {
    /// The name identifier of the flags
    pub name: Identifier,
    /// The list of annotations for this flags
    pub annotations: Vec<Attribute>,
    /// List of items within the flags
    pub items: Vec<Item>,
    /// Source code span where this flags appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an enums definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Enums {
    /// The name identifier of the enums
    pub name: Identifier,
    /// The list of annotations for this enums
    pub annotations: Vec<Attribute>,
    /// List of items within the enums
    pub items: Vec<Item>,
    /// Source code span where this enums appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a trait definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trait {
    /// The name identifier of the trait
    pub name: Identifier,
    /// The list of annotations for this trait
    pub annotations: Vec<Attribute>,
    /// Parent traits
    pub parents: Vec<NamePath>,
    /// List of items within the trait
    pub items: Vec<Item>,
    /// Source code span where this trait appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a widget definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Widget {
    /// The name identifier of the widget
    pub name: Identifier,
    /// The list of annotations for this widget
    pub annotations: Vec<Attribute>,
    /// List of items within the widget
    pub items: Vec<Item>,
    /// Source code span where this widget appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a namespace definition in Dejavu source code.
///
/// Namespaces are used to organize code and prevent naming conflicts.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Namespace {
    /// The name path of the namespace
    pub name: NamePath,
    /// The list of annotations for this namespace
    pub annotations: Vec<Attribute>,
    /// List of items within the namespace
    pub items: Vec<Item>,
    /// Source code span where this namespace appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an import statement in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Using {
    /// The name path being imported
    pub path: NamePath,
    /// Source code span where this using statement appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a micro definition in Dejavu source code.
///
/// Micro definitions are specialized constructs in the Dejavu language.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MicroDefinition {
    /// The name identifier of the micro definition
    pub name: Identifier,
    /// The list of annotations for this micro definition
    pub annotations: Vec<Attribute>,
    /// List of function parameters
    pub params: Vec<Param>,
    /// The optional return type of the function
    pub return_type: Option<String>,
    /// The function body containing executable statements
    pub body: Block,
    /// Source code span where this micro definition appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents an effect definition in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EffectDefinition {
    /// The name identifier of the effect
    pub name: Identifier,
    /// The list of annotations for this effect
    pub annotations: Vec<Attribute>,
    /// List of items within the effect
    pub items: Vec<Item>,
    /// Source code span where this effect appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a lambda expression in Dejavu source code.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Lambda {
    /// List of function parameters
    pub params: Vec<Param>,
    /// The optional return type of the function
    pub return_type: Option<String>,
    /// The function body containing executable statements
    pub body: Block,
    /// Source code span where this lambda appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a function parameter with its type annotation.
///
/// Parameters define the inputs that a function can accept, with their respective types.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Param {
    /// The list of annotations for this parameter
    pub annotations: Vec<Attribute>,
    /// The parameter name identifier
    pub name: Identifier,
    /// The type annotation for this parameter
    pub ty: Option<String>,
    /// Source code span where this parameter appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents a block of statements enclosed in braces.
///
/// Blocks are fundamental control structures in Dejavu that group statements together.
/// They create new scopes and can be used as expressions.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
    /// Collection of statements within the block
    pub statements: Vec<Statement>,
    /// Source code span where this block appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents different types of patterns in Dejavu source code.
///
/// Statements are executable instructions that don't return values.
/// They form the body of functions and other code blocks.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    /// A variable binding statement with `let` keyword
    ///
    /// Contains the pattern, initialization expression, and source location
    Let {
        /// The list of annotations for this let statement
        annotations: Vec<Attribute>,
        /// Whether the variable is mutable
        is_mutable: bool,
        /// The pattern being bound
        pattern: Pattern,
        /// The initialization expression
        expr: Expr,
        /// Source code span where this statement appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An expression statement that may end with a semicolon
    ///
    /// Contains the expression, whether it ends with semicolon, and source location
    ExprStmt {
        /// The list of annotations for this expression statement
        annotations: Vec<Attribute>,
        /// The expression being evaluated
        expr: Expr,
        /// Whether this statement ends with a semicolon
        semi: bool,
        /// Source code span where this statement appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl Statement {
    /// Returns the source code span of this statement.
    pub fn span(&self) -> Range<usize> {
        match self {
            Statement::Let { span, .. } => *span,
            Statement::ExprStmt { span, .. } => *span,
        }
    }
}

/// Represents different types of expressions in Dejavu source code.
///
/// Expressions are code constructs that evaluate to values. They can be simple
/// like identifiers and literals, or complex like function calls and binary operations.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expr {
    /// An identifier that refers to a variable, function, or other named entity
    Ident(Identifier),
    /// A path to a symbol (e.g., A::B::f)
    Path(NamePath),
    /// A string or numeric literal value
    ///
    /// Contains the literal value as a string and its source location
    Literal {
        /// The string representation of the literal value
        value: String,
        /// Source code span where this literal appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A boolean literal (true or false)
    ///
    /// Contains the boolean value and its source location
    Bool {
        /// The boolean value
        value: bool,
        /// Source code span where this boolean appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A unary operation expression (e.g., !x, -y)
    ///
    /// Contains the operator, operand expression, and source location
    Unary {
        /// The unary operator kind
        op: DejavuSyntaxKind,
        /// The operand expression
        expr: Box<Expr>,
        /// Source code span where this unary expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A binary operation expression (e.g., x + y, a == b)
    ///
    /// Contains the left operand, operator, right operand, and source location
    Binary {
        /// The left operand expression
        left: Box<Expr>,
        /// The binary operator kind
        op: DejavuSyntaxKind,
        /// The right operand expression
        right: Box<Expr>,
        /// Source code span where this binary expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
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
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A parenthesized expression
    ///
    /// Contains the wrapped expression and source location
    Paren {
        /// The expression wrapped in parentheses
        expr: Box<Expr>,
        /// Source code span where this parenthesized expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A block expression that can be used as a value
    Block(Block),
    /// A lambda expression
    Lambda(Lambda),
    /// An anonymous class definition
    AnonymousClass {
        /// List of parent types/traits
        parents: Vec<String>,
        /// List of fields and methods
        items: Vec<Item>,
        /// Source code span where this anonymous class appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An if-else expression
    If {
        /// The optional pattern for if-let
        pattern: Option<Pattern>,
        /// The condition expression
        condition: Box<Expr>,
        /// The then-branch block
        then_branch: Block,
        /// The optional else-branch block
        else_branch: Option<Block>,
        /// Source code span where this if expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A match expression
    Match {
        /// The expression being matched
        scrutinee: Box<Expr>,
        /// List of match arms
        arms: Vec<MatchArm>,
        /// Source code span where this match expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A loop expression
    Loop {
        /// Optional loop label
        label: Option<String>,
        /// The optional pattern for while-let or for-in
        pattern: Option<Pattern>,
        /// The optional loop condition or iterable expression
        condition: Option<Box<Expr>>,
        /// The loop body block
        body: Block,
        /// Source code span where this loop expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A return expression
    Return {
        /// The optional return value
        expr: Option<Box<Expr>>,
        /// Source code span where this return expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A break expression
    Break {
        /// Optional break label
        label: Option<String>,
        /// Optional break value
        expr: Option<Box<Expr>>,
        /// Source code span where this break expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A continue expression
    Continue {
        /// Optional continue label
        label: Option<String>,
        /// Source code span where this continue expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A yield expression
    Yield {
        /// Optional yield value
        expr: Option<Box<Expr>>,
        /// Whether this is a yield-from expression
        yield_from: bool,
        /// Source code span where this yield expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A raise/throw expression
    Raise {
        /// The exception expression to raise
        expr: Box<Expr>,
        /// Source code span where this raise expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A resume expression
    Resume {
        /// The optional value to resume with
        expr: Option<Box<Expr>>,
        /// Source code span where this resume expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A raise expression
    Perform {
        /// The effect expression to raise
        expr: Box<Expr>,
        /// Source code span where this raise expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A catch expression
    Catch {
        /// Optional explicit return type (e.g., try Result<T, E> { ... })
        return_type: Option<NamePath>,
        /// The expression to try
        expr: Box<Expr>,
        /// List of catch arms
        arms: Vec<MatchArm>,
        /// Source code span where this catch expression appears
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An object creation or trailing closure call (e.g., Point { x: 1 })
    Object {
        /// The name or expression being "called" with a block
        callee: Box<Expr>,
        /// The block contents
        block: Block,
        /// Source code span
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl Expr {
    /// Returns the source code span of this expression.
    pub fn span(&self) -> Range<usize> {
        match self {
            Expr::Ident(i) => i.span.clone(),
            Expr::Path(p) => p.span.clone(),
            Expr::Literal { span, .. } => span.clone(),
            Expr::Bool { span, .. } => span.clone(),
            Expr::Unary { span, .. } => span.clone(),
            Expr::Binary { span, .. } => span.clone(),
            Expr::Call { span, .. } => span.clone(),
            Expr::Field { span, .. } => span.clone(),
            Expr::Index { span, .. } => span.clone(),
            Expr::Paren { span, .. } => span.clone(),
            Expr::Block(b) => b.span.clone(),
            Expr::Lambda(l) => l.span.clone(),
            Expr::AnonymousClass { span, .. } => span.clone(),
            Expr::If { span, .. } => span.clone(),
            Expr::Match { span, .. } => span.clone(),
            Expr::Loop { span, .. } => span.clone(),
            Expr::Return { span, .. } => span.clone(),
            Expr::Break { span, .. } => span.clone(),
            Expr::Continue { span, .. } => span.clone(),
            Expr::Yield { span, .. } => span.clone(),
            Expr::Raise { span, .. } => span.clone(),
            Expr::Resume { span, .. } => span.clone(),
            Expr::Perform { span, .. } => span.clone(),
            Expr::Catch { span, .. } => span.clone(),
            Expr::Object { span, .. } => span.clone(),
        }
    }
}

/// Represents an arm in a match or catch expression.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatchArm {
    /// The pattern to match
    pub pattern: Pattern,
    /// Optional guard expression
    pub guard: Option<Expr>,
    /// The body expression
    pub body: Expr,
    /// Source code span where this match arm appears
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Represents different types of patterns in Dejavu.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pattern {
    /// A wildcard pattern (_)
    Wildcard {
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A variable pattern
    Variable {
        /// The name of the variable
        name: Identifier,
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A literal pattern
    Literal {
        /// The value of the literal
        value: String,
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A type pattern (is Type)
    Type {
        /// The name of the type
        name: NamePath,
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// An else pattern
    Else {
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
    /// A class pattern (Point { x = a, y = b })
    Class {
        /// The name of the class
        name: NamePath,
        /// The fields of the class pattern
        fields: Vec<(Identifier, Pattern)>,
        /// The span of the pattern
        #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
        span: Range<usize>,
    },
}

impl Pattern {
    /// Returns the source code span of this pattern.
    pub fn span(&self) -> Range<usize> {
        match self {
            Pattern::Wildcard { span, .. } => *span,
            Pattern::Variable { span, .. } => *span,
            Pattern::Literal { span, .. } => *span,
            Pattern::Type { span, .. } => *span,
            Pattern::Else { span, .. } => *span,
            Pattern::Class { span, .. } => *span,
        }
    }
}
