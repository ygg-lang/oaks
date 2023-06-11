use crate::Lexer;

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
    type SyntaxKind: crate::SyntaxKind;
}
