/// Syntax kind definitions for tokens and nodes in the parsing system.
///
/// This module provides the [`SyntaxKind`] trait which serves as the foundation
/// for defining different types of tokens and nodes in the parsing system.
/// It enables categorization of kind elements and provides methods for
/// identifying their roles in the language grammar.
pub trait SyntaxKind: Copy + Eq + Send {
    /// Returns true if this kind represents trivia (whitespace, comments, etc.).
    ///
    /// Trivia tokens are typically ignored during parsing but preserved for
    /// formatting and tooling purposes.
    fn is_trivia(&self) -> bool;

    /// Returns true if this kind represents a comment.
    ///
    /// Comment tokens are a type of trivia that contain developer annotations
    /// in the source code.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Check if the current kind kind is a comment
    /// if kind.is_comment() {
    ///     // Handle comment-related logic
    ///     handle_comment();
    /// }
    /// ```
    fn is_comment(&self) -> bool;

    /// Returns true if this kind represents whitespace.
    fn is_whitespace(&self) -> bool;

    /// Returns true if this kind represents a token type.
    ///
    /// Token types are the basic lexical units that form the building blocks
    /// of the language grammar.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Check if the current kind kind is a token type
    /// if kind.is_token_type() {
    ///     // Handle token-related logic
    ///     process_token();
    /// }
    /// ```
    fn is_token_type(&self) -> bool;

    /// Returns true if this kind represents an element type.
    ///
    /// Element types represent higher-level syntactic structures composed
    /// of multiple tokens.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Check if the current kind kind is an element type
    /// if kind.is_element_type() {
    ///     // Handle element-related logic
    ///     handle_element();
    /// }
    /// ```
    fn is_element_type(&self) -> bool;
}
