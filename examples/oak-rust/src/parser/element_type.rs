use crate::lexer::RustTokenType;
use oak_core::{ElementType, GreenNode, UniversalElementRole};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Rust 语法树元素的类型别名
pub type RustElement<'a> = Arc<GreenNode<'a, RustElementType>>;

/// Rust 语法树中所有可能的元素类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum RustElementType {
    /// The root element of any Rust source file
    /// ```rust
    /// // Entire file content
    /// fn main() {}
    /// ```
    SourceFile,

    /// A function definition
    /// ```rust
    /// fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    /// ```
    Function,

    /// A trait definition
    /// ```rust
    /// trait Display {
    ///     fn fmt(&self) -> String;
    /// }
    /// ```
    Trait,

    /// An impl block
    /// ```rust,ignore
    /// impl Display for Point {
    ///     fn fmt(&self, f: &mut Formatter) -> Result { /* ... */ }
    /// }
    /// ```
    Impl,

    /// A use statement
    /// ```rust
    /// use std::collections::HashMap;
    /// ```
    UseItem,

    /// A static variable
    /// ```rust
    /// static COUNT: i32 = 0;
    /// ```
    Static,

    /// A constant
    /// ```rust
    /// const MAX_SIZE: usize = 100;
    /// ```
    Const,

    /// A type alias
    /// ```rust
    /// type MyResult = Result<i32, String>;
    /// ```
    TypeAlias,

    /// A macro invocation or definition
    /// ```rust,ignore
    /// println!("Hello, {}!", name);
    /// vec![1, 2, 3]
    /// ```
    Macro,

    /// An extern crate declaration
    /// ```rust
    /// extern crate serde;
    /// ```
    ExternCrate,

    /// An extern block
    /// ```rust,ignore
    /// extern "C" {
    ///     fn call_c_function();
    /// }
    /// ```
    ExternBlock,

    /// A module item
    /// ```rust,ignore
    /// mod my_module;
    /// mod my_module { /* content */ }
    /// ```
    ModuleItem,

    /// A struct definition
    /// ```rust
    /// struct Point {
    ///     x: f64,
    ///     y: f64,
    /// }
    /// ```
    StructItem,

    /// An enum definition
    /// ```rust
    /// enum Color {
    ///     Red,
    ///     Green,
    ///     Blue,
    /// }
    /// ```
    EnumItem,

    /// A return statement
    /// ```rust,ignore
    /// return 42;
    /// ```
    ReturnStatement,

    /// A parameter list in function definitions
    /// ```rust,ignore
    /// fn example(a: i32, b: String) // (a: i32, b: String)
    /// ```
    ParameterList,

    /// A single parameter in a function
    /// ```rust,ignore
    /// fn example(x: i32) // x: i32
    /// ```
    Parameter,

    /// A return type annotation
    /// ```rust,ignore
    /// fn example() -> i32 // -> i32
    /// ```
    ReturnType,

    /// A let statement
    /// ```rust,ignore
    /// let x = 42;
    /// let mut y: String = "hello".to_string();
    /// ```
    LetStatement,

    /// An expression used as a statement
    /// ```rust,ignore
    /// x + 1; // Expression statement
    /// ```
    ExpressionStatement,

    /// A block expression
    /// ```rust,ignore
    /// {
    ///     let x = 1;
    ///     x + 2
    /// }
    /// ```
    BlockExpression,

    /// An if expression
    /// ```rust,ignore
    /// if x > 0 { "positive" } else { "non-positive" }
    /// ```
    IfExpression,

    /// A match expression
    /// ```rust,ignore
    /// match x {
    ///     1 => "one",
    ///     2 => "two",
    ///     _ => "other",
    /// }
    /// ```
    MatchExpression,

    /// A loop expression
    /// ```rust,ignore
    /// loop {
    ///     println!("infinite loop");
    /// }
    /// ```
    LoopExpression,

    /// A while loop
    /// ```rust,ignore
    /// while x < 10 {
    ///     x += 1;
    /// }
    /// ```
    WhileExpression,

    /// A for loop
    /// ```rust,ignore
    /// for item in collection {
    ///     println!("{}", item);
    /// }
    /// ```
    ForExpression,

    /// A variable or identifier expression
    /// ```rust,ignore
    /// let name = x; // x is an identifier expression
    /// ```
    IdentifierExpression,

    /// A literal expression (any literal value)
    /// ```rust,ignore
    /// let x = 42; // 42 is a literal expression
    /// ```
    LiteralExpression,

    /// A path expression
    /// ```rust,ignore
    /// std::collections::HashMap::new()
    /// ```
    PathExpression,

    /// An integer literal
    /// ```rust,ignore
    /// 42, -10, 0xFF, 0b1010
    /// ```
    IntegerLiteral,

    /// A floating-point literal
    /// ```rust,ignore
    /// 3.14, -2.5, 1e10
    /// ```
    FloatLiteral,

    /// A string literal
    /// ```rust,ignore
    /// "hello", "world\n", r#"raw string"#
    /// ```
    StringLiteral,

    /// A character literal
    /// ```rust,ignore
    /// 'a', '\n', '🦀'
    /// ```
    CharLiteral,

    /// A boolean literal
    /// ```text
    /// true, false
    /// ```
    BooleanLiteral,

    /// A parenthesized expression
    /// ```rust,ignore
    /// (x + 1) * 2
    /// ```
    ParenthesizedExpression,

    /// A tuple expression
    /// ```rust,ignore
    /// (1, "hello", 3.14)
    /// ```
    TupleExpression,

    /// An array expression
    /// ```rust,ignore
    /// [1, 2, 3, 4, 5]
    /// ```
    ArrayExpression,

    /// A struct expression (struct instantiation)
    /// ```rust,ignore
    /// Point { x: 1.0, y: 2.0 }
    /// ```
    StructExpression,

    /// An assignment expression
    /// ```rust,ignore
    /// x = 42
    /// ```
    AssignmentExpression,

    /// A function call expression
    /// ```rust,ignore
    /// println!("Hello!")
    /// ```
    CallExpression,

    /// A method call expression
    /// ```rust,ignore
    /// string.trim().to_uppercase()
    /// ```
    MethodCallExpression,

    /// A field access expression
    /// ```rust,ignore
    /// point.x, user.name
    /// ```
    FieldExpression,

    /// An index expression
    /// ```rust,ignore
    /// array[0], vector[1..3]
    /// ```
    IndexExpression,

    /// A return expression
    /// ```rust,ignore
    /// return 42;
    /// ```
    ReturnExpression,

    /// A break expression
    /// ```rust,ignore
    /// break;
    /// break 42;
    /// ```
    BreakExpression,

    /// A continue expression
    /// ```rust,ignore
    /// continue;
    /// ```
    ContinueExpression,

    /// A pattern (used in match, let, etc.)
    /// ```rust,ignore
    /// Some(x), Ok(value), Point { x, y }
    /// ```
    Pattern,

    /// A match arm
    /// ```rust,ignore
    /// 1 => "one", // pattern => expression
    /// ```
    MatchArm,

    /// A type annotation
    /// ```rust,ignore
    /// i32, String, Vec<T>
    /// ```
    Type,

    /// A path type
    /// ```rust,ignore
    /// std::collections::HashMap<K, V>
    /// ```
    PathType,

    /// A tuple type
    /// ```rust,ignore
    /// (i32, String)
    /// ```
    TupleType,

    /// An array type
    /// ```rust,ignore
    /// [i32; 10]
    /// ```
    ArrayType,

    /// A slice type
    /// ```rust,ignore
    /// [i32], str
    /// ```
    SliceType,

    /// A reference type
    /// ```rust,ignore
    /// &str, &mut i32
    /// ```
    ReferenceType,

    /// A raw pointer type
    /// ```rust,ignore
    /// *const i32, *mut String
    /// ```
    PointerType,

    /// A function type
    /// ```rust,ignore
    /// fn(i32) -> String
    /// ```
    FunctionType,

    /// Generic parameters
    /// ```rust,ignore
    /// fn example<T, U>(x: T) -> U
    ///          ^^^^^^ generic parameters
    /// ```
    GenericParams,

    /// Generic arguments
    /// ```rust,ignore
    /// Vec<i32>, Option<String>
    ///     ^^^ generic arguments
    /// ```
    GenericArgs,

    /// A where clause
    /// ```rust,ignore
    /// where T: Display + Clone, U: Debug
    /// ```
    WhereClause,

    /// An attribute
    /// ```rust,ignore
    /// #[derive(Debug)]
    /// #[inline]
    /// ```
    Attribute,

    /// A visibility modifier
    /// ```rust,ignore
    /// pub, pub(crate), private (no modifier)
    /// ```
    Visibility,

    /// The async keyword
    /// ```rust,ignore
    /// async fn example() {}
    /// ```
    Async,

    /// The unsafe keyword
    /// ```rust,ignore
    /// unsafe fn example() {}
    /// ```
    Unsafe,

    /// The extern keyword
    /// ```rust,ignore
    /// extern "C" fn example();
    /// ```
    Extern,

    /// An identifier
    /// ```rust,ignore
    /// variable_name, function_name, TypeName
    /// ```
    Identifier,

    /// A generic expression (catch-all for expressions)
    /// ```rust,ignore
    /// Any expression that doesn't fit specific categories
    /// ```
    Expression,

    /// A binary expression
    /// ```rust,ignore
    /// x + y, a && b, count > 0
    /// ```
    BinaryExpression,

    /// A unary expression
    /// ```rust,ignore
    /// !flag, -number, *pointer
    /// ```
    UnaryExpression,

    /// An item used as a statement
    /// ```rust,ignore
    /// fn nested() {}
    /// ```
    ItemStatement,

    /// An argument list in function calls
    /// ```rust,ignore
    /// example(1, "hello", true)
    /// ```
    ArgumentList,

    /// A public item
    /// ```rust,ignore
    /// pub struct PublicStruct;
    /// ```
    PubItem,

    /// A block expression
    /// ```rust,ignore
    /// {
    ///     statements;
    ///     expr
    /// }
    /// ```
    Block,

    /// Represents a syntax error in the parsed code
    /// ```rust,ignore
    /// Invalid or unparseable syntax
    /// ```
    Error,
}

impl ElementType for RustElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> UniversalElementRole {
        use UniversalElementRole::*;
        match self {
            Self::SourceFile => Root,

            // Symbol Management: Definitions
            // These define names in a scope and are primary targets for outlines.
            Self::Function | Self::StructItem | Self::EnumItem | Self::Trait | Self::Impl | Self::ModuleItem | Self::Static | Self::Const | Self::TypeAlias | Self::ExternCrate | Self::Macro => Definition,

            // Hierarchy & Scoping: Containers
            // These provide structural grouping and lexical scopes.
            Self::Block | Self::ExternBlock | Self::ParameterList | Self::ArgumentList | Self::ArrayExpression | Self::TupleExpression | Self::StructExpression | Self::ParenthesizedExpression => Container,

            // Flow Control & Logic: Statements
            // Discrete instructions or units within a container.
            Self::LetStatement | Self::ExpressionStatement | Self::ItemStatement | Self::UseItem | Self::ReturnExpression | Self::BreakExpression | Self::ContinueExpression => Statement,

            // Flow Control & Logic: Expressions
            // Computed results involving operators or complex logic.
            Self::BinaryExpression
            | Self::UnaryExpression
            | Self::IfExpression
            | Self::MatchExpression
            | Self::LoopExpression
            | Self::WhileExpression
            | Self::ForExpression
            | Self::AssignmentExpression
            | Self::FieldExpression
            | Self::IndexExpression
            | Self::BlockExpression
            | Self::Expression => Expression,

            // Flow Control & Logic: Calls
            // Explicit invocations of functions or macros.
            Self::CallExpression | Self::MethodCallExpression => Call,

            // Atomic Values
            // Primitive data payloads (literals).
            Self::IntegerLiteral | Self::FloatLiteral | Self::StringLiteral | Self::CharLiteral | Self::BooleanLiteral | Self::LiteralExpression => Value,

            // Symbol Management: References
            Self::IdentifierExpression | Self::PathExpression => Reference,

            // Metadata & Auxiliaries: Typing
            Self::Type | Self::PathType | Self::TupleType | Self::ArrayType | Self::SliceType | Self::ReferenceType | Self::PointerType | Self::FunctionType | Self::ReturnType | Self::GenericParams | Self::GenericArgs | Self::WhereClause => Typing,

            // Metadata & Auxiliaries: Attributes & Modifiers
            Self::Attribute => Metadata,
            Self::Visibility | Self::Async | Self::Unsafe | Self::Extern => Attribute,

            // Special cases or unclassified
            Self::Parameter | Self::MatchArm | Self::Pattern => None,

            Self::Error => Error,
            _ => None,
        }
    }
}

impl From<RustTokenType> for RustElementType {
    fn from(token_type: RustTokenType) -> Self {
        match token_type {
            // Error type - represents invalid tokens
            RustTokenType::Error => Self::Error,

            // Identifiers and literals map directly to their element types
            RustTokenType::Identifier => Self::Identifier,
            RustTokenType::IntegerLiteral => Self::IntegerLiteral,
            RustTokenType::FloatLiteral => Self::FloatLiteral,
            RustTokenType::StringLiteral => Self::StringLiteral,
            RustTokenType::CharLiteral => Self::CharLiteral,
            RustTokenType::BoolLiteral => Self::BooleanLiteral,

            // Boolean keywords are treated as boolean literals
            RustTokenType::True => Self::BooleanLiteral,
            RustTokenType::False => Self::BooleanLiteral,

            // Other keywords and symbols are not converted to element types
            // The parser should handle these tokens directly without conversion
            _ => {
                // For all other token types, we don't perform conversion
                // Let the parser use token types directly for matching
                Self::Error // Temporary placeholder - parser should use tokens directly
            }
        }
    }
}
