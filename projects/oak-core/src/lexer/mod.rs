#![doc = include_str!("readme.md")]

use crate::{Language, TextEdit, errors::OakDiagnostics};

mod cache;
mod scan_comment;
mod scan_identifier;
mod scan_number;
mod scan_string;
mod scan_white_space;
mod state;
mod token;

pub use cache::*;
pub use core::range::Range;
pub use scan_comment::*;
pub use scan_identifier::*;
pub use scan_number::*;
pub use scan_string::*;
pub use scan_white_space::*;
pub use state::*;
pub use token::*;

/// Output type for lexical analysis operations, including diagnostics.
pub type LexOutput<L: Language> = OakDiagnostics<Tokens<L>>;

/// Trait for tokenizing source code into a sequence of tokens.
///
/// This trait defines the interface for converting raw source text into a stream of
/// atomic units ([`Token`s]) that the parser can consume.
///
/// # Usage Scenario
///
/// A `Lexer` implementation typically:
/// 1. Scans the source text using a [`SourceCursor`].
/// 2. Identifies token boundaries and kinds (e.g., Keywords, Identifiers, Punctuation).
/// 3. Returns a [`Tokens`] collection that supports efficient random access.
/// 4. Handles incremental lexing by using the provided [`LexerCache`] and [`TextEdit`]s.
pub trait Lexer<L: Language + Send + Sync> {
    /// Tokenizes the source text into a sequence of tokens.
    ///
    /// This method performs lexical analysis, converting raw source text into
    /// a sequence of [`Token`]s. It supports incremental lexing if edits and
    /// a cache are provided.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to tokenize.
    /// * `edits` - A slice of [`TextEdit`]s representing changes since the last lexing.
    /// * `cache` - A [`LexerCache`] for storing and retrieving lexical results.
    ///
    /// # Returns
    ///
    /// A [`LexOutput`] containing the tokens and any diagnostic messages.
    fn lex<'a, S: crate::source::Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<L>) -> LexOutput<L>;
}
