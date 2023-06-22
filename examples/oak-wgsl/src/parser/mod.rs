use crate::language::WgslLanguage;
use oak_core::{
    parser::{Parser, ParserState},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, WgslLanguage, S>;

/// WGSL Parser
pub struct WgslParser<'config> {
    pub(crate) config: &'config WgslLanguage,
}

impl<'config> WgslParser<'config> {
    /// Creates a new WGSL parser
    pub fn new(config: &'config WgslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WgslLanguage> for WgslParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<WgslLanguage>) -> oak_core::ParseOutput<'a, WgslLanguage> {
        let lexer = crate::lexer::WgslLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
