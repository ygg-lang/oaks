use crate::{language::DockerfileLanguage, lexer::DockerfileLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, DockerfileLanguage, S>;

pub struct DockerfileParser<'config> {
    pub(crate) config: &'config DockerfileLanguage,
}

impl<'config> DockerfileParser<'config> {
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DockerfileLanguage> for DockerfileParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DockerfileLanguage>) -> ParseOutput<'a, DockerfileLanguage> {
        let lexer = DockerfileLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
