/// Element types for the IDL language.
pub mod element_type;

use crate::{language::IdlLanguage, lexer::IdlLexer, parser::element_type::IdlElementType};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::Source,
};

/// A parser for the IDL language.
pub struct IdlParser<'config> {
    pub(crate) config: &'config IdlLanguage,
}

impl<'config> IdlParser<'config> {
    /// Creates a new IDL parser with the given configuration.
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<IdlLanguage> for IdlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<IdlLanguage>) -> ParseOutput<'a, IdlLanguage> {
        let lexer = IdlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.bump();
            }

            Ok(state.finish_at(checkpoint, IdlElementType::Module))
        })
    }
}
