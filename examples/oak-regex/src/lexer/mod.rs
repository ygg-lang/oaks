use crate::language::RegexLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};

mod lex;

type State<'a, S> = LexerState<'a, S, RegexLanguage>;

/// Lexer for regular expressions.
///
/// `RegexLexer` is responsible for tokenizing regular expression source code into a series of tokens
/// that can be used by the parser. It handles all regex syntax including character classes,
/// quantifiers, groups, assertions, and special characters.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use oak_core::{Lexer, LexerCache, LexerState, ParseSession, SourceText};
/// use oak_regex::{RegexLanguage, RegexLexer};
///
/// let language = RegexLanguage::default();
/// let lexer = RegexLexer::new(&language);
/// let source = SourceText::new(r"[a-z]+\d{1,3}");
/// let mut cache = ParseSession::<RegexLanguage>::default();
/// let output = lexer.lex(&source, &[], &mut cache);
///
/// // Output contains tokens for the entire source
/// assert!(!output.result.unwrap().is_empty());
/// ```
///
/// Tokenizing different regex constructs:
///
/// ```
/// use oak_core::{Lexer, LexerCache, LexerState, ParseSession, SourceText};
/// use oak_regex::{RegexLanguage, RegexLexer};
///
/// let language = RegexLanguage::default();
/// let lexer = RegexLexer::new(&language);
///
/// // Tokenize a complex regular expression
/// let source = SourceText::new(r"(?:(?:[a-zA-Z0-9-]+\.)+[a-zA-Z]{2,})");
/// let mut cache = ParseSession::<RegexLanguage>::default();
/// let output = lexer.lex(&source, &[], &mut cache);
///
/// // Verify tokens were generated
/// assert!(output.result.unwrap().len() > 5);
/// ```
#[derive(Clone)]
pub struct RegexLexer;

impl Lexer<RegexLanguage> for RegexLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<RegexLanguage>) -> LexOutput<RegexLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl RegexLexer {
    /// Creates a new `RegexLexer` with the given language configuration.
    ///
    /// # Arguments
    ///
    /// * `_config` - A reference to the `RegexLanguage` configuration that controls
    ///   language-specific parsing behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use oak_regex::{RegexLexer, RegexLanguage};
    ///
    /// let language = RegexLanguage::default();
    /// let lexer = RegexLexer::new(&language);
    /// ```
    pub fn new(_config: &RegexLanguage) -> Self {
        Self
    }

    /// Returns the whitespace configuration for the lexer.
    ///
    /// This method defines how the lexer should handle whitespace characters.
    /// The configuration enables Unicode whitespace support, allowing the lexer
    /// to recognize all Unicode whitespace characters, not just ASCII spaces.
    pub fn whitespace_rules(&self) -> &WhitespaceConfig {
        &WhitespaceConfig { unicode_whitespace: true }
    }

    /// Returns the comment configuration for the lexer.
    ///
    /// This method defines how the lexer should handle comments in regular expressions.
    /// Regular expressions typically use `#` as a line comment marker, with comments
    /// continuing to the end of the line.
    pub fn comment_rules(&self) -> CommentConfig {
        CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false }
    }

    /// Returns the string literal configuration for the lexer.
    ///
    /// This method defines how the lexer should handle string literals in regular expressions.
    /// Regex strings are typically enclosed in double quotes (") and use backslash (\) as escape character.
    pub fn string_rules(&self) -> StringConfig {
        StringConfig { quotes: &['"'], escape: Some('\\') }
    }

    /// Returns the character literal configuration for the lexer.
    ///
    /// This method defines how the lexer should handle character literals in regular expressions.
    /// Regex character literals are enclosed in single quotes (') and do not use escape characters
    /// in the same way as strings.
    pub fn char_rules(&self) -> StringConfig {
        StringConfig { quotes: &['\''], escape: None }
    }
}
