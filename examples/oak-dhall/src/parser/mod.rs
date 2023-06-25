/// Element type module for DHall.
pub mod element_type;

use crate::{language::DHallLanguage, lexer::DHallLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Internal parser state for DHall.
pub(crate) type State<'a, S> = ParserState<'a, DHallLanguage, S>;

/// Parser implementation for DHall.
pub struct DHallParser<'config> {
    pub(crate) config: &'config DHallLanguage,
}

impl<'config> DHallParser<'config> {
    /// Creates a new `DHallParser`.
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DHallLanguage> for DHallParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DHallLanguage>) -> ParseOutput<'a, DHallLanguage> {
        let lexer = DHallLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::DHallElementType::Root))
        })
    }
}
