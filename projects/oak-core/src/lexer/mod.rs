#![doc = include_str!("readme.md")]

use crate::{
    Language, TextEdit, TokenType,
    errors::{OakDiagnostics, OakError},
    source::{Source, SourceCursor},
};
pub use core::range::Range;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use triomphe::Arc;

/// Utilities for scanning comments.
mod scan_comment;
/// Utilities for scanning identifiers.
mod scan_identifier;
/// Utilities for scanning numbers.
mod scan_number;
/// Utilities for scanning string literals.
mod scan_string;
/// Utilities for scanning whitespace.
mod scan_white_space;

pub use self::{scan_comment::CommentConfig, scan_string::StringConfig, scan_white_space::WhitespaceConfig};

/// Output type for lexical analysis operations.
///
/// This type alias represents the result of tokenization, containing
/// a vector of tokens and any diagnostic language that occurred during
/// the lexing process.
pub type Tokens<L: Language> = Arc<[Token<L::TokenType>]>;

/// Output type for lexical analysis operations, including diagnostics.
pub type LexOutput<L: Language> = OakDiagnostics<Tokens<L>>;

/// Trait for tokenizing source code into sequences of tokens.
///
/// This trait defines the interface for converting source text into a sequence of
/// tokens that can be consumed by the parser. Implementations should handle
/// the specific lexical rules of their target language.
///
/// # Examples
///
/// ```ignore
/// struct MyLexer;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Copy)]
/// enum MyToken {
///     Number,
///     Identifier,
///     End,
/// }
///
/// impl TokenType for MyToken {
///     const END_OF_STREAM: Self = MyToken::End;
///     type Role = UniversalTokenRole;
///     fn role(&self) -> Self::Role { UniversalTokenRole::None }
/// }
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum MyElement {}
///
/// impl ElementType for MyElement {
///     type Role = UniversalElementRole;
///     fn role(&self) -> Self::Role { UniversalElementRole::None }
/// }
///
/// struct MyLanguage;
///
/// impl Language for MyLanguage {
///     const NAME: &'static str = "my-language";
///     type TokenType = MyToken;
///     type ElementType = MyElement;
///     type TypedRoot = ();
/// }
///
/// impl Lexer<MyLanguage> for MyLexer {
///     fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<MyLanguage>) -> LexOutput<MyLanguage> {
///         // Tokenization logic here
///         todo!()
///     }
/// }
/// ```
pub trait Lexer<L: Language + Send + Sync> {
    /// Tokenizes the given source text into a sequence of tokens.
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
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<L>) -> LexOutput<L>;
}

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

/// Represents a single kind in the source code.
///
/// Tokens are the fundamental units of lexical analysis, representing
/// categorized pieces of source text with their position information.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token<K> {
    /// The kind/category of this kind (e.g., keyword, identifier, number)
    pub kind: K,
    /// The byte range in the source text that this kind occupies
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_range"))]
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
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use oak_core::lexer::Token;
    /// # use core::range::Range;
    /// let kind = Token { kind: "ident", span: Range { start: 0, end: 5 } }
    /// assert_eq!(kind.length(), 5);
    /// ```
    #[inline]
    pub fn length(&self) -> usize {
        self.span.end - self.span.start
    }
}

/// A stream of tokens with associated source text.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "K: Serialize", deserialize = "K: Deserialize<'de>")))]
pub struct TokenStream<K: Copy> {
    /// The raw source text.
    pub raw: String,
    /// The tokens extracted from the source text.
    #[cfg_attr(feature = "serde", serde(with = "arc_slice_serde"))]
    pub tokens: Arc<[Token<K>]>,
}

#[cfg(feature = "serde")]
mod arc_slice_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<K, S>(arc: &Arc<[Token<K>]>, serializer: S) -> Result<S::Ok, S::Error>
    where
        K: Serialize,
        S: Serializer,
    {
        arc.as_ref().serialize(serializer)
    }

    pub fn deserialize<'de, K, D>(deserializer: D) -> Result<Arc<[Token<K>]>, D::Error>
    where
        K: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let vec = Vec::<Token<K>>::deserialize(deserializer)?;
        Ok(Arc::from_iter(vec))
    }
}

/// State information for incremental lexical analysis.
///
/// This struct maintains the current position and context during
/// tokenization, enabling incremental and resumable lexing operations.
#[derive(Debug)]
pub struct LexerState<'s, S: Source + ?Sized, L: Language> {
    pub(crate) cursor: SourceCursor<'s, S>,
    pub(crate) tokens: Vec<Token<L::TokenType>>,
    pub(crate) errors: Vec<OakError>,
}

impl<'s, S: Source + ?Sized, L: Language> LexerState<'s, S, L> {
    /// Creates a new lexer state with the given source text.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to lex
    ///
    /// # Returns
    ///
    /// A new `LexerState` initialized at the beginning of the source
    pub fn new(source: &'s S) -> Self {
        Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![] }
    }

    /// Creates a new lexer state with the given source text and incremental cache.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to lex
    /// * `relex_from` - The minimum byte offset that may have been affected by edits
    ///   (use `source.length()` to indicate no edits)
    /// * `cache` - The incremental cache containing previous lexing results
    ///
    /// # Returns
    ///
    /// A new `LexerState` initialized at the beginning of the source with cache support
    pub fn new_with_cache(source: &'s S, relex_from: usize, cache: &impl LexerCache<L>) -> Self {
        if !cache.has_tokens() {
            return Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![] };
        }

        let len = source.length();
        let relex_from = relex_from.min(len);

        // Fast path: fully re-used
        if relex_from >= len {
            let mut tokens = Vec::new();
            if let Some(cached) = cache.get_tokens() {
                tokens.extend_from_slice(cached)
            }
            else {
                let count = cache.count_tokens();
                tokens.reserve(count);
                for i in 0..count {
                    if let Some(t) = cache.get_token(i) {
                        tokens.push(t)
                    }
                }
            }
            let offset = tokens.last().map(|t| t.span.end).unwrap_or(0).min(len);
            return Self { cursor: SourceCursor::new_at(source, offset), tokens, errors: vec![] };
        }

        if relex_from == 0 {
            return Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![] };
        }

        let mut reused_tokens = Vec::new();
        const BACKTRACK_TOKENS: usize = 1;

        if let Some(cached) = cache.get_tokens() {
            // Binary search for the cut-off point since tokens are sorted by position
            let idx = cached.partition_point(|t| t.span.end <= relex_from);
            let keep = idx.saturating_sub(BACKTRACK_TOKENS);
            if keep > 0 {
                reused_tokens.extend_from_slice(&cached[..keep])
            }
        }
        else {
            // Fallback for caches that don't support slice access
            let count = cache.count_tokens();
            for i in 0..count {
                let Some(token) = cache.get_token(i)
                else {
                    break;
                };
                if token.span.end <= relex_from {
                    reused_tokens.push(token);
                }
                else {
                    break;
                }
            }
            let keep = reused_tokens.len().saturating_sub(BACKTRACK_TOKENS);
            reused_tokens.truncate(keep);
        }

        let stable_offset = reused_tokens.last().map(|t| t.span.end).unwrap_or(0);
        Self { cursor: SourceCursor::new_at(source, stable_offset), tokens: reused_tokens, errors: vec![] }
    }

    pub fn sub_state(&mut self, start: usize, _end: usize) -> Self {
        Self { cursor: SourceCursor::new_at(self.cursor.source(), start), tokens: vec![], errors: vec![] }
    }

    /// Returns the source text provider.
    pub fn get_source(&self) -> &'s S {
        self.cursor.source()
    }

    /// Gets the remaining text from the current position to the end of the source.
    ///
    /// # Returns
    ///
    /// A string slice containing the remaining text
    pub fn rest(&mut self) -> &str {
        self.cursor.rest()
    }

    /// Gets the remaining text as a byte slice.
    ///
    /// Useful for byte-oriented scanning operations.
    #[inline]
    pub fn rest_bytes(&mut self) -> &[u8] {
        self.cursor.rest().as_bytes()
    }

    /// Checks if the lexer has consumed all input from the source.
    ///
    /// Returns `true` if the current position is at or beyond the end of the source.
    pub fn fully_reused(&self) -> bool {
        self.cursor.position() >= self.cursor.source().length()
    }

    /// Gets the current byte offset position in the source text.
    ///
    /// # Returns
    ///
    /// The current byte offset from the start of the source text.
    #[inline]
    pub fn get_position(&self) -> usize {
        self.cursor.position()
    }

    /// Checks if the lexer has NOT consumed all input from the source.
    ///
    /// Returns `true` if there are still bytes left to be scanned.
    #[inline]
    pub fn not_at_end(&self) -> bool {
        self.cursor.position() < self.cursor.source().length()
    }

    /// Peeks at the next character without advancing the cursor.
    ///
    /// Returns `None` if at the end of the source.
    #[inline]
    pub fn peek(&mut self) -> Option<char> {
        self.cursor.peek_char()
    }

    /// Peeks at the character immediately following the current character.
    #[inline]
    pub fn peek_next(&mut self) -> Option<char> {
        self.cursor.peek_next_char()
    }

    /// Peeks at the character at the specified byte offset relative to the current position.
    #[inline]
    pub fn peek_next_n(&mut self, n: usize) -> Option<char> {
        self.cursor.peek_next_n(n)
    }

    /// Advances the cursor by the specified number of bytes.
    #[inline]
    pub fn advance(&mut self, len: usize) {
        self.cursor.advance_bytes(len);
    }

    /// Gets the total length of the source text in bytes.
    #[inline]
    pub fn get_length(&self) -> usize {
        self.cursor.source().length()
    }

    /// Gets a single character at the specified absolute byte offset.
    #[inline]
    pub fn get_char_at(&self, offset: usize) -> Option<char> {
        self.cursor.source().get_char_at(offset)
    }

    /// Peeks at the next byte without advancing the cursor.
    #[inline]
    pub fn peek_byte(&mut self) -> Option<u8> {
        self.cursor.peek_byte()
    }

    /// Advances the cursor by one byte and returns it.
    #[inline]
    pub fn advance_byte(&mut self) -> Option<u8> {
        self.cursor.advance_byte()
    }

    /// Advances the cursor while the byte predicate is true.
    ///
    /// Returns the byte range covered by the matched bytes.
    #[inline]
    pub fn take_while_byte(&mut self, pred: impl FnMut(u8) -> bool) -> Range<usize> {
        self.cursor.take_while_byte(pred)
    }

    /// Skips common ASCII whitespace (space, tab, newline, carriage return).
    ///
    /// Uses SIMD acceleration if available on the platform.
    /// Returns the range of the skipped whitespace.
    #[inline]
    pub fn skip_ascii_whitespace(&mut self) -> Range<usize> {
        self.cursor.skip_ascii_whitespace()
    }

    /// Skips all consecutive ASCII digits at the current position.
    ///
    /// Returns the range of the skipped digits.
    #[inline]
    pub fn skip_ascii_digits(&mut self) -> Range<usize> {
        self.cursor.skip_ascii_digits()
    }

    /// Skips all characters that can continue an ASCII identifier.
    ///
    /// This includes alphanumeric characters and underscores.
    /// Returns the range of the skipped characters.
    #[inline]
    pub fn skip_ascii_ident_continue(&mut self) -> Range<usize> {
        self.cursor.skip_ascii_ident_continue()
    }

    /// Skips all characters until the target byte is encountered.
    ///
    /// The target byte itself is NOT consumed.
    /// Returns the range of the skipped characters.
    #[inline]
    pub fn skip_until(&mut self, target: u8) -> Range<usize> {
        self.cursor.skip_until(target)
    }

    /// Scans an ASCII identifier.
    ///
    /// An identifier must start with an alphabetic character or an underscore,
    /// and can be followed by any number of alphanumeric characters or underscores.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token type to assign if an identifier is found.
    ///
    /// # Returns
    ///
    /// `true` if an identifier was successfully scanned and added.
    #[inline]
    pub fn scan_ascii_identifier(&mut self, kind: L::TokenType) -> bool {
        let start = self.get_position();
        if let Some(b) = self.peek_byte() {
            if b == b'_' || b.is_ascii_alphabetic() {
                self.advance_byte();
                self.skip_ascii_ident_continue();
                self.add_token(kind, start, self.get_position());
                return true;
            }
        }
        false
    }

    /// Scans a line comment starting with the given prefix.
    ///
    /// Consumes the prefix and all characters until the next newline or EOF.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token type for the line comment.
    /// * `prefix` - The string sequence that starts the comment (e.g., "//").
    #[inline]
    pub fn scan_line_comment(&mut self, kind: L::TokenType, prefix: &str) -> bool {
        let start = self.get_position();
        if self.consume_if_starts_with(prefix) {
            self.skip_until(b'\n');
            self.add_token(kind, start, self.get_position());
            return true;
        }
        false
    }

    /// Scans a block comment with given start and end sequences.
    ///
    /// Handles nested comments if the underlying implementation supports it,
    /// though this basic implementation is non-recursive.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token type for the block comment.
    /// * `start_seq` - The sequence that starts the block (e.g., "/*").
    /// * `end_seq` - The sequence that ends the block (e.g., "*/").
    #[inline]
    pub fn scan_block_comment(&mut self, kind: L::TokenType, start_seq: &str, end_seq: &str) -> bool {
        let start = self.get_position();
        if self.consume_if_starts_with(start_seq) {
            while let Some(_b) = self.peek_byte() {
                self.skip_until(end_seq.as_bytes()[0]);
                if self.consume_if_starts_with(end_seq) {
                    self.add_token(kind, start, self.get_position());
                    return true;
                }
                self.advance_byte();
            }
            // Unclosed block comment is still a comment in many languages,
            // but we might want to add an error here in the future.
            self.add_token(kind, start, self.get_position());
            return true;
        }
        false
    }

    /// Gets a reference to the tokens collected so far.
    ///
    /// # Returns
    ///
    /// A slice of tokens collected during the lexing process.
    #[inline]
    pub fn tokens(&self) -> &[Token<L::TokenType>] {
        &self.tokens
    }

    /// Sets the current position to the specified byte offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The new byte offset position.
    ///
    /// # Returns
    ///
    /// The previous byte offset position.
    #[inline]
    pub fn set_position(&mut self, offset: usize) -> usize {
        self.cursor.set_position(offset)
    }

    /// Returns a reference to the underlying source.
    pub fn source(&self) -> &'s S {
        self.cursor.source()
    }

    /// Returns the text in the specified byte range.
    pub fn get_text_in(&self, range: Range<usize>) -> Cow<'_, str> {
        self.cursor.source().get_text_in(range)
    }

    /// Returns the text from the specified byte offset to the end of the source.
    pub fn get_text_from(&self, offset: usize) -> Cow<'_, str> {
        self.cursor.source().get_text_from(offset)
    }

    /// Checks if the source starts with the given pattern at the current position.
    pub fn starts_with(&mut self, pattern: &str) -> bool {
        self.cursor.starts_with(pattern)
    }

    /// Consumes the pattern if it exists at the current position.
    ///
    /// Returns `true` if the pattern was found and consumed, advancing the cursor.
    pub fn consume_if_starts_with(&mut self, pattern: &str) -> bool {
        self.cursor.consume_if_starts_with(pattern)
    }

    /// Gets the tokens collected so far in the lexer state.
    ///
    /// # Returns
    ///
    /// A slice of tokens collected during lexing.
    #[inline]
    pub fn get_tokens(&self) -> &[Token<L::TokenType>] {
        &self.tokens
    }

    /// Adds an error to the lexer state's diagnostics.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to add.
    #[inline]
    pub fn add_error(&mut self, error: impl Into<OakError>) {
        self.errors.push(error.into());
    }

    /// Adds a token to the lexer state.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind/type of the token.
    /// * `start` - The starting byte offset.
    /// * `end` - The ending byte offset.
    #[inline]
    pub fn add_token(&mut self, kind: L::TokenType, start: usize, end: usize) {
        self.tokens.push(Token { kind, span: Range { start, end } });
    }

    /// Adds an end-of-file (EOF) token to the lexer state.
    ///
    /// This method creates and adds an `END_OF_STREAM` token at the current position.
    /// It is typically called when the lexer reaches the end of the source text.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use core::range::Range;
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, UniversalTokenRole, TokenRole, UniversalElementRole, ElementRole, ElementType};
    /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken {
    /// #     End,
    /// # }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # #[derive(Clone)]
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("test");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    /// state.take_while(|_| true); // Advance to end
    /// state.add_eof();
    ///
    /// assert_eq!(state.tokens().len(), 1);
    /// assert_eq!(state.tokens()[0].span, Range { start: 4, end: 4 });
    /// ```
    #[inline]
    pub fn add_eof(&mut self) {
        let end = self.get_position();
        self.add_token(L::TokenType::END_OF_STREAM, end, end)
    }

    /// Gets the current character at the current position.
    ///
    /// # Returns
    ///
    /// The current character, or `None` if at the end of the source
    #[inline]
    pub fn current(&mut self) -> Option<char> {
        self.cursor.peek_char()
    }

    /// Advances the position by the current character's length.
    ///
    /// # Returns
    ///
    /// The character that was skipped, or `None` if at the end of the source
    #[inline]
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.advance(ch.len_utf8());
        Some(ch)
    }

    /// Advances the position by the token's length and adds the token to the lexer state.
    ///
    /// This method combines two common operations: advancing the lexer position
    /// and adding a token to the token list. It calculates the advance distance
    /// from the token's span, ensuring consistent positioning.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to add to the lexer state
    ///
    /// # Returns
    ///
    /// The new byte offset position after advancing
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use core::range::Range;
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, UniversalTokenRole, TokenRole, UniversalElementRole, ElementRole, ElementType};
    /// #     /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # #[derive(Clone)]
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("hello world");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    ///
    /// // Create a token for "hello"
    /// let token = Token { kind: SimpleToken::Identifier, span: Range { start: 0, end: 5 } }
    ///
    /// // Initially at position 0
    /// assert_eq!(state.get_position(), 0);
    ///
    /// // Advance and add the token
    /// let new_pos = state.advance_with(token);
    ///
    /// // Now at position 5 and token is added
    /// assert_eq!(new_pos, 5);
    /// assert_eq!(state.get_position(), 5);
    /// assert_eq!(state.get_tokens().len(), 1);
    /// assert_eq!(state.get_tokens()[0].kind, SimpleToken::Identifier);
    /// ```
    ///
    /// # Note
    ///
    /// The caller must ensure that the token's span is valid and that the advance
    /// does not split multi-byte UTF-8 characters. The token should be created
    /// with proper character boundaries.
    #[inline]
    pub fn advance_with(&mut self, token: Token<L::TokenType>) -> usize {
        self.cursor.advance_bytes(token.length());
        self.tokens.push(token);
        self.cursor.position()
    }

    /// Consumes characters while the predicate returns true, returning the consumed range.
    ///
    /// This method iterates through the source text from the current position,
    /// consuming characters as long as the predicate function returns true.
    /// It's commonly used for recognizing patterns like identifiers, numbers,
    /// or whitespace sequences.
    ///
    /// # Arguments
    ///
    /// * `pred` - A closure that takes a character and returns true if the character
    ///            should be consumed, false otherwise
    ///
    /// # Returns
    ///
    /// A byte range representing the span of consumed characters
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use core::range::Range;
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, UniversalTokenRole, TokenRole, UniversalElementRole, ElementRole, ElementType};
    /// #     /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken { End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # #[derive(Clone)]
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("hello123world");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    ///
    /// // Consume alphabetic characters
    /// let range = state.take_while(|c| c.is_alphabetic());
    ///
    /// // Should have consumed "hello"
    /// assert_eq!(range, Range { start: 0, end: 5 });
    /// assert_eq!(state.get_position(), 5);
    ///
    /// // Consume numeric characters
    /// let range = state.take_while(|c| c.is_numeric());
    ///
    /// // Should have consumed "123"
    /// assert_eq!(range, Range { start: 5, end: 8 });
    /// assert_eq!(state.get_position(), 8);
    /// ```
    ///
    /// # Performance Note
    ///
    /// This method operates on a character-by-character basis, which means it
    /// correctly handles multi-byte UTF-8 characters. For performance-critical
    /// code, consider using byte-based methods when working with ASCII-only text.
    pub fn take_while(&mut self, mut pred: impl FnMut(char) -> bool) -> Range<usize> {
        let start = self.cursor.position();
        while let Some(ch) = self.peek() {
            if pred(ch) { self.advance(ch.len_utf8()) } else { break }
        }
        Range { start, end: self.cursor.position() }
    }

    /// Performs a safety check to prevent infinite loops during lexing.
    ///
    /// This method ensures that the lexer always makes progress by forcing
    /// advancement when stuck at the same position. It's used as a safeguard
    /// against infinite loops in lexer implementations.
    ///
    /// The method compares the current position with a previously saved "safe point"
    /// position. If they're the same, it means the lexer hasn't made progress since
    /// that safe point, potentially indicating an infinite loop. In this case, the
    /// method forces advancement by at least one character.
    ///
    /// # Arguments
    ///
    /// * `safe_point` - The position to check against for potential deadlock
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, UniversalTokenRole, TokenRole, UniversalElementRole, ElementRole, ElementType};
    /// #     /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken { End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("test");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    ///
    /// // Save the current position as a safe point
    /// let safe_point = state.get_position();
    ///
    /// // In a real lexer, you would do some processing here
    /// // If something went wrong and we didn't advance, this would prevent infinite loop
    /// state.advance_if_dead_lock(safe_point);
    ///
    /// // If we were stuck, we would have advanced by at least 1
    /// assert!(state.get_position() >= safe_point);
    /// ```
    ///
    /// # Usage in Lexer Implementations
    ///
    /// This method is typically used at the beginning or end of lexing loops:
    ///
    /// ```ignore
    /// loop {
    ///     let safe_point = state.get_position();
    ///     
    ///     // Try to recognize a token
    ///     if let Some(token) = try_recognize_token(&mut state) {
    ///         // Success, continue loop
    ///         continue;
    ///     }
    ///     
    ///     // If we get here, we didn't recognize anything
    ///     // This prevents infinite loops if recognition fails
    ///     state.advance_if_dead_lock(safe_point);
    ///     
    ///     if state.not_at_end() {
    ///         // Continue trying to recognize tokens
    ///         continue;
    ///     } else {
    ///         // Reached end of source
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn advance_if_dead_lock(&mut self, safe_point: usize) {
        // Force advance if no progress was made
        if self.cursor.position() == safe_point {
            if let Some(ch) = self.peek() {
                // Skip current character
                self.advance(ch.len_utf8())
            }
            else {
                // Advance anyway to prevent infinite loop
                self.advance(1)
            }
            // tracing::warn!("deadlock")
        }
    }

    /// Finishes lexing and returns the final output with tokens and diagnostics.
    ///
    /// This method concludes the lexing process by converting the collected tokens
    /// and errors into a `LexOutput` result. It takes a `Result` parameter that
    /// represents the overall success or failure of the lexing operation.
    ///
    /// If the result is `Ok`, the tokens are returned as the successful result.
    /// If the result is `Err`, the error is returned as the failure result.
    /// In both cases, any collected diagnostic errors are included in the output.
    ///
    /// # Arguments
    ///
    /// * `result` - The result of the lexing operation (Ok for success, Err for failure)
    ///
    /// # Returns
    ///
    /// A `LexOutput` containing the tokens (if successful) and any diagnostic errors
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(new_range_api)]
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, OakError, OakDiagnostics, UniversalTokenRole, UniversalElementRole, ElementType};
    /// #     /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("test");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    ///
    /// // Add some tokens during lexing
    /// state.add_token(SimpleToken::Identifier, 0, 4);
    ///
    /// // Finish with successful result
    /// let output = state.finish(Ok(()));
    ///
    /// // Check the results
    /// assert!(output.result.is_ok());
    /// assert_eq!(output.result.unwrap().len(), 1);
    /// assert_eq!(output.diagnostics.len(), 0);
    ///
    /// // Example with error
    /// let source2 = SourceText::new("test");
    /// let mut state2 = LexerState::<_, SimpleLanguage>::new(&source2);
    /// state2.add_error(OakError::custom_error("Test error"));
    ///
    /// let output2 = state2.finish(Err(OakError::custom_error("Fatal error")));
    ///
    /// // Check the results
    /// assert!(output2.result.is_err());
    /// assert_eq!(output2.diagnostics.len(), 1); // The added error
    /// ```
    pub fn finish(self, result: Result<(), OakError>) -> LexOutput<L> {
        match result {
            Ok(_) => {
                let tokens: Tokens<L> = self.tokens.into();
                OakDiagnostics { result: Ok(tokens), diagnostics: self.errors }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: self.errors },
        }
    }

    /// Finishes lexing and returns the final output with tokens, diagnostics, and updated cache.
    ///
    /// This method is similar to `finish` but additionally updates the incremental cache
    /// with the new tokens. It's used for incremental lexing where the results need to
    /// be cached for future reuse when the source text changes.
    ///
    /// The method first creates the output in the same way as `finish`, then updates
    /// the cache's `last_lex` field with the new tokens. This enables the next call
    /// to `new_with_cache` to reuse these tokens if the source text hasn't changed.
    ///
    /// # Arguments
    ///
    /// * `result` - The result of the lexing operation (Ok for success, Err for failure)
    /// * `cache` - The incremental cache to update with the new tokens
    ///
    /// # Returns
    ///
    /// A `LexOutput` containing the tokens (if successful) and any diagnostic errors
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #![feature(new_range_api)]
    /// # use core::range::Range;
    /// # use oak_core::lexer::{LexerState, Token};
    /// # use oak_core::{Language, TokenType, SourceText, OakError, LexOutput, UniversalTokenRole, UniversalElementRole, ElementType};
    /// # use oak_core::parser::session::ParseSession;
    /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # enum SimpleElement {}
    /// #
    /// # impl ElementType for SimpleElement {
    /// #     type Role = UniversalElementRole;
    /// #     fn role(&self) -> Self::Role { UniversalElementRole::None }
    /// # }
    /// #
    /// # struct SimpleLanguage;
    /// #
    /// # impl Language for SimpleLanguage {
    /// #     const NAME: &'static str = "simple";
    /// #     type TokenType = SimpleToken;
    /// #     type ElementType = SimpleElement;
    /// #     type TypedRoot = ();
    /// # }
    /// #
    /// let source = SourceText::new("test");
    /// let mut state = LexerState::<_, SimpleLanguage>::new(&source);
    ///
    /// // Create a cache for incremental lexing
    /// let mut cache = ParseSession::<SimpleLanguage>::new(16);
    ///
    /// // Add some tokens during lexing
    /// state.add_token(SimpleToken::Identifier, 0, 4);
    ///
    /// // Finish with cache update
    /// let output = state.finish_with_cache(Ok(()), &mut cache);
    ///
    /// // Check the results
    /// assert!(output.result.is_ok());
    /// assert_eq!(output.result.unwrap().len(), 1);
    /// ```
    ///
    /// # Incremental Lexing Workflow
    ///
    /// This method is typically used as part of an incremental lexing workflow:
    ///
    /// ```ignore
    /// // First lexing
    /// let mut state = LexerState::new_with_cache(source, source.length(), cache);
    /// // ... lexing logic ...
    /// let output = state.finish_with_cache(Ok(()), cache);
    ///
    /// // Later, when source changes
    /// let relex_from = calculate_min_affected_offset(old_source, new_source);
    /// let mut state = LexerState::new_with_cache(new_source, relex_from, cache);
    /// // ... lexing logic (reusing unchanged tokens) ...
    /// let output = state.finish_with_cache(Ok(()), cache);
    /// ```
    pub fn finish_with_cache(self, result: Result<(), OakError>, cache: &mut impl LexerCache<L>) -> LexOutput<L> {
        let out = self.finish(result);
        cache.set_lex_output(out.clone());
        out
    }
}
