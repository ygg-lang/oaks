/// Element type definitions.
pub mod element_type;

use crate::{language::VLangLanguage, lexer::VLangLexer, parser::element_type::VLangElementType};
use oak_core::{
    parser::{ParseCache, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Parser for the V language.
pub struct VLangParser<'config> {
    pub(crate) config: &'config VLangLanguage,
}

impl<'config> VLangParser<'config> {
    /// Creates a new `VLangParser`.
    pub fn new(config: &'config VLangLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VLangLanguage> for VLangParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VLangLanguage>) -> oak_core::ParseOutput<'a, VLangLanguage> {
        let lexer = VLangLexer::new(&self.config);
        parse_with_lexer(&lexer, source, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, VLangElementType::SourceFile))
        })
    }
}
