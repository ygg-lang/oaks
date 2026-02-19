use oak_core::TokenType;

/// Java element types.
pub mod element_type;

mod parse_declaration;
mod parse_expression;
mod parse_statement;

use crate::{
    language::JavaLanguage,
    lexer::{JavaLexer, token_type::JavaTokenType},
    parser::element_type::JavaElementType,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, Parser, ParserState,
        pratt::{Pratt, PrattParser},
    },
    source::Source,
};

/// Parser state type alias for Java parsing
pub(crate) type State<'a, S> = ParserState<'a, JavaLanguage, S>;

/// Java parser implementation that handles Java source code parsing
///
/// This parser supports modern Java features including:
/// - Records
/// - Sealed classes
/// - Lambdas
/// - Annotations
/// - Generics
/// - Pattern matching
///
/// # Example
/// ```rust
/// use oak_java::{JavaLanguage, JavaParser, SourceText};
///
/// let code = r#"
/// public class Example {
///     public static void main(String[] args) {
///         System.out.println("Hello, World!");
///     }
/// }
/// "#;
///
/// let source = SourceText::new(code);
/// let config = JavaLanguage::new();
/// let parser = JavaParser::new(&config);
/// let result = parser.parse(&source);
///
/// assert!(result.is_success());
/// ```
pub struct JavaParser<'config> {
    /// Parser configuration containing language-specific settings
    pub(crate) config: &'config JavaLanguage,
}

impl<'config> JavaParser<'config> {
    /// Creates a new `JavaParser` with the given configuration
    ///
    /// # Parameters
    /// - `config`: The Java language configuration
    ///
    /// # Returns
    /// A new instance of `JavaParser`
    ///
    /// # Example
    /// ```rust
    /// use oak_java::{JavaLanguage, JavaParser};
    ///
    /// let config = JavaLanguage::new();
    /// let parser = JavaParser::new(&config);
    /// ```
    pub fn new(config: &'config JavaLanguage) -> Self {
        Self { config }
    }

    /// Skip trivia tokens (whitespace, comments)
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            let kind = state.peek_kind().unwrap();
            if kind.is_ignored() {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        parse_statement::parse_statement(self, state)
    }
}

impl<'config> Pratt<JavaLanguage> for JavaParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
        parse_expression::primary(self, state)
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
        parse_expression::prefix(self, state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, JavaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, JavaLanguage>> {
        parse_expression::infix(self, state, left, min_precedence)
    }
}

impl<'config> parse_declaration::DeclarationParser for JavaParser<'config> {}

impl<'config> Parser<JavaLanguage> for JavaParser<'config> {
    /// Parses Java source code into an abstract syntax tree
    ///
    /// # Parameters
    /// - `text`: The source text to parse
    /// - `edits`: Text edits for incremental parsing
    /// - `cache`: Parse cache for incremental parsing
    ///
    /// # Returns
    /// A parse output containing the syntax tree and diagnostics
    ///
    /// # Example
    /// ```rust
    /// use oak_core::parser::DefaultParseCache;
    /// use oak_java::{JavaLanguage, JavaParser, SourceText};
    ///
    /// let code = "public class Example { }";
    /// let source = SourceText::new(code);
    /// let config = JavaLanguage::new();
    /// let parser = JavaParser::new(&config);
    /// let mut cache = DefaultParseCache::default();
    ///
    /// let result = parser.parse(&source, &[], &mut cache);
    /// println!("Parse success: {}", result.is_success());
    /// ```
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavaLanguage>) -> oak_core::parser::ParseOutput<'a, JavaLanguage> {
        let lexer = JavaLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                self.parse_item(state).ok();
            }
            Ok(state.finish_at(checkpoint, JavaElementType::CompilationUnit))
        })
    }
}
