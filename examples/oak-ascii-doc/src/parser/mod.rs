use crate::language::AsciiDocLanguage;
use oak_core::{
    parser::{Parser, ParserState},
    source::{Source, TextEdit},
};

pub mod element_type;
mod parse;

pub use element_type::AsciiDocElementType;
pub type State<'a, S> = ParserState<'a, AsciiDocLanguage, S>;

pub struct AsciiDocParser<'config> {
    pub(crate) config: &'config AsciiDocLanguage,
}

impl<'config> AsciiDocParser<'config> {
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<AsciiDocLanguage> for AsciiDocParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<AsciiDocLanguage>) -> oak_core::ParseOutput<'a, AsciiDocLanguage> {
        let lexer = crate::lexer::AsciiDocLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
