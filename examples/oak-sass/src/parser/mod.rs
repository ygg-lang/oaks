use crate::{language::SassLanguage, lexer::SassLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, SassLanguage, S>;

pub struct SassParser<'config> {
    pub(crate) config: &'config SassLanguage,
}

impl<'config> SassParser<'config> {
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<SassLanguage> for SassParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SassLanguage>) -> ParseOutput<'a, SassLanguage> {
        let lexer = SassLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
