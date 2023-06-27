use alloc::string::String;
use oak_core::{language::Language, tree::RedTree};

/// Generic formatter trait for language-specific formatters
///
/// This trait defines the interface that language-specific formatters must implement.
pub trait Formatter<L: Language> {
    /// The state type used by this formatter
    type State;

    /// The output type produced by this formatter
    type Output;

    /// Formats a red-green tree
    ///
    /// # Parameters
    /// - `tree`: The red-green tree to format
    /// - `state`: The current formatter state
    ///
    /// # Returns
    /// The formatted output
    fn format<'a>(&self, tree: &RedTree<'a, L>, state: &mut Self::State) -> Self::Output;
}

/// A generic formatter that can be used for any language
///
/// This struct provides a common interface for formatting code in any language.
pub struct GenericFormatter<L: Language, F: Formatter<L>> {
    /// The language-specific formatter implementation
    formatter: F,
    _marker: core::marker::PhantomData<L>,
}

impl<L: Language, F: Formatter<L>> GenericFormatter<L, F> {
    /// Creates a new GenericFormatter
    ///
    /// # Parameters
    /// - `formatter`: The language-specific formatter implementation
    pub fn new(formatter: F) -> Self {
        Self { formatter, _marker: core::marker::PhantomData }
    }

    /// Formats the given source code
    ///
    /// # Parameters
    /// - `source`: The source code to format
    ///
    /// # Returns
    /// The formatted source code
    pub fn format_source(&self, source: &str) -> String {
        source.to_string()
    }
}
