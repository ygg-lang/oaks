/// Trait for kind kinds that represent different types of tokens and nodes.
///
/// This trait provides methods for categorizing and working with kind elements
/// in the parsing system.
pub trait SyntaxKind: Copy + Eq {
    /// Returns true if this kind kind represents trivia (whitespace, comments, etc.).
    ///
    /// Trivia tokens are typically ignored during parsing but preserved for
    /// formatting and tooling purposes.
    fn is_trivia(&self) -> bool;

    fn is_comment(&self) -> bool;

    /// Returns true if this kind kind represents whitespace.
    fn is_whitespace(&self) -> bool;

    fn is_token_type(&self) -> bool;

    fn is_element_type(&self) -> bool;
}
