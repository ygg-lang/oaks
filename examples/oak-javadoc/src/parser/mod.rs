use crate::{language::JavadocLanguage, lexer::JavadocLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, JavadocLanguage, S>;

pub struct JavadocParser<'config> {
    pub(crate) config: &'config JavadocLanguage,
}

impl<'config> JavadocParser<'config> {
    pub fn new(config: &'config JavadocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<JavadocLanguage> for JavadocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavadocLanguage>) -> ParseOutput<'a, JavadocLanguage> {
        let lexer = JavadocLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
