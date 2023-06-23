pub mod element_type;

use crate::{language::PascalLanguage, lexer::PascalLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, PascalLanguage, S>;

pub struct PascalParser;

impl PascalParser {
    pub fn new(_config: &PascalLanguage) -> Self {
        Self
    }
}

impl Parser<PascalLanguage> for PascalParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PascalLanguage>) -> oak_core::ParseOutput<'a, PascalLanguage> {
        let language = PascalLanguage::default();
        let lexer = PascalLexer::new(&language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
