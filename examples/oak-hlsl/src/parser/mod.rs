pub mod element_type;
pub use element_type::HlslElementType;

use crate::{language::HlslLanguage, lexer::HlslLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, HlslLanguage, S>;

pub struct HlslParser<'config> {
    pub(crate) _config: &'config HlslLanguage,
}

impl<'config> HlslParser<'config> {
    pub fn new(config: &'config HlslLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<HlslLanguage> for HlslParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HlslLanguage>) -> ParseOutput<'a, HlslLanguage> {
        let lexer = HlslLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
