mod pratt_parser;
pub use self::pratt_parser::{Associativity, OperatorInfo, PrattParser, Precedence};
use crate::{GreenBuilder, GreenNode, IncrementalCache, Language, OakDiagnostics, Token, errors::OakError, source::Source};
use triomphe::Arc;

/// Output type for parsing operations, containing either a green kind tree or errors.
pub type ParseOutput<L: Language> = OakDiagnostics<Arc<GreenNode<L::SyntaxKind>>>;

/// Parser trait for converting tokens into kind trees.
///
/// This trait provides a unified interface for parsing source text into
/// green kind trees, supporting both full parsing and incremental updates.
pub trait Parser<L: Language> {
    /// Parses source text into a kind tree.
    ///
    /// This method performs a full parse of the source text, creating a new
    /// kind tree from scratch. It uses a default cache configuration.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to parse
    ///
    /// # Returns
    ///
    /// A parse output containing either the green kind tree or errors
    fn parse(&self, text: impl Source) -> ParseOutput<L> {
        let mut pool = GreenBuilder::new(0);
        let cache = IncrementalCache::new(&mut pool);
        self.parse_incremental(text, 0, cache)
    }

    /// Parses source text incrementally using an existing cache.
    ///
    /// This method enables efficient re-parsing by reusing information from
    /// previous parsing operations, only processing the changed portions
    /// of the source text.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to parse
    /// * `changed` - The number of bytes that have changed since the last parse
    /// * `cache` - The incremental cache containing previous parsing results
    ///
    /// # Returns
    ///
    /// A parse output containing either the green kind tree or errors
    fn parse_incremental(&self, text: impl Source, changed: usize, cache: IncrementalCache<L>) -> ParseOutput<L>;
}

/// Generic parsing state that encapsulates cursor for token stream and error aggregation.
///
/// This struct maintains the current parsing position and provides utilities for
/// consuming tokens, recording errors, and building kind trees incrementally.
///
/// # Examples
///
/// ```rust
/// use core::range::Range;
/// use oak_core::{SourceText, Token, parser::ParserState};
///
/// #[derive(Copy, Clone, PartialEq)]
/// enum K {
///     A,
///     B,
///     Eof,
/// }
///
/// let source = SourceText::new("ab");
/// let tokens = [
///     Token { kind: K::A, span: Range { start: 0, end: 1 } },
///     Token { kind: K::B, span: Range { start: 1, end: 2 } },
///     Token { kind: K::Eof, span: Range { start: 2, end: 2 } },
/// ];
/// let mut st = ParserState::new_with_cache(&source, &tokens);
/// assert!(st.match_kind(&[K::A]));
/// assert!(st.match_kind(&[K::B]));
/// let out = st.finish(Ok(()));
/// assert!(out.diagnostics.is_empty());
/// ```
pub struct ParserState<'a, S: Source, L: Language> {
    /// The source text being parsed
    pub source: S,
    /// The incremental cache containing tokens and previous parse results
    pub cache: IncrementalCache<'a, L>,
    /// Current position in the token stream
    pub index: usize,
    /// Collection of errors encountered during parsing
    pub errors: Vec<OakError>,
}

impl<'a, S: Source, L: Language> ParserState<'a, S, L> {
    /// Creates a new parser state with the given source text and tokens.
    #[inline]
    pub fn new_with_cache(source: S, change: usize, cache: IncrementalCache<'a, L>) -> Self {
        Self { cache, source, index: 0, errors: Vec::new() }
    }

    /// Checks if there are more tokens to consume.
    ///
    /// # Returns
    ///
    /// `true` if there are more tokens to parse, `false` otherwise
    pub fn not_at_end(&self) -> bool {
        self.index < self.cache.count_tokens()
    }

    /// Returns the current token without consuming it.
    ///
    /// # Returns
    ///
    /// An optional reference to the current token, or `None` if at end of stream
    #[inline]
    pub fn current(&self) -> Option<&Token<L::SyntaxKind>> {
        self.cache.get_token(self.index)
    }

    /// Returns the previous token (the one before the current position).
    ///
    /// # Returns
    ///
    /// An optional reference to the previous token, or `None` if at start of stream
    #[inline]
    pub fn previous(&self) -> Option<&Token<L::SyntaxKind>> {
        if self.index > 0 { self.cache.get_token(self.index - 1) } else { None }
    }

    /// Advances to the next token and returns it.
    ///
    /// # Returns
    ///
    /// An optional reference to the consumed token, or `None` if at end of stream
    #[inline]
    pub fn advance(&mut self) -> Option<&Token<L::SyntaxKind>> {
        if self.not_at_end() {
            let i = self.index;
            self.index += 1;
            self.cache.get_token(i)
        }
        else {
            None
        }
    }

    /// Returns the kind of the current token without consuming it.
    ///
    /// # Returns
    ///
    /// An optional token kind, or `None` if at end of stream
    #[inline]
    pub fn peek_kind(&self) -> Option<L::SyntaxKind> {
        self.current().map(|t| t.kind)
    }

    /// Checks if the current token matches any of the given kinds and consumes it if so.
    ///
    /// # Arguments
    ///
    /// * `kinds` - Array of token kinds to match against
    ///
    /// # Returns
    ///
    /// `true` if the current token was consumed (matched), `false` otherwise
    #[inline]
    pub fn match_kind(&mut self, kinds: &[L::SyntaxKind]) -> bool {
        if let Some(t) = self.current() {
            if kinds.iter().any(|k| *k == t.kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Records a kind error at the specified byte position
    ///
    /// # Arguments
    ///
    /// * `position` - The byte position where the error occurred
    /// * `msg` - The error message to record
    pub fn record_error_at(&mut self, position: usize, msg: impl Into<String>) {
        let err = self.source.syntax_error(msg, position);
        self.errors.push(err);
    }

    /// Records an "unexpected current kind" error
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message to record
    pub fn record_unexpected(&mut self, msg: impl Into<String>) {
        let pos = self.current().map(|t| t.span.start).unwrap_or(self.source.length());
        self.record_error_at(pos, msg);
    }

    /// Consumes an expected kind; if it doesn't match, records an error and returns None (suitable for error recovery)
    ///
    /// # Arguments
    ///
    /// * `kind` - The expected token kind
    /// * `msg` - The error message to record if the token doesn't match
    ///
    /// # Returns
    ///
    /// An optional token if the expected kind was found and consumed, `None` otherwise
    pub fn consume(&mut self, kind: L::SyntaxKind, msg: impl Into<String>) -> Option<Token<L::SyntaxKind>> {
        if let Some(t) = self.current() {
            if t.kind == kind {
                let tok = t.clone();
                self.index += 1;
                return Some(tok);
            }
        }
        self.record_unexpected(msg);
        None
    }

    /// Finishes parsing and returns the final parse output.
    ///
    /// This method consumes the parser state and returns a parse output containing
    /// either the successfully parsed green tree or parsing errors.
    ///
    /// # Arguments
    ///
    /// * `result` - The parsing result (Ok for success, Err for failure)
    ///
    /// # Returns
    ///
    /// A parse output containing the green tree or errors
    pub fn finish(self, result: Result<(), OakError>) -> ParseOutput<L> {
        match result {
            Ok(_) => {
                if let Some(root) = self.cache.last_parse {
                    OakDiagnostics { result: Ok(root), diagnostics: self.errors }
                }
                else {
                    OakDiagnostics {
                        result: Err(OakError::custom_error("Parser finished without building a root node")),
                        diagnostics: self.errors,
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: self.errors },
        }
    }
}
