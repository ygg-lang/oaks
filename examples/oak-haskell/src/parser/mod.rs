use crate::{language::HaskellLanguage, lexer::HaskellLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser},
    source::Source,
};

mod parse;

pub struct HaskellParser<'config> {
    pub(crate) config: &'config HaskellLanguage,
}

impl<'config> HaskellParser<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<HaskellLanguage> for HaskellParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HaskellLanguage>) -> oak_core::ParseOutput<'a, HaskellLanguage> {
        let lexer = HaskellLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
