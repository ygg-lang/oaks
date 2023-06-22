use crate::{language::GraphQLLanguage, lexer::GraphQLLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, GraphQLLanguage, S>;

pub struct GraphQLParser<'config> {
    pub(crate) config: &'config GraphQLLanguage,
}

impl<'config> GraphQLParser<'config> {
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<GraphQLLanguage> for GraphQLParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GraphQLLanguage>) -> ParseOutput<'a, GraphQLLanguage> {
        let lexer = GraphQLLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
