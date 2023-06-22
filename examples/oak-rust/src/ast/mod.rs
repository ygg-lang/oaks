#![doc = include_str!("readme.md")]
use crate::lexer::RustTokenType;
use core::range::Range;
use serde::{Deserialize, Serialize};

/// Represents an identifier in Rust source code.
///
/// Identifiers are names used for variables, functions, types, and other named entities.
/// Each identifier carries its textual representation and source location information.
///
/// # Examples
///
/// ```rust,ignore
/// /// use oak_rust::ast::Identifier;
/// let ident = Identifier { name: "main".to_string(), span: 0..4 };
/// assert_eq!(ident.name, "main");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    /// The textual name of the identifier
    pub name: String,
    /// Source code span where this identifier appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Strongly-typed AST root node representing the entire Rust source file.
///
/// This is the top-level structure that contains all items (functions, statements, etc.)
/// parsed from a Rust source file.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RustRoot {
    /// Collection of top-level items in the Rust file
    pub items: Vec<Item>,
}

/// Top-level items that can appear in a Rust source file.
///
/// These represent the main constructs that can exist at the module level,
/// such as function definitions, structs, enums, modules, and other declarations.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Item {
    /// A function definition
    Function(Function),
    /// A struct definition
    Struct(Struct),
    /// An enum definition
    Enum(Enum),
    /// A module definition
    Module(Module),
    /// A use statement
    Use(UseItem),
    /// A trait definition
    Trait(Trait),
    /// An impl block
    Impl(Impl),
    /// A type alias
    TypeAlias(TypeAlias),
    /// A constant definition
    Const(Const),
    /// A static definition
    Static(Static),
    /// An extern block
    ExternBlock(ExternBlock),
}

/// Represents a function definition in Rust source code.
///
/// Functions are fundamental building blocks in Rust that encapsulate reusable logic.
/// They can have parameters, return types, and contain executable code blocks.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Function {
    /// The name identifier of the function
    pub name: Identifier,
    /// List of function parameters
    pub params: Vec<Param>,
    /// Optional return type
    pub return_type: Option<Type>,
    /// The function body containing executable statements
    pub body: Block,
    /// Generic parameters
    pub generics: Vec<String>,
    /// Whether the function is async
    pub is_async: bool,
    /// Whether the function is unsafe
    pub is_unsafe: bool,
    /// Whether the function is extern
    pub is_extern: bool,
    /// Source code span where this function appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a struct definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Struct {
    /// The name identifier of the struct
    pub name: Identifier,
    /// List of struct fields
    pub fields: Vec<Field>,
    /// Source code span where this struct appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a field in a struct definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Field {
    /// The field name identifier
    pub name: Identifier,
    /// The field type
    pub ty: Type,
    /// Source code span where this field appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents an enum definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Enum {
    /// The name identifier of the enum
    pub name: Identifier,
    /// List of enum variants
    pub variants: Vec<Variant>,
    /// Source code span where this enum appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a variant in an enum definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variant {
    /// The variant name identifier
    pub name: Identifier,
    /// Optional fields for tuple or struct variants
    pub fields: Option<Vec<Field>>,
    /// Source code span where this variant appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a module definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Module {
    /// The name identifier of the module
    pub name: Identifier,
    /// List of items within the module
    pub items: Vec<Item>,
    /// Source code span where this module appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a use statement in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UseItem {
    /// The path being imported
    pub path: String,
    /// Source code span where this use statement appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a trait definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trait {
    /// The name identifier of the trait
    pub name: Identifier,
    /// List of trait items (methods, associated types, etc.)
    pub items: Vec<TraitItem>,
    /// Source code span where this trait appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents items within a trait definition.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraitItem {
    /// A method signature
    Method(Function),
    /// An associated type
    Type(TypeAlias),
}

/// Represents an impl block in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Impl {
    /// The type being implemented for
    pub ty: Type,
    /// Optional trait being implemented
    pub trait_: Option<Type>,
    /// List of implementation items
    pub items: Vec<ImplItem>,
    /// Source code span where this impl block appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents items within an impl block.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImplItem {
    /// A method implementation
    Method(Function),
    /// An associated type implementation
    Type(TypeAlias),
    /// An associated constant
    Const(Const),
}

/// Represents a type alias in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeAlias {
    /// The alias name identifier
    pub name: Identifier,
    /// The target type
    pub ty: Type,
    /// Source code span where this type alias appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a constant definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Const {
    /// The constant name identifier
    pub name: Identifier,
    /// The constant type
    pub ty: Type,
    /// The constant value expression
    pub expr: Expr,
    /// Source code span where this constant appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a static definition in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Static {
    /// The static name identifier
    pub name: Identifier,
    /// The static type
    pub ty: Type,
    /// The static value expression
    pub expr: Expr,
    /// Whether the static is mutable
    pub mutable: bool,
    /// Source code span where this static appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a type in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    /// A path type (e.g., std::vec::Vec)
    Path(String),
    /// A reference type (e.g., &str, &mut T)
    Reference {
        /// Whether the reference is mutable
        mutable: bool,
        /// The referenced type
        ty: Box<Type>,
    },
    /// A tuple type (e.g., (i32, String))
    Tuple(Vec<Type>),
    /// An array type (e.g., [i32; 10])
    Array {
        /// The element type
        ty: Box<Type>,
        /// The array size expression
        size: Expr,
    },
    /// A slice type (e.g., [i32])
    Slice(Box<Type>),
    /// A function pointer type (e.g., fn(i32) -> String)
    Fn {
        /// Parameter types
        params: Vec<Type>,
        /// Return type
        ret: Option<Box<Type>>,
    },
    /// An inferred type (_)
    Infer,
}

/// Represents a function parameter with its type annotation.
///
/// Parameters define the inputs that a function can accept, with their respective types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Param {
    /// The parameter name identifier
    pub name: Identifier,
    /// The parameter type
    pub ty: Type,
    /// Whether the parameter is mutable
    pub is_mut: bool,
    /// Source code span where this parameter appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a block of statements enclosed in braces.
///
/// Blocks are used to group statements together and define scope boundaries.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Block {
    /// List of statements within the block
    pub statements: Vec<Statement>,
    /// Block start position
    pub block_start: usize,
    /// Block end position
    pub block_end: usize,
    /// Nested block level
    pub nested: usize,
    /// Source code span where this block appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents different types of statements in Rust source code.
///
/// Statements are executable units that perform actions or declare bindings.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Statement {
    /// A let binding statement
    Let {
        /// The variable name being bound
        name: Identifier,
        /// Optional type annotation
        ty: Option<Type>,
        /// The expression being assigned to the variable
        expr: Option<Expr>,
        /// Whether the binding is mutable
        mutable: bool,
        /// Source code span where this let statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An expression statement (expression followed by optional semicolon)
    ExprStmt {
        /// The expression being evaluated
        expr: Expr,
        /// Whether the statement ends with a semicolon
        semi: bool,
        /// Source code span where this expression statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A return statement
    Return {
        /// Optional return expression
        expr: Option<Expr>,
        /// Source code span where this return statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A break statement
    Break {
        /// Optional break expression
        expr: Option<Expr>,
        /// Source code span where this break statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A continue statement
    Continue {
        /// Source code span where this continue statement appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An item statement (item declaration within a block)
    Item(Item),
}

/// Represents different types of expressions in Rust source code.
///
/// Expressions are constructs that evaluate to values and can be used in various contexts.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Expr {
    /// An identifier expression
    Ident(Identifier),
    /// A literal expression
    Literal {
        /// The literal value as a string
        value: String,
        /// Source code span where this literal appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A boolean literal expression
    Bool {
        /// The boolean value
        value: bool,
        /// Source code span where this boolean literal appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A unary expression (e.g., !x, -x, *x, &x)
    Unary {
        /// The unary operator
        op: RustTokenType,
        /// The operand expression
        expr: Box<Expr>,
        /// Source code span where this unary expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A binary expression (e.g., x + y, x == y)
    Binary {
        /// The left operand
        left: Box<Expr>,
        /// The binary operator
        op: RustTokenType,
        /// The right operand
        right: Box<Expr>,
        /// Source code span where this binary expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A function call expression
    Call {
        /// The function being called
        callee: Box<Expr>,
        /// The arguments passed to the function
        args: Vec<Expr>,
        /// Source code span where this call expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A field access expression (e.g., obj.field)
    Field {
        /// The object being accessed
        receiver: Box<Expr>,
        /// The field being accessed
        field: Identifier,
        /// Source code span where this field expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An index expression (e.g., arr[0])
    Index {
        /// The object being indexed
        receiver: Box<Expr>,
        /// The index expression
        index: Box<Expr>,
        /// Source code span where this index expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A parenthesized expression
    Paren {
        /// The inner expression
        expr: Box<Expr>,
        /// Source code span where this parenthesized expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A block expression
    Block(Block),
    /// An if expression
    If {
        /// The condition expression
        condition: Box<Expr>,
        /// The then block
        then_block: Block,
        /// Optional else block
        else_block: Option<Block>,
        /// Source code span where this if expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A while loop expression
    While {
        /// The condition expression
        condition: Box<Expr>,
        /// The loop body
        body: Block,
        /// Source code span where this while expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A for loop expression
    For {
        /// The loop variable
        var: Identifier,
        /// The iterable expression
        iter: Box<Expr>,
        /// The loop body
        body: Block,
        /// Source code span where this for expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A loop expression
    Loop {
        /// The loop body
        body: Block,
        /// Source code span where this loop expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A match expression
    Match {
        /// The expression being matched
        expr: Box<Expr>,
        /// The match arms
        arms: Vec<MatchArm>,
        /// Source code span where this match expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A tuple expression
    Tuple {
        /// The tuple elements
        elements: Vec<Expr>,
        /// Source code span where this tuple expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// An array expression
    Array {
        /// The array elements
        elements: Vec<Expr>,
        /// Source code span where this array expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
    /// A struct expression
    Struct {
        /// The struct path
        path: String,
        /// The struct fields
        fields: Vec<FieldInit>,
        /// Source code span where this struct expression appears
        #[serde(with = "oak_core::serde_range")]
        span: Range<usize>,
    },
}

/// Represents a match arm in a match expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchArm {
    /// The pattern to match
    pub pattern: Pattern,
    /// Optional guard expression
    pub guard: Option<Expr>,
    /// The arm body expression
    pub body: Expr,
    /// Source code span where this match arm appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a pattern in pattern matching.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pattern {
    /// A wildcard pattern (_)
    Wildcard,
    /// An identifier pattern
    Ident(Identifier),
    /// A literal pattern
    Literal(String),
    /// A tuple pattern
    Tuple(Vec<Pattern>),
    /// A struct pattern
    Struct {
        /// The struct path
        path: String,
        /// The field patterns
        fields: Vec<FieldPattern>,
    },
    /// A wild pattern (alias for Wildcard)
    Wild,
}

/// Represents a field pattern in a struct pattern.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldPattern {
    /// The field name
    pub name: Identifier,
    /// The field pattern
    pub pattern: Pattern,
    /// Source code span where this field pattern appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents a field initialization in a struct expression.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FieldInit {
    /// The field name
    pub name: Identifier,
    /// The field value expression
    pub expr: Expr,
    /// Source code span where this field initialization appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}

/// Represents an extern block in Rust source code.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExternBlock {
    /// The ABI string (e.g., "C", "system")
    pub abi: Option<String>,
    /// List of items within the extern block
    pub items: Vec<Item>,
    /// Source code span where this extern block appears
    #[serde(with = "oak_core::serde_range")]
    pub span: Range<usize>,
}
