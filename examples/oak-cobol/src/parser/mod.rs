use crate::language::CobolLanguage;
pub mod element_type;
pub use element_type::CobolElementType;

use oak_core::{
    parser::{Parser, ParserState},
    source::{Source, TextEdit},
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, CobolLanguage, S>;

pub struct CobolParser<'config> {
    pub(crate) _config: &'config CobolLanguage,
}

impl<'config> CobolParser<'config> {
    pub fn new(config: &'config CobolLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<CobolLanguage> for CobolParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<CobolLanguage>) -> oak_core::ParseOutput<'a, CobolLanguage> {
        let lexer = crate::lexer::CobolLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
