pub mod element_type;

use crate::{language::WatLanguage, lexer::WatLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, WatLanguage, S>;

/// Wat Parser
pub struct WatParser<'config> {
    pub(crate) config: &'config WatLanguage,
}

impl<'config> WatParser<'config> {
    /// Creates a new Wat parser
    pub fn new(config: &'config WatLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WatLanguage> for WatParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<WatLanguage>) -> ParseOutput<'a, WatLanguage> {
        let lexer = WatLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
