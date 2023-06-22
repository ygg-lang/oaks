use crate::{language::VhdlLanguage, lexer::VhdlLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, VhdlLanguage, S>;

pub struct VhdlParser<'config> {
    pub(crate) config: &'config VhdlLanguage,
}

impl<'config> VhdlParser<'config> {
    pub fn new(config: &'config VhdlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VhdlLanguage> for VhdlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VhdlLanguage>) -> ParseOutput<'a, VhdlLanguage> {
        let lexer = VhdlLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
