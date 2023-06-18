//! Lexical analysis and tokenization for the Oak Core parsing framework.
//!
//! This module provides traits and utilities for converting source text into
//! sequences of tokens that can be consumed by parsers. It includes support
//! for common lexical patterns and incremental tokenization.

pub use self::{scan_comment::*, scan_string::*, scan_white_space::*};
use crate::{
    GreenBuilder, IncrementalCache, Language,
    errors::{OakDiagnostics, OakError},
    source::Source,
};
use std::{ops::Deref, range::Range};

/// Common lexical patterns and utilities shared across different languages.
///
/// This module provides reusable components for common lexical constructs such as
/// whitespace handling, number literals, string literals, and identifier recognition.
/// These utilities can be used by language-specific lexers to avoid reimplementing
/// basic tokenization patterns.
mod scan_white_space;

mod scan_comment;

pub mod scan_string;

/// Output type for lexical analysis operations.
///
/// This type alias represents the result of tokenization, containing
/// a vector of tokens and any diagnostic language that occurred during
/// the lexing process.
pub type LexOutput<L: Language> = OakDiagnostics<Vec<Token<L::SyntaxKind>>>;

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
pub trait Lexer<L: Language + Send + Sync + 'static> {
    /// Tokenizes the given source text into a sequence of tokens.
    ///
    /// Tokenizes source text into a sequence of tokens.
    ///
    /// This method performs a full lexical analysis of the source text,
    /// creating a new sequence of tokens from scratch. It uses a default
    /// cache configuration.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to tokenize
    ///
    /// # Returns
    ///
    /// A [`LexOutput`] containing the tokens and any diagnostic messages
    fn lex(&self, source: impl Source) -> LexOutput<L> {
        let mut pool = GreenBuilder::new(0);
        let cache = IncrementalCache::new(&mut pool);
        self.lex_incremental(source, 0, cache)
    }

    /// Tokenizes source text using an existing cache for incremental parsing.
    ///
    /// This method enables efficient re-lexing by reusing information from previous
    /// parsing operations, only processing the changed portions of the source.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to tokenize
    /// * `changed` - The number of bytes that have changed since the last parse
    /// * `cache` - The incremental cache containing previous parsing results
    ///
    /// # Returns
    ///
    /// A [`LexOutput`] containing the tokens and any diagnostic messages
    fn lex_incremental(&self, source: impl Source, changed: usize, cache: IncrementalCache<L>) -> LexOutput<L>;
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
pub struct LexerState<S, L: Language> {
    /// The source text being tokenized
    pub(crate) source: S,
    /// Current byte offset position in the source text
    pub(crate) offset: usize,
    pub(crate) tokens: Vec<Token<L::SyntaxKind>>,
    pub(crate) errors: Vec<OakError>,
}

impl<S: Source, L: Language> Deref for LexerState<S, L> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<S: Source, L: Language> LexerState<S, L> {
    /// Creates a new lexer state with the given source text.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to lex
    ///
    /// # Returns
    ///
    /// A new `LexerState` initialized at the beginning of the source
    pub fn new(source: S) -> Self {
        Self { source, offset: 0, tokens: vec![], errors: vec![] }
    }

    /// Creates a new lexer state with the given source text and incremental cache.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to lex
    /// * `changed` - The number of bytes that have changed since the last lex
    /// * `cache` - The incremental cache containing previous lexing results
    ///
    /// # Returns
    ///
    /// A new `LexerState` initialized at the beginning of the source with cache support
    pub fn new_with_cache(source: S, changed: usize, cache: IncrementalCache<L>) -> Self {
        Self { source, offset: 0, tokens: vec![], errors: vec![] }
    }

    /// Gets the remaining text from the current position to the end of the source.
    ///
    /// # Returns
    ///
    /// A string slice containing the remaining text
    pub fn rest(&self) -> &str {
        self.source.get_text_from(self.offset)
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
        self.source.length()
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
        self.peek_next_n(0)
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
        let rest = self.source.get_text_from(self.offset);
        rest.chars().nth(n)
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
    pub fn advance_with(&mut self, token: Token<L::SyntaxKind>) -> usize {
        self.offset += token.length();
        self.tokens.push(token);
        self.offset
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

    /// Checks if the lexer has not reached the end of the source text.
    ///
    /// # Returns
    ///
    /// `true` if not at the end of the source, `false` otherwise
    #[inline]
    pub fn not_at_end(&self) -> bool {
        self.offset < self.source.length()
    }

    /// Performs a safety check to prevent infinite loops during lexing.
    ///
    /// This method ensures that the lexer always makes progress by forcing
    /// advancement when stuck at the same position. It's used as a safeguard
    /// against infinite loops in lexer implementations.
    ///
    /// # Arguments
    ///
    /// * `safe_point` - The position to check against for potential deadlock
    pub fn safe_check(&mut self, safe_point: usize) {
        // 如果没有前进过，强制前进
        if self.offset == safe_point {
            match self.peek_next_n(0) {
                // 跳过当前字符
                Some(c) => self.offset += c.len_utf8(),
                // 无论如何都要前进，防止死循环
                None => self.offset += 1,
            }
            // tracing::warn!("deadlock");
        }
    }

    /// Finishes lexing and returns the final output with tokens and diagnostics.
    ///
    /// # Returns
    ///
    /// A `LexOutput` containing the collected tokens and any errors encountered
    pub fn finish(self, result: Result<(), OakError>) -> LexOutput<L> {
        match result {
            Ok(_) => OakDiagnostics { result: Ok(self.tokens), diagnostics: self.errors },
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: self.errors },
        }
    }
}
