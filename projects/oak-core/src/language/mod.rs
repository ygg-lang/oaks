use std::{fmt::Debug, hash::Hash};

/// Represents the broad category a language belongs to.
///
/// Categories are used by the framework to apply default behaviors,
/// such as choosing appropriate lexer/parser strategies or selecting
/// default themes for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LanguageCategory {
    /// General-purpose programming languages (e.g., Rust, C, Java).
    Programming,
    /// Markup and document languages (e.g., Markdown, HTML, Typst).
    Markup,
    /// Configuration and data serialization languages (e.g., YAML, JSON, TOML).
    Config,
    /// Styling languages (e.g., CSS, Sass, Less).
    StyleSheet,
    /// Domain-specific languages or specialized notation (e.g., SQL, Regex, Math).
    Dsl,
    /// Modeling languages (e.g., UML, Mermaid, PlantUML).
    Modeling,
    /// Other or unclassified.
    Other,
}

/// Language definition trait that coordinates all language-related types and behaviors.
///
/// This trait ties together language-specific components like lexers, parsers, and ASTs.
/// It enables the framework to be language-agnostic while providing type-safe
/// language-specific functionality.
pub trait Language: Send + Sync {
    /// The name of the language (e.g., "rust", "sql").
    const NAME: &'static str;

    /// The category of the language.
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    /// The token type used to represent different token and node types in the language.
    ///
    /// This associated type defines how different syntactic elements (tokens, nodes) are
    /// categorized and identified within the language. It must implement `Copy` and `Eq`
    /// to ensure efficient handling in the parsing system.
    ///
    /// # Requirements
    ///
    /// The token type must:
    /// - Implement the `TokenType` trait
    /// - Be copyable to enable efficient passing
    /// - Support equality comparison for token matching
    /// - Be sendable across thread boundaries
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    /// enum RustSyntaxKind {
    ///     LetKeyword,
    ///     Identifier,
    ///     Number,
    ///     // ... other token kinds
    /// }
    /// ```
    type TokenType: TokenType;

    /// The element type used to represent composite structures in the parsed tree.
    ///
    /// While tokens represent the atomic units of the language, elements represent
    /// the composite structures formed by combining tokens according to grammar rules.
    /// This includes expressions, statements, declarations, and other syntactic constructs.
    ///
    /// # Requirements
    ///
    /// The element type must:
    /// - Implement the `ElementType` trait
    /// - Be copyable for efficient handling
    /// - Support equality comparison
    /// - Be sendable across thread boundaries
    type ElementType: ElementType;

    /// The root type for the parsed tree that represents the top-level structure of the language.
    ///
    /// This associated type defines the structure of the root node in the parsed tree,
    /// which typically contains the entire parsed source code organized according to the
    /// language's grammar rules. The root type serves as the entry point for traversing
    /// and manipulating the parsed representation.
    ///
    /// # Design Considerations
    ///
    /// The root type should:
    /// - Contain references to all top-level language constructs
    /// - Provide efficient access to the parsed content
    /// - Support incremental updates when the source changes
    ///
    /// # Examples
    ///
    /// ```ignore
    /// struct RustRoot {
    ///     items: Vec<RustItem>,
    /// }
    ///
    /// struct RustRoot {
    ///     modules: Vec<Module>,
    ///     imports: Vec<Import>,
    ///     declarations: Vec<Declaration>,
    /// }
    /// ```
    type TypedRoot;
}

/// Token type definitions for tokens in the parsing system.
///
/// This trait serves as the foundation for defining different types of tokens.
/// By mapping language-specific kinds to [`UniversalTokenRole`], generic tools
/// like highlighters and formatters can work across many languages.
macro_rules! define_token_type {
    ($($bound:tt)*) => {
        /// A trait for types that represent a token's kind in a specific language.
        pub trait TokenType: Copy + Eq + Hash + Send + Sync + std::fmt::Debug $($bound)* {
            /// The associated role type for this token kind.
            type Role: TokenRole;

            /// A constant representing the end of the input stream.
            const END_OF_STREAM: Self;

            /// Returns the general syntactic role of this token.
    ///
    /// The role determines how the token is treated by generic language tools.
    /// For example, tokens with [`UniversalTokenRole::Keyword`] are typically
    /// highlighted using a specific theme color, regardless of the language.
    fn role(&self) -> Self::Role;

            /// Returns true if this token matches the specified language-specific role.
            fn is_role(&self, role: Self::Role) -> bool {
                self.role() == role
            }

            /// Returns true if this token matches the specified universal role.
            fn is_universal(&self, role: UniversalTokenRole) -> bool {
                self.role().universal() == role
            }

            /// Returns true if this token represents a comment.
            fn is_comment(&self) -> bool {
                self.is_universal(UniversalTokenRole::Comment)
            }

            /// Returns true if this token represents whitespace.
            fn is_whitespace(&self) -> bool {
                self.is_universal(UniversalTokenRole::Whitespace)
            }

            /// Returns true if this token represents an error condition.
            fn is_error(&self) -> bool {
                self.is_universal(UniversalTokenRole::Error)
            }

            /// Returns true if this token represents trivia (whitespace, comments, etc.).
            fn is_ignored(&self) -> bool {
                self.is_whitespace() || self.is_comment()
            }

            /// Returns true if this token represents the end of the input stream.
            fn is_end_of_stream(&self) -> bool {
                *self == Self::END_OF_STREAM
            }
        }
    };
}

#[cfg(feature = "serde")]
define_token_type!(+ serde::Serialize + for<'de> serde::Deserialize<'de>);

#[cfg(not(feature = "serde"))]
define_token_type!();

/// A trait for types that can represent a token's syntactic role.
pub trait TokenRole: Copy + Eq + Send {
    /// Maps this role to a universal, language-agnostic role.
    fn universal(&self) -> UniversalTokenRole;

    /// Returns a specific name for this role, used for granular highlighting.
    ///
    /// For universal roles, this should return the standard scope name (e.g., "keyword").
    /// For language-specific roles, it can return more specific names (e.g., "keyword.control").
    fn name(&self) -> &str;
}

/// Represents the general syntactic role of a token across diverse languages.
///
/// By mapping to these roles, generic tools can identify names, literals, or operators
/// across 100+ languages without needing to learn the specifics of each grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum UniversalTokenRole {
    /// Language reserved words or built-in commands (e.g., 'SELECT', 'let', 'MOV').
    Keyword,
    /// Identifiers, labels, keys, tags, or any name-like token.
    Name,
    /// Literal values like strings, numbers, booleans, or nulls.
    Literal,
    /// An escape sequence or a special character representation within a literal.
    Escape,
    /// Mathematical, logical, or structural operators (e.g., '+', '=>', 'LIKE').
    Operator,
    /// Structural characters like brackets, commas, semicolons.
    Punctuation,
    /// Developer annotations or documentation.
    Comment,
    /// Formatting characters like spaces or tabs.
    Whitespace,
    /// Malformed or unrecognized content.
    Error,
    /// No specific role assigned.
    None,
    /// End of stream marker.
    Eof,
}

impl TokenRole for UniversalTokenRole {
    fn universal(&self) -> UniversalTokenRole {
        *self
    }

    fn name(&self) -> &str {
        match *self {
            UniversalTokenRole::Keyword => "keyword",
            UniversalTokenRole::Name => "variable.other",
            UniversalTokenRole::Literal => "constant",
            UniversalTokenRole::Escape => "constant.character.escape",
            UniversalTokenRole::Operator => "keyword.operator",
            UniversalTokenRole::Punctuation => "punctuation",
            UniversalTokenRole::Comment => "comment",
            UniversalTokenRole::Whitespace => "punctuation.whitespace",
            UniversalTokenRole::Error => "invalid",
            UniversalTokenRole::None => "none",
            UniversalTokenRole::Eof => "punctuation.eof",
        }
    }
}

/// Element type definitions for nodes in the parsed tree.
///
/// While tokens represent the atomic units of a language, elements represent the
/// composite structures formed by combining tokens according to grammar rules.
/// This includes expressions, statements, declarations, and other syntactic constructs.
///
/// # Implementation Guidelines
///
/// When implementing this trait for a specific language:
/// - Use an enum with discriminant values for efficient matching
/// - Include a Root variant to identify the top-level element
/// - Include an Error variant for malformed constructs
/// - Define a `Role` associated type and implement the `role()` method.
///
/// # Examples
///
/// ```ignore
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// enum MyElement {
///     Root,
///     FunctionDeclaration,
///     Block,
///     Error,
/// }
///
/// impl ElementType for MyElement {
///     type Role = UniversalElementRole;
///
///     fn role(&self) -> Self::Role {
///         match self {
///             MyElement::Root => UniversalElementRole::Root,
///             MyElement::FunctionDeclaration => UniversalElementRole::Binding,
///             MyElement::Block => UniversalElementRole::Container,
///             MyElement::Error => UniversalElementRole::Error,
///         }
///     }
///
///     fn is_root(&self) -> bool {
///         matches!(self, MyElement::Root)
///     }
///
///     fn is_error(&self) -> bool {
///         matches!(self, MyElement::Error)
///     }
/// }
/// ```
macro_rules! define_element_type {
    ($($bound:tt)*) => {
        /// A trait for types that represent an element's kind in a syntax tree.
        pub trait ElementType: Copy + Eq + Hash + Send + Sync + std::fmt::Debug $($bound)* {
            /// The associated role type for this element kind.
            type Role: ElementRole;

            /// Returns the general syntactic role of this element.
    ///
    /// The role enables structural analysis without knowing the specific grammar.
    /// For example, a tool can find all [`UniversalElementRole::Definition`] nodes
    /// to build a symbol outline for any supported language.
    fn role(&self) -> Self::Role;

            /// Returns true if this element matches the specified language-specific role.
            fn is_role(&self, role: Self::Role) -> bool {
                self.role() == role
            }

            /// Returns true if this element matches the specified universal role.
            fn is_universal(&self, role: UniversalElementRole) -> bool {
                self.role().universal() == role
            }

            /// Returns true if this element represents the root of the parsed tree.
            fn is_root(&self) -> bool {
                self.is_universal(UniversalElementRole::Root)
            }

            /// Returns true if this element represents an error condition.
            fn is_error(&self) -> bool {
                self.is_universal(UniversalElementRole::Error)
            }
        }
    };
}

#[cfg(feature = "serde")]
define_element_type!(+ serde::Serialize + for<'de> serde::Deserialize<'de>);

#[cfg(not(feature = "serde"))]
define_element_type!();

/// A trait for types that can represent an element's structural role.
pub trait ElementRole: Copy + Eq + Send {
    /// Maps this role to a universal, language-agnostic role.
    fn universal(&self) -> UniversalElementRole;

    /// Returns a specific name for this role, used for granular highlighting.
    fn name(&self) -> &str;
}

/// Represents the general structural role of a syntax tree element.
///
/// Elements are categorized by their role to enable generic analysis across diverse
/// language families. For example, highlighters and symbol extractors can use these
/// roles to identify definitions, references, or containers without knowing the
/// specific grammar of each language.
///
/// # Structural Groups
///
/// - **Flow Control**: [`UniversalElementRole::Statement`], [`UniversalElementRole::Expression`], [`UniversalElementRole::Call`], and [`UniversalElementRole::Root`].
/// - **Symbol Management**: [`UniversalElementRole::Definition`], [`UniversalElementRole::Binding`], and [`UniversalElementRole::Reference`].
/// - **Hierarchy**: [`UniversalElementRole::Container`].
/// - **Metadata**: [`UniversalElementRole::Typing`], [`UniversalElementRole::Metadata`], [`UniversalElementRole::Attribute`], [`UniversalElementRole::Documentation`].
///
/// When a node could fit multiple roles, choose the one that represents its **primary
/// structural intent**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[non_exhaustive]
pub enum UniversalElementRole {
    /// The top-level root of the syntax tree, representing the entire document or source file.
    Root,

    /// A high-level structural container that defines a scope or logical grouping.
    Container,

    /// A node that represents the entire declaration or definition of a symbol.
    ///
    /// This role identifies the "whole" entity that defines something in the code,
    /// which is crucial for building symbol trees and navigation outlines.
    ///
    /// # Examples
    /// - **Rust**: The entire `Fn` declaration block, `Struct` item, or `Enum`.
    /// - **Markdown**: `Heading` or `LinkDefinition`.
    /// - **SQL**: The whole `CREATE TABLE` or `CREATE PROCEDURE` statement.
    /// - **ASM**: A `Proc` (procedure) block or a multi-line data definition.
    /// - **YAML**: A schema-defined object or a complex configuration block.
    Definition,

    /// A node that specifically performs the act of binding a name to an entity.
    ///
    /// Unlike `Definition`, which represents the entire construct, `Binding` targets
    /// the specific part (usually the identifier) that introduces the name.
    ///
    /// # Examples
    /// - **Rust**: The identifier node in a `let` pattern or function name.
    /// - **Markdown**: `LinkLabel` in a reference link definition.
    /// - **SQL**: The `Table` name identifier in `CREATE TABLE`.
    /// - **ASM**: A `Label` node (e.g., `main:`).
    /// - **YAML**: The `Key` in a key-value mapping.
    Binding,

    /// A node that refers to an existing name or entity defined elsewhere.
    ///
    /// # Examples
    /// - **Rust**: `PathExpr` (variable usage) or `MethodCall`.
    /// - **Markdown**: `LinkReference` or `FootnoteReference`.
    /// - **SQL**: `ColumnName` in a `SELECT` clause or `TableName` in `FROM`.
    /// - **ASM**: A `Label` reference in a jump (e.g., `JMP main`).
    /// - **YAML**: An `Alias` anchor (e.g., `*anchor_name`).
    Reference,

    /// A node representing a type signature, constraint, or type reference.
    ///
    /// This role distinguishes type information from general logic or values,
    /// which is essential for type checking and intelligent completion.
    ///
    /// # Examples
    /// - **Rust**: `TypePath` (e.g., `: i32`), `GenericArgument`, or `WhereClause`.
    /// - **SQL**: `DataType` (e.g., `VARCHAR(255)` or `INT`).
    /// - **ASM**: Size specifiers (e.g., `DWORD`, `PTR`).
    /// - **TypeScript**: `TypeAnnotation` or `InterfaceDeclaration`.
    Typing,

    /// Structured comments or documentation nodes attached to other elements.
    ///
    /// Unlike raw `Comment` tokens, these are syntax nodes that may contain
    /// their own internal structure (like Markdown or Tagged parameters).
    ///
    /// # Examples
    /// - **Rust**: `DocComment` (e.g., `/// ...`).
    /// - **Java**: `Javadoc` blocks.
    /// - **Python**: `Docstring` literals.
    Documentation,

    /// High-level annotations, decorators, or macros that provide extra semantic info.
    ///
    /// # Metadata vs Attribute
    /// - **Metadata**: Usually refers to language-level extensions that "decorate" an element
    ///   from the outside, often affecting compilation or runtime behavior (e.g., Rust attributes).
    /// - **Attribute**: Usually refers to built-in, structural properties that are part of the
    ///   element's native definition (e.g., HTML attributes).
    ///
    /// # Examples
    /// - **Rust**: `Attribute` (e.g., `#[derive(...)]`) or `MacroCall`.
    /// - **Markdown**: `Frontmatter` (YAML/TOML header).
    /// - **Java/TS**: `@Decorator` or `@Annotation`.
    /// - **Python**: `@decorator` syntax.
    Metadata,

    /// A specific property, flag, or attribute-value pair.
    ///
    /// Unlike `Metadata`, which decorates an element with external logic, `Attribute`
    /// represents intrinsic properties defined by the language's schema or structure.
    ///
    /// # Examples
    /// - **HTML/XML**: An `Attribute` (e.g., `id="main"`).
    /// - **Markdown**: `LinkTitle` or `ImageAlt` text.
    /// - **YAML**: A specific configuration property.
    /// - **ASM**: Segment attributes (e.g., `READONLY`, `EXECUTE`).
    Attribute,

    /// The key part of an attribute, property, or configuration entry.
    ///
    /// This role is distinct because:
    /// - It is not a **Reference** (it doesn't refer to an external symbol).
    /// - It is not a traditional **Binding** (it doesn't define a symbol in a global or lexical scope).
    /// - It is not a **Keyword** (it is typically a user-defined or schema-defined identifier).
    ///
    /// # Examples
    /// - **HTML**: The `id` in `id="main"`.
    /// - **Markdown**: `AttributeName` (in Pandoc-style `{ #id .class };`).
    /// - **YAML**: The key in a property mapping.
    /// - **TOML**: The key in a table entry.
    AttributeKey,

    /// A node that provides additional details or secondary information for another element.
    ///
    /// # Examples
    /// - **Rust**: `GenericParameter` list, `FunctionParameter` list.
    /// - **SQL**: `Constraint` details.
    Detail,

    /// A node that represents the name of an element, typically used in declarations.
    ///
    /// # Examples
    /// - **Rust**: The name identifier in a function or struct definition.
    /// - **HTML**: The tag name in an element.
    Name,

    /// A discrete syntactic unit within a container, representing a single
    /// logical entry or instruction.
    ///
    /// This typically maps to a **Statement** in programming languages, or a standalone
    /// instruction in assembly. In markup, it could represent a list item or a table row.
    ///
    /// # Examples
    /// - **Rust**: A `Stmt` inside a block.
    /// - **Markdown**: `ListItem` or `TableCell`.
    /// - **SQL**: A standalone `Statement` or a `Clause` (like `WHERE`).
    /// - **ASM**: A single `Instruction` (e.g., `NOP`).
    Statement,

    /// A node representing a computed result or a complex logical operation.
    ///
    /// Unlike a simple `Value` (which is an atomic literal), an `Expression` involves
    /// operators or logic that must be evaluated.
    ///
    /// # Examples
    /// - **Rust**: `BinaryExpr`, `UnaryExpr`, or `RangeExpr`.
    /// - **SQL**: `BinaryOp` in a `WHERE` clause.
    /// - **Python**: `ListComprehension` or `Lambda`.
    Expression,

    /// A node that performs an invocation or call to a function, method, or macro.
    ///
    /// This role identifies the active execution of a named entity with optional arguments.
    ///
    /// # Examples
    /// - **Rust**: `CallExpr`, `MethodCallExpr`, or `MacroInvocation`.
    /// - **SQL**: `FunctionCall` (e.g., `COUNT(*)`).
    /// - **Excel**: A formula call.
    Call,

    /// A node representing an **atomic** data value or a primitive constant.
    ///
    /// This role is strictly for atomic values like numbers, strings, or booleans.
    /// It **does not** include composite structures like arrays `[]` or objects `{}`,
    /// which should be categorized as [`UniversalElementRole::Container`].
    ///
    /// # Examples
    /// - **Rust**: `Literal` (strings, numbers, booleans).
    /// - **Markdown**: `InlineCode`, `Emphasis`, or `Strong`.
    /// - **SQL**: `Literal` values.
    /// - **JSON/YAML**: Atomic `Scalar` values (strings, integers, nulls).
    Value,

    /// A node that acts as a host for content in a different language or a raw
    /// fragment requiring a separate parsing pass (Language Injection).
    ///
    /// # Examples
    /// - **HTML**: A `<script>` or `<style>` block containing JS/CSS.
    /// - **Markdown**: `CodeBlock` (host for other languages).
    /// - **Rust/Java**: A string literal containing SQL (if marked for injection).
    /// - **PHP**: Raw HTML fragments outside of `<?php ... ?>` tags.
    Embedded,

    /// A node specifically created to represent a syntax error or recovery point
    /// in the source code.
    Error,

    /// No specific structural role assigned or recognized for this element.
    None,
}

impl ElementRole for UniversalElementRole {
    fn universal(&self) -> UniversalElementRole {
        *self
    }

    fn name(&self) -> &str {
        match *self {
            UniversalElementRole::Container => "meta.block",
            UniversalElementRole::Statement => "meta.statement",
            UniversalElementRole::Binding => "variable.other.declaration",
            UniversalElementRole::Reference => "variable.other.usage",
            UniversalElementRole::Call => "entity.name.function.call",
            UniversalElementRole::Expression => "meta.expression",
            UniversalElementRole::Value => "constant",
            UniversalElementRole::Definition => "entity.name.function",
            UniversalElementRole::Typing => "entity.name.type",
            UniversalElementRole::Metadata => "meta.preprocessor",
            UniversalElementRole::Attribute => "entity.other.attribute-name",
            UniversalElementRole::AttributeKey => "entity.other.attribute-name.key",
            UniversalElementRole::Detail => "meta.detail",
            UniversalElementRole::Name => "entity.name",
            UniversalElementRole::Embedded => "meta.embedded",
            UniversalElementRole::Documentation => "comment.block.documentation",
            UniversalElementRole::Root => "source",
            UniversalElementRole::Error => "invalid",
            UniversalElementRole::None => "none",
        }
    }
}
