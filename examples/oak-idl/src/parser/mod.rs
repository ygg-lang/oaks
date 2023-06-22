use crate::{language::IdlLanguage, lexer::IdlLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = oak_core::parser::ParserState<'a, IdlLanguage, S>;

pub struct IdlParser<'config> {
    pub(crate) config: &'config IdlLanguage,
}

impl<'config> IdlParser<'config> {
    pub fn new(config: &'config IdlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<IdlLanguage> for IdlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<IdlLanguage>) -> ParseOutput<'a, IdlLanguage> {
        let lexer = IdlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
