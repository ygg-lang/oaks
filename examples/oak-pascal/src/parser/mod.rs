/// Element type definitions for Pascal.
pub mod element_type;

use crate::{language::PascalLanguage, lexer::PascalLexer, parser::element_type::PascalElementType};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser},
    source::Source,
};

/// A parser for Pascal source files.
pub struct PascalParser;

impl PascalParser {
    /// Creates a new Pascal parser.
    pub fn new(config: &PascalLanguage) -> Self {
        Self
    }
}

impl Parser<PascalLanguage> for PascalParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PascalLanguage>) -> oak_core::ParseOutput<'a, PascalLanguage> {
        let language = PascalLanguage::default();
        let lexer = PascalLexer::new(&language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                state.bump();
            }
            Ok(state.finish_at(checkpoint, PascalElementType::Root))
        })
    }
}
