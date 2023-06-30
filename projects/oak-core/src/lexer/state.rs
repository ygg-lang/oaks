use crate::{
    Language, TokenType,
    errors::OakError,
    lexer::{LexOutput, LexerCache, Token, Tokens},
    source::{Source, SourceCursor},
};
pub use core::range::Range;
use std::borrow::Cow;

/// Represents the state of the lexer during a tokenization session.
///
/// This struct maintains the current position and context during
/// tokenization, enabling incremental and resumable lexing operations.
/// It tracks the current position in the source text, collected tokens,
/// and any errors encountered.
#[derive(Debug)]
pub struct LexerState<'s, S: Source + ?Sized, L: Language> {
    pub(crate) cursor: SourceCursor<'s, S>,
    pub(crate) tokens: Vec<Token<L::TokenType>>,
    pub(crate) errors: Vec<OakError>,
    pub(crate) end_limit: Option<usize>,
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
        Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![], end_limit: None }
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
            return Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![], end_limit: None };
        }

        let len = source.length();
        let relex_from = relex_from.min(len);

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
            return Self { cursor: SourceCursor::new_at(source, offset), tokens, errors: vec![], end_limit: None };
        }

        if relex_from == 0 {
            return Self { cursor: SourceCursor::new(source), tokens: vec![], errors: vec![], end_limit: None };
        }

        let mut reused_tokens = Vec::new();
        const BACKTRACK_TOKENS: usize = 1;

        if let Some(cached) = cache.get_tokens() {
            let idx = cached.partition_point(|t| t.span.end <= relex_from);
            let keep = idx.saturating_sub(BACKTRACK_TOKENS);
            if keep > 0 {
                reused_tokens.extend_from_slice(&cached[..keep])
            }
        }
        else {
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
        Self { cursor: SourceCursor::new_at(source, stable_offset), tokens: reused_tokens, errors: vec![], end_limit: None }
    }

    /// Creates a sub-state for scanning a sub-range of the source.
    pub fn sub_state(&mut self, start: usize, end: usize) -> Self {
        Self { cursor: SourceCursor::new_at(self.cursor.source(), start), tokens: vec![], errors: vec![], end_limit: Some(end) }
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
        self.end_limit.unwrap_or_else(|| self.cursor.source().length())
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

    /// Skips all ASCII hex digits (0-9, a-f, A-F).
    ///
    /// Uses SIMD acceleration if available on the platform.
    /// Returns the range of the skipped hex digits.
    #[inline]
    pub fn skip_ascii_hexdigits(&mut self) -> std::range::Range<usize> {
        let start = self.get_position();
        let rest = self.rest_bytes();
        let skipped = crate::source::SimdScanner::skip_ascii_hexdigits(rest);
        self.advance(skipped);
        (start..self.get_position()).into()
    }

    /// Finds the first occurrence of the target byte in the remaining text.
    ///
    /// Uses SIMD acceleration if available on the platform.
    /// Returns the byte offset relative to the current position, or None if not found.
    #[inline]
    pub fn find_byte(&mut self, target: u8) -> Option<usize> {
        let rest = self.rest_bytes();
        crate::source::SimdScanner::find_byte(rest, target)
    }

    /// Finds the first occurrence of any of the 4 bytes in the remaining text.
    ///
    /// Uses SIMD acceleration if available on the platform.
    /// Returns the byte offset relative to the current position, or None if not found.
    #[inline]
    pub fn find_first_of_4(&mut self, a: u8, b: u8, c: u8, d: u8) -> Option<usize> {
        let rest = self.rest_bytes();
        crate::source::SimdScanner::find_first_of_4(rest, a, b, c, d)
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
            self.add_token(kind, start, self.get_position());
            return true;
        }
        false
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
    /// state.take_while(|_| true);
    /// state.add_eof();
    ///
    /// assert_eq!(state.get_tokens().len(), 1);
    /// assert_eq!(state.get_tokens()[0].span, Range { start: 4, end: 4 });
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
    /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// let token = Token { kind: SimpleToken::Identifier, span: Range { start: 0, end: 5 } }
    ///
    /// assert_eq!(state.get_position(), 0);
    ///
    /// let new_pos = state.advance_with(token);
    ///
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
    /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// # enum SimpleToken { End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// let range = state.take_while(|c| c.is_alphabetic());
    ///
    /// assert_eq!(range, Range { start: 0, end: 5 });
    /// assert_eq!(state.get_position(), 5);
    ///
    /// let range = state.take_while(|c| c.is_numeric());
    ///
    /// assert_eq!(range, Range { start: 5, end: 8 });
    /// assert_eq!(state.get_position(), 8);
    /// ```
    ///
    /// # Performance Note
    ///
    /// This method operates on a character-by-character basis, which means it
    /// correctly handles multi-byte UTF-8 characters. For performance-critical
    /// code, consider using byte-based methods when working with ASCII-only text.
    pub fn take_while(&mut self, pred: impl FnMut(char) -> bool) -> Range<usize> {
        self.cursor.take_while(pred)
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
    /// #
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
    /// let safe_point = state.get_position();
    ///
    /// state.advance_if_dead_lock(safe_point);
    ///
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
    ///     if let Some(token) = try_recognize_token(&mut state) {
    ///         continue;
    ///     }
    ///
    ///     state.advance_if_dead_lock(safe_point);
    ///
    ///     if state.not_at_end() {
    ///         continue;
    ///     } else {
    ///         break;
    ///     }
    /// }
    /// ```
    pub fn advance_if_dead_lock(&mut self, safe_point: usize) {
        if self.cursor.position() == safe_point {
            if let Some(ch) = self.peek() { self.advance(ch.len_utf8()) } else { self.advance(1) }
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
    /// #
    /// # #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// state.add_token(SimpleToken::Identifier, 0, 4);
    ///
    /// let output = state.finish(Ok(()));
    ///
    /// assert!(output.result.is_ok());
    /// assert_eq!(output.result.unwrap().len(), 1);
    /// assert_eq!(output.diagnostics.len(), 0);
    ///
    /// let source2 = SourceText::new("test");
    /// let mut state2 = LexerState::<_, SimpleLanguage>::new(&source2);
    /// state2.add_error(OakError::custom_error("Test error"));
    ///
    /// let output2 = state2.finish(Err(OakError::custom_error("Fatal error")));
    ///
    /// assert!(output2.result.is_err());
    /// assert_eq!(output2.diagnostics.len(), 1);
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
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// # enum SimpleToken { Identifier, End }
    /// #
    /// # impl TokenType for SimpleToken {
    /// #     const END_OF_STREAM: Self = SimpleToken::End;
    /// #     type Role = UniversalTokenRole;
    /// #     fn role(&self) -> Self::Role { UniversalTokenRole::None }
    /// # }
    /// #
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    /// # #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// let mut cache = ParseSession::<SimpleLanguage>::new(16);
    ///
    /// state.add_token(SimpleToken::Identifier, 0, 4);
    ///
    /// let output = state.finish_with_cache(Ok(()), &mut cache);
    ///
    /// assert!(output.result.is_ok());
    /// assert_eq!(output.result.unwrap().len(), 1);
    /// ```
    ///
    /// # Incremental Lexing Workflow
    ///
    /// This method is typically used as part of an incremental lexing workflow:
    ///
    /// ```ignore
    /// let mut state = LexerState::new_with_cache(source, source.length(), cache);
    /// let output = state.finish_with_cache(Ok(()), cache);
    ///
    /// let relex_from = calculate_min_affected_offset(old_source, new_source);
    /// let mut state = LexerState::new_with_cache(new_source, relex_from, cache);
    /// let output = state.finish_with_cache(Ok(()), cache);
    /// ```
    pub fn finish_with_cache(self, result: Result<(), OakError>, cache: &mut impl LexerCache<L>) -> LexOutput<L> {
        let out = self.finish(result);
        cache.set_lex_output(out.clone());
        out
    }
}

use crate::OakDiagnostics;
