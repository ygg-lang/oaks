pub mod element_type;
use crate::language::CrystalLanguage;
pub use element_type::CrystalElementType;
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, CrystalLanguage, S>;

mod parse_top_level;

pub struct CrystalParser<'config> {
    pub(crate) _config: &'config CrystalLanguage,
}

impl<'config> CrystalParser<'config> {
    pub fn new(config: &'config CrystalLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<CrystalLanguage> for CrystalParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CrystalLanguage>) -> ParseOutput<'a, CrystalLanguage> {
        let lexer = crate::lexer::CrystalLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
