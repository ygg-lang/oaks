#![doc = include_str!("readme.md")]
/// Rust token type definitions.
pub mod token_type;
pub use self::token_type::{RustToken, RustTokenType};
use crate::language::RustLanguage;
use oak_core::{Lexer, LexerCache, LexerState, Source, lexer::LexOutput};

mod lex;

/// A lexer for the Rust programming language.
///
/// The `RustLexer` is responsible for tokenizing Rust source code into a sequence of tokens
/// that can be used by the parser. It handles all Rust syntax including modern features like
/// raw strings, byte strings, lifetimes, and all standard Rust keywords.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,ignore
/// use oak_core::{Lexer, LexerState, SourceText};
/// use oak_rust::{RustLanguage, RustLexer};
///
/// let language = RustLanguage::default();
/// let lexer = RustLexer::new(&language);
/// let source = SourceText::new("fn main() { println!(\"Hello, world!\") }");
/// let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
/// let output = lexer.lex(&source, &[], &mut cache);
///
/// // The output contains tokens for the entire source code
/// assert!(output.result.is_ok());
/// ```
///
/// Tokenizing different Rust constructs:
///
/// ```rust,ignore
/// use oak_core::{Lexer, LexerState, SourceText};
/// use oak_rust::{RustLanguage, RustLexer};
///
/// let language = RustLanguage::default();
/// let lexer = RustLexer::new(&language);
///
/// // Tokenize a function with various Rust features
/// let source = SourceText::new(
///     r#"
/// fn calculate<'a>(x: &'a i32, y: i32) -> i32 {
///     let result = x + y;
///     println!("Result: {}", result);
///     result
/// }
/// "#,
/// );
/// let mut cache = oak_core::parser::session::ParseSession::<RustLanguage>::default();
/// let output = lexer.lex(&source, &[], &mut cache);
///
/// // Verify that tokens were generated
/// assert!(output.result.is_ok());
/// ```
#[derive(Clone)]
pub struct RustLexer<'config> {
    _config: &'config RustLanguage,
}

type State<'a, S> = LexerState<'a, S, RustLanguage>;

impl<'config> Lexer<RustLanguage> for RustLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<RustLanguage>) -> LexOutput<RustLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RustLexer<'config> {
    /// Creates a new `RustLexer` with the given language configuration.
    ///
    /// # Parameters
    ///
    /// * `config` - A `RustLanguage` configuration that controls
    ///   language-specific parsing behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use oak_rust::{RustLexer, RustLanguage};
    ///
    /// let language = RustLanguage::default();
    /// let lexer = RustLexer::new(&language);
    /// ```
    pub fn new(config: &'config RustLanguage) -> Self {
        Self { _config: config }
    }

    /// Internal method to run the lexer on the given state.
    /// This delegates to the implementation in the `lex` module.
    pub(crate) fn run<'s, S: Source + ?Sized>(&self, state: &mut LexerState<'s, S, RustLanguage>) -> Result<(), oak_core::OakError> {
        lex::run(self, state)
    }
}
