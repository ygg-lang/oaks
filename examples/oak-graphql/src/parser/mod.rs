/// Element type definitions for GraphQL.
pub mod element_type;

use crate::{language::GraphQLLanguage, lexer::GraphQLLexer, parser::element_type::GraphQLElementType};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

/// A parser for GraphQL source files.
pub struct GraphQLParser<'config> {
    /// The language configuration.
    pub(crate) config: &'config GraphQLLanguage,
}

impl<'config> GraphQLParser<'config> {
    /// Creates a new GraphQL parser.
    pub fn new(config: &'config GraphQLLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<GraphQLLanguage> for GraphQLParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GraphQLLanguage>) -> ParseOutput<'a, GraphQLLanguage> {
        let lexer = GraphQLLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance()
            }

            Ok(state.finish_at(checkpoint, GraphQLElementType::SourceFile))
        })
    }
}
