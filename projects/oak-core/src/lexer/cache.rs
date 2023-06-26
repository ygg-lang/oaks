use crate::{
    Language,
    lexer::{LexOutput, Token},
};

/// Cache trait for lexical results.
///
/// This trait defines the interface for caching and accessing lexical analysis results.
/// It provides methods for storing and retrieving token information from previous
/// lexical analysis operations.
#[allow(unused_variables)]
pub trait LexerCache<L: Language> {
    /// Sets the lexed output in the cache.
    ///
    /// # Arguments
    ///
    /// * `output` - The output from lexical analysis, including tokens and diagnostics
    fn set_lex_output(&mut self, output: LexOutput<L>);

    /// Gets a token from the cache by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the token to retrieve
    ///
    /// # Returns
    ///
    /// An `Option<Token<L::TokenType>>` containing the token if it exists,
    /// or `None` if the index is out of bounds or no tokens are cached
    fn get_token(&self, index: usize) -> Option<Token<L::TokenType>>;

    /// Gets the total number of tokens in the cache.
    ///
    /// # Returns
    ///
    /// The number of cached tokens, or 0 if no tokens are cached
    fn count_tokens(&self) -> usize;

    /// Checks if the cache contains any tokens.
    ///
    /// # Returns
    ///
    /// `true` if the cache contains tokens, `false` otherwise
    fn has_tokens(&self) -> bool;

    /// Gets all cached tokens as a slice.
    ///
    /// # Returns
    ///
    /// An optional slice of tokens if available.
    fn get_tokens(&self) -> Option<&[Token<L::TokenType>]> {
        None
    }
}

impl<'a, L: Language, C: LexerCache<L> + ?Sized> LexerCache<L> for &'a mut C {
    fn set_lex_output(&mut self, output: LexOutput<L>) {
        (**self).set_lex_output(output)
    }

    fn get_token(&self, index: usize) -> Option<Token<L::TokenType>> {
        (**self).get_token(index)
    }

    fn count_tokens(&self) -> usize {
        (**self).count_tokens()
    }

    fn has_tokens(&self) -> bool {
        (**self).has_tokens()
    }

    fn get_tokens(&self) -> Option<&[Token<L::TokenType>]> {
        (**self).get_tokens()
    }
}

/// A no-op implementation of `LexerCache`.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoLexerCache;

impl<L: Language> LexerCache<L> for NoLexerCache {
    fn set_lex_output(&mut self, _output: LexOutput<L>) {}

    fn get_token(&self, _index: usize) -> Option<Token<L::TokenType>> {
        None
    }

    fn count_tokens(&self) -> usize {
        0
    }

    fn has_tokens(&self) -> bool {
        false
    }
}
