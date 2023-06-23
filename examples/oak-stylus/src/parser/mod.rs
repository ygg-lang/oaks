pub mod element_type;

use crate::{
    language::StylusLanguage,
    lexer::{StylusLexer, token_type::StylusTokenType},
    parser::element_type::StylusElementType,
};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

pub struct StylusParser<'config> {
    pub(crate) config: &'config StylusLanguage,
}

impl<'config> StylusParser<'config> {
    pub fn new(config: &'config StylusLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<StylusLanguage> for StylusParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<StylusLanguage>) -> ParseOutput<'a, StylusLanguage> {
        let lexer = StylusLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                state.bump()
            }
            Ok(state.finish_at(checkpoint, crate::parser::element_type::StylusElementType::Root))
        })
    }
}
