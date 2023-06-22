pub mod element_type;
pub mod parse;
use crate::{language::CsvLanguage, lexer::CsvLexer};
pub use element_type::CsvElementType;
use oak_core::{
    ParseOutput,
    parser::{ParseCache, Parser, ParserState},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CsvLanguage, S>;

pub struct CsvParser<'config> {
    config: &'config CsvLanguage,
}

impl<'config> CsvParser<'config> {
    pub fn new(config: &'config CsvLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<CsvLanguage> for CsvParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CsvLanguage>) -> ParseOutput<'a, CsvLanguage> {
        let lexer = CsvLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
