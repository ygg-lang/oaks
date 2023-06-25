/// Element types for YAML.
pub mod element_type;

use crate::{language::YamlLanguage, lexer::YamlLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, YamlLanguage, S>;

/// Parser for YAML.
pub struct YamlParser<'config> {
    pub(crate) config: &'config YamlLanguage,
}

impl<'config> YamlParser<'config> {
    /// Creates a new YAML parser.
    pub fn new(config: &'config YamlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<YamlLanguage> for YamlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<YamlLanguage>) -> ParseOutput<'a, YamlLanguage> {
        let lexer = YamlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, element_type::YamlElementType::Root))
        })
    }
}
