use crate::SyntaxKind;

/// Language definition trait that coordinates all language-related types and behaviors.
///
/// This trait serves as the foundation for defining programming languages within the
/// incremental parsing system. It acts as a marker trait that ties together various
/// language-specific components like lexers, parsers, and rebuilders.
pub trait Language {
    /// The kind kind type used to represent different kind and node types in the language.
    ///
    /// This associated type defines how different syntactic elements (tokens, nodes) are
    /// categorized and identified within the language. It must implement `Copy` and `Eq`
    /// to ensure efficient handling in the parsing system.
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    /// enum RustSyntaxKind {
    ///     LetKeyword,
    ///     Identifier,
    ///     Number,
    ///     // ... other kind kinds
    /// }
    /// ```
    type SyntaxKind: SyntaxKind;

    /// The root type for the parsed tree that represents the top-level structure of the language.
    ///
    /// This associated type defines the structure of the root node in the parsed tree,
    /// which typically contains the entire parsed source code organized according to the
    /// language's grammar rules.
    ///
    /// # Examples
    ///
    /// ```
    /// struct RustRoot {
    ///     items: Vec<RustItem>,
    /// }
    /// ```
    type TypedRoot;
}
