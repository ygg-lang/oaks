pub mod element_type;

use crate::{language::VLangLanguage, lexer::VLangLexer};
use oak_core::{
    parser::{ParseCache, Parser, ParserState},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, VLangLanguage, S>;

pub struct VLangParser<'config> {
    pub(crate) config: &'config VLangLanguage,
}

impl<'config> VLangParser<'config> {
    pub fn new(config: &'config VLangLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VLangLanguage> for VLangParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VLangLanguage>) -> oak_core::ParseOutput<'a, VLangLanguage> {
        let lexer = VLangLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}
