mod pratt_parser;

pub use self::pratt_parser::PrattParser;
#[cfg(feature = "std")]
use crate::helpers::source_from_path;
use crate::{
    GreenNode, Language, SourceText, Token,
    errors::{OakDiagnostics, OakError},
    source::TextEdit,
};
use alloc::{boxed::Box, rc::Rc};
use core::range::Range;

/// 解析输出：根绿树 + 错误
pub type ParseOutput<K: Copy> = OakDiagnostics<Rc<GreenNode<K>>>;

/// Parser trait for constructing kind trees.
///
/// This trait defines the interface for parsing tokens into abstract kind trees
/// (ASTs) that represent the structure of the source code.
pub trait Parser<L: Language> {
    /// Parses the entire source text into a kind tree.
    ///
    /// This method performs a complete parse of the source text, converting it
    /// into an abstract kind tree (AST) that represents the code structure.
    /// It handles the entire parsing process from start to finish.
    ///
    /// # Parameters
    ///
    /// * `source` - The source text to parse
    ///
    /// # Returns
    ///
    /// A `ParseOutput` containing the parsed kind tree and any language encountered
    ///
    /// # Examples
    ///
    /// ```
    /// let source = SourceText::new("let x = 42;");
    /// let result = parser.parse(&source);
    /// ```
    fn parse(&self, source: &SourceText) -> ParseOutput<L::SyntaxKind>;
    /// Parses a range of tokens from the source text, e.g. java docs parsing.
    ///
    /// # Parameters
    ///
    /// * `source` - The source text to parse
    /// * `range` - The range of tokens to parse, specified as a half-open range `[start, end)`
    ///
    /// # Returns
    ///
    /// A `ParseOutput` containing the parsed kind tree and any language encountered
    fn parse_range(&self, source: &SourceText, range: Range<usize>) -> ParseOutput<L::SyntaxKind> {
        self.parse(&source.slice(range))
    }
    /// Parses a pre-tokenized sequence of tokens into a kind tree.
    ///
    /// This method is useful for incremental parsing scenarios where tokens have already
    /// been lexed and cached. It bypasses the tokenization step and directly parses
    /// the provided kind sequence.
    ///
    /// # Parameters
    ///
    /// * `source` - The original source text (used for error reporting and location information)
    /// * `tokens` - The pre-tokenized sequence of tokens to parse
    ///
    /// # Returns
    ///
    /// A `ParseOutput` containing the parsed kind tree and any language encountered
    ///
    /// # Examples
    ///
    /// ```
    /// let tokens = vec![Token::new(SyntaxKind::Let, 0..3), Token::new(SyntaxKind::Ident, 4..5)];
    /// let result = parser.parse_tokens(&source, &tokens);
    /// ```
    fn parse_tokens(&self, source: &SourceText, tokens: &[Token<L::SyntaxKind>]) -> ParseOutput<L::SyntaxKind>;
    #[cfg(feature = "std")]
    fn parse_path(&self, path: &std::path::Path) -> ParseOutput<L::SyntaxKind> {
        let source = match source_from_path(path) {
            Ok(o) => o,
            Err(e) => {
                return OakDiagnostics { result: Err(e), diagnostics: Vec::new() };
            }
        };
        self.parse(&source)
    }
}

/// Rebuilder trait for incremental parsing and kind tree reconstruction.
///
/// This trait extends the parser with capabilities for incremental parsing,
/// allowing efficient updates to kind trees when source code changes.
/// Instead of reparsing the entire file, the rebuilder can identify affected
/// portions and rebuild only those sections.
///
/// # Type Parameters
///
/// * `L` - The language type that this rebuilder operates on
///
/// # Implementation Notes
///
/// Rebuilders are crucial for performance in interactive environments like
/// IDEs and text editors where files are frequently modified. They work by:
///
/// 1. Identifying which parts of the kind tree are affected by text changes
/// 2. Reparsing only the affected regions
/// 3. Integrating the new parse results back into the existing tree
///
/// This approach significantly reduces parsing overhead compared to full reparsing.
pub trait IncrementalParser<L: Language>: Parser<L> {
    /// Parses source code incrementally using a cached kind tree.
    ///
    /// This method enables efficient parsing when source code has been modified,
    /// by reusing parts of the previous kind tree that are unaffected by the changes.
    /// It identifies the affected regions and reparses only those portions,
    /// significantly improving performance compared to full reparsing.
    ///
    /// # Parameters
    ///
    /// * `cache` - The cached green node from a previous parse, or `None` if no cache is available
    /// * `source` - The source text to parse
    /// * `changed` - The byte offset where the change occurred in the source text
    ///
    /// # Returns
    ///
    /// A `ParseOutput` containing the updated kind tree and any language encountered during parsing
    ///
    /// # Examples
    ///
    /// ```
    /// let cache = previous_parse_result;
    /// let source = SourceText::new("let x = 42;");
    /// let result = parser.parse_incremental(Some(cache), &source, 8);
    /// ```
    fn parse_incremental(
        &self,
        cache: Option<Rc<GreenNode<L::SyntaxKind>>>,
        source: &SourceText,
        changed: usize,
    ) -> ParseOutput<L::SyntaxKind>;
}

/// 操作符优先级
pub type Precedence = u8;

/// Operator associativity defines how operators of the same precedence are grouped.
///
/// Associativity determines whether an expression like `a + b + c` is interpreted as
/// `(a + b) + c` (left-associative) or `a + (b + c)` (right-associative).
/// Non-associative operators cannot be chained together without explicit parentheses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    /// Left-associative operators group from left to right.
    /// For example, subtraction is left-associative: `a - b - c` means `(a - b) - c`
    Left,
    /// Right-associative operators group from right to left.
    /// For example, assignment is right-associative: `a = b = c` means `a = (b = c)`
    Right,
    /// Non-associative operators cannot be chained without parentheses.
    /// For example, comparison operators are typically non-associative
    None,
}

/// Operator information containing precedence and associativity.
///
/// This structure encapsulates the essential properties needed for operator
/// precedence parsing, including the precedence level and how operators of the
/// same precedence should be grouped.
#[derive(Debug, Clone)]
pub struct OperatorInfo {
    /// The precedence level of the operator (higher values have higher precedence)
    pub precedence: Precedence,
    /// The associativity of the operator (Left, Right, or None)
    pub associativity: Associativity,
}

impl OperatorInfo {
    /// Creates a new operator with the specified precedence and associativity.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator (higher values have higher precedence)
    /// * `associativity` - The associativity of the operator (Left, Right, or None)
    ///
    /// # Examples
    ///
    /// ```
    /// let op = OperatorInfo::new(10, Associativity::Left);
    /// ```
    pub fn new(precedence: Precedence, associativity: Associativity) -> Self {
        Self { precedence, associativity }
    }

    /// Creates a left-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let left_op = OperatorInfo::left(5);
    /// ```
    pub fn left(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::Left)
    }

    /// Creates a right-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let right_op = OperatorInfo::right(8);
    /// ```
    pub fn right(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::Right)
    }

    /// Creates a non-associative operator with the specified precedence.
    ///
    /// # Arguments
    ///
    /// * `precedence` - The precedence level of the operator
    ///
    /// # Examples
    ///
    /// ```
    /// let non_assoc_op = OperatorInfo::none(3);
    /// ```
    pub fn none(precedence: Precedence) -> Self {
        Self::new(precedence, Associativity::None)
    }
}

/// 通用的解析状态，封装对 kind 流的游标与错误聚合
///
/// # 示例
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
/// let mut st = ParserState::new(&source, &tokens);
/// assert!(st.match_kind(&[K::A]));
/// assert!(st.match_kind(&[K::B]));
/// let out = st.diagnostics(());
/// assert!(out.diagnostics.is_empty());
/// ```
pub struct ParserState<'a, K: Copy> {
    pub source: &'a SourceText,
    pub tokens: &'a [Token<K>],
    pub index: usize,
    pub errors: alloc::vec::Vec<OakError>,
}

impl<'a, K: Copy> ParserState<'a, K> {
    #[inline]
    pub fn new(source: &'a SourceText, tokens: &'a [Token<K>]) -> Self {
        Self { source, tokens, index: 0, errors: alloc::vec::Vec::new() }
    }

    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    #[inline]
    pub fn current(&self) -> Option<&Token<K>> {
        self.tokens.get(self.index)
    }

    #[inline]
    pub fn previous(&self) -> Option<&Token<K>> {
        if self.index > 0 { self.tokens.get(self.index - 1) } else { None }
    }

    #[inline]
    pub fn advance(&mut self) -> Option<&Token<K>> {
        if self.is_at_end() {
            None
        }
        else {
            let i = self.index;
            self.index += 1;
            self.tokens.get(i)
        }
    }

    #[inline]
    pub fn peek_kind(&self) -> Option<K> {
        self.current().map(|t| t.kind)
    }

    #[inline]
    pub fn match_kind(&mut self, kinds: &[K]) -> bool
    where
        K: PartialEq,
    {
        if let Some(t) = self.current() {
            if kinds.iter().any(|k| *k == t.kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// 记录一个语法错误（按指定字节位置）
    pub fn record_error_at(&mut self, position: usize, msg: impl Into<alloc::string::String>) {
        let err = self.source.syntax_error(msg, position);
        self.errors.push(err);
    }

    /// 记录“意外的当前 kind”错误
    pub fn record_unexpected(&mut self, msg: impl Into<alloc::string::String>) {
        let pos = self.current().map(|t| t.span.start).unwrap_or(self.source.len());
        self.record_error_at(pos, msg);
    }

    /// 消耗一个期望的 kind；不匹配则记录错误并返回 None（适合错误恢复）
    pub fn consume(&mut self, kind: K, msg: impl Into<alloc::string::String>) -> Option<Token<K>>
    where
        K: PartialEq + Clone,
    {
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

    pub fn diagnostics(self, result: Rc<GreenNode<K>>) -> ParseOutput<K> {
        OakDiagnostics { result: Ok(result), diagnostics: self.errors }
    }
}
