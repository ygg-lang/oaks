use crate::{language::PythonLanguage, lexer::PythonLexer};
use oak_core::{
    Source,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::TextEdit,
};

mod element_type;
mod parse_comprehensions;
mod parse_expressions;
mod parse_statements;
mod parse_utils;

pub use element_type::PythonElementType;

/// Python parser.
pub struct PythonParser<'config> {
    /// The Python language configuration.
    config: &'config PythonLanguage,
}

impl<'config> PythonParser<'config> {
    /// Create a new Python parser.
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<PythonLanguage> for PythonParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PythonLanguage>) -> ParseOutput<'a, PythonLanguage> {
        let lexer = PythonLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while !state.at(crate::lexer::PythonTokenType::Eof) && state.peek_kind().is_some() {
                self.parse_statement(state);
                self.skip_trivia(state);
            }
            Ok(state.finish_at(cp, PythonElementType::Module))
        })
    }
}
