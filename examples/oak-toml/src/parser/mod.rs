#![doc = include_str!("readme.md")]
/// Element types for the TOML parser.
pub mod element_type;
use crate::{TomlBuilder, ast::TomlRoot, language::TomlLanguage, lexer::token_type::TomlTokenKind as TomlSyntaxKind};
use oak_core::{Builder, OakError, ParseCache, Parser, Source, parser::session::ParseSession, source::SourceText};

/// TOML language parser.
///
/// Implements the `Parser` trait for the TOML language, providing
/// basic parsing functionality by leveraging the Oak core's lexer-based parsing.
pub struct TomlParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config TomlLanguage,
}

impl<'config> TomlParser<'config> {
    /// Creates a new `TomlParser` with the given configuration.
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<TomlLanguage> for TomlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<TomlLanguage>) -> oak_core::ParseOutput<'a, TomlLanguage> {
        let lexer = crate::lexer::TomlLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            // Simple parsing logic: consume all tokens and put into Root node
            while state.current().is_some() {
                state.bump()
            }

            Ok(state.finish_at(checkpoint, TomlSyntaxKind::Root.into()))
        })
    }
}

/// Parses a TOML string.
pub fn parse(toml: &str) -> Result<TomlRoot, OakError> {
    let language = TomlLanguage::default();
    parse_with_config(toml, &language)
}

/// Parses a TOML string.
pub fn parse_with_config(toml: &str, config: &TomlLanguage) -> Result<TomlRoot, OakError> {
    let builder = TomlBuilder::new(&config);
    let source = SourceText::new(toml);
    let mut cache = ParseSession::default();
    let result = builder.build(&source, &[], &mut cache);
    result.result
}
