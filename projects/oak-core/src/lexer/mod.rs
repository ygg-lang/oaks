//! Lexical analysis and tokenization for the Oak Core parsing framework.
//!
//! This module provides traits and utilities for converting source text into
//! sequences of tokens that can be consumed by parsers. It includes support
//! for common lexical patterns and incremental tokenization.

use crate::{
    Language, SourceText,
    errors::{OakDiagnostics, OakError},
};
use alloc::{vec, vec::Vec};
use core::range::Range;

/// Common lexical patterns and utilities shared across different languages.
///
/// This module provides reusable components for common lexical constructs such as
/// whitespace handling, number literals, string literals, and identifier recognition.
/// These utilities can be used by language-specific lexers to avoid reimplementing
/// basic tokenization patterns.
pub mod common;

/// Output type for lexical analysis operations.
///
/// This type alias represents the result of tokenization, containing
/// a vector of tokens and any diagnostic language that occurred during
/// the lexing process.
pub type LexOutput<K: Copy> = OakDiagnostics<Vec<Token<K>>>;

/// Trait for tokenizing source code into sequences of tokens.
///
/// This trait defines the interface for converting source text into a sequence of
/// tokens that can be consumed by the parser. Implementations should handle
/// the specific lexical rules of their target language.
///
/// # Examples
///
/// ```rust
/// # use oak_core::{Lexer, Language, SourceText, LexOutput};
///
/// struct MyLexer;
/// enum MyToken {
///     Number,
///     Identifier,
/// }
///
/// impl Language for MyToken {
///     type SyntaxKind = MyToken;
/// }
///
/// impl Lexer<MyToken> for MyLexer {
///     fn lex(&self, source: &SourceText) -> LexOutput<MyToken> {
///         // Tokenization logic here
///         todo!()
///     }
/// }
/// ```
pub trait Lexer<L: Language> {
    /// Tokenizes the given source text into a sequence of tokens.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to tokenize
    ///
    /// # Returns
    ///
    /// A [`LexOutput`] containing the tokens and any diagnostic language
    fn lex(&self, source: &SourceText) -> LexOutput<L::SyntaxKind>;
}

/// Represents a single kind in the source code.
///
/// Tokens are the fundamental units of lexical analysis, representing
/// categorized pieces of source text with their position information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<K> {
    /// The kind/category of this kind (e.g., keyword, identifier, number)
    pub kind: K,
    /// The byte range in the source text that this kind occupies
    pub span: Range<usize>,
}

impl<K> Token<K> {
    /// Returns the length of this kind in bytes.
    ///
    /// # Returns
    ///
    /// The number of bytes between the start and end of the kind's span
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::Token;
    /// # use core::range::Range;
    ///
    /// let kind = Token { kind: "ident", span: Range { start: 0, end: 5 } };
    /// assert_eq!(kind.length(), 5);
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        self.span.end - self.span.start
    }
}

/// State information for incremental lexical analysis.
///
/// This struct maintains the current position and context during
/// tokenization, enabling incremental and resumable lexing operations.
#[derive(Debug)]
pub struct LexerState<'input, L: Language> {
    /// The source text being tokenized
    pub(crate) source: &'input SourceText,
    /// Current byte offset position in the source text
    pub(crate) offset: usize,
    pub(crate) tokens: Vec<Token<L::SyntaxKind>>,
    pub(crate) errors: Vec<OakError>,
}
impl<'input, L: Language> LexerState<'input, L> {
    /// Creates a new lexer state for the given source text.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to be tokenized
    ///
    /// # Returns
    ///
    /// A new lexer state initialized at the beginning of the source text
    #[inline]
    pub fn new(source: &'input SourceText) -> Self {
        Self { source, offset: 0, tokens: vec![], errors: vec![] }
    }

    /// Gets the current byte offset position in the source text.
    ///
    /// # Returns
    ///
    /// The current byte offset from the start of the source text
    #[inline]
    pub fn get_position(&self) -> usize {
        self.offset
    }

    /// Sets the current position to the specified byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The new byte offset position
    ///
    /// # Returns
    ///
    /// The previous byte offset position
    #[inline]
    pub fn set_position(&mut self, offset: usize) -> usize {
        let last = self.offset;
        self.offset = offset;
        last
    }

    /// Gets the total length of the source text in bytes.
    ///
    /// # Returns
    ///
    /// The total number of bytes in the source text
    pub fn get_length(&self) -> usize {
        self.source.raw.len()
    }

    /// Adds an error to the lexer state.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to add to the diagnostics
    #[inline]
    pub fn add_error(&mut self, error: impl Into<OakError>) {
        self.errors.push(error.into());
    }

    /// Adds a token to the lexer state.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token
    /// * `start` - The starting byte offset of the token
    /// * `end` - The ending byte offset of the token
    #[inline]
    pub fn add_token(&mut self, kind: L::SyntaxKind, start: usize, end: usize) {
        self.tokens.push(Token { kind, span: Range { start, end } });
    }

    /// Gets the current character at the current position.
    ///
    /// # Returns
    ///
    /// The current character, or `None` if at the end of the source
    #[inline]
    pub fn current(&self) -> Option<char> {
        self.peek_next_n(0)
    }

    /// Peeks at the next character without advancing the position.
    ///
    /// # Returns
    ///
    /// The next character, or `None` if at the end of the source
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.peek_next_n(1)
    }

    /// Peeks at the character n positions ahead without advancing the position.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of characters to peek ahead
    ///
    /// # Returns
    ///
    /// The character n positions ahead, or `None` if beyond the end of the source
    pub fn peek_next_n(&self, n: usize) -> Option<char> {
        let rest = self.source.raw.get(self.offset..)?;
        rest.chars().nth(n.saturating_sub(1))
    }

    /// Advances the position by the specified number of bytes.
    ///
    /// # Arguments
    ///
    /// * `length` - The number of bytes to advance
    ///
    /// # Returns
    ///
    /// The new byte offset position
    #[inline]
    pub fn advance(&mut self, length: usize) -> usize {
        let end = self.offset + length;
        self.offset = end;
        end
    }

    /// Advances the position by the specified number of bytes and adds a token.
    ///
    /// # Arguments
    ///
    /// * `length` - The number of bytes to advance
    /// * `token` - The kind of token to add
    ///
    /// # Returns
    ///
    /// The new byte offset position
    ///
    /// # Note
    ///
    /// The caller must ensure that the advance is at character boundaries.
    #[inline]
    pub fn advance_with(&mut self, length: usize, token: L::SyntaxKind) -> usize {
        let end = self.offset + length;
        self.tokens.push(Token { kind: token, span: Range { start: self.offset, end } });
        self.offset = end;
        end
    }

    /// Consumes characters while the predicate returns true, returning the consumed range.
    ///
    /// # Arguments
    ///
    /// * `pred` - The predicate function that determines whether to consume a character
    ///
    /// # Returns
    ///
    /// The byte range of consumed characters
    pub fn take_while(&mut self, mut pred: impl FnMut(char) -> bool) -> Range<usize> {
        let start = self.offset;
        while let Some(ch) = self.peek() {
            if pred(ch) {
                self.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        Range { start, end: self.offset }
    }

    /// Checks if the lexer has reached the end of the source text.
    ///
    /// # Returns
    ///
    /// `true` if at or beyond the end of the source, `false` otherwise
    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.offset >= self.source.len()
    }

    /// Checks if the lexer has not reached the end of the source text.
    ///
    /// # Returns
    ///
    /// `true` if not at the end of the source, `false` otherwise
    #[inline]
    pub fn not_at_end(&self) -> bool {
        self.offset < self.source.len()
    }

    /// Finishes lexing and returns the final output with tokens and diagnostics.
    ///
    /// # Returns
    ///
    /// A `LexOutput` containing the collected tokens and any errors encountered
    pub fn finish(self) -> LexOutput<L::SyntaxKind> {
        OakDiagnostics { result: Ok(self.tokens), diagnostics: self.errors }
    }
}
