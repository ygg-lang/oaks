pub mod element_type;
pub use element_type::VampireElementType;

use crate::{
    language::VampireLanguage,
    lexer::{VampireLexer, VampireTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, VampireLanguage, S>;

pub struct VampireParser<'config> {
    pub(crate) config: &'config VampireLanguage,
}

impl<'config> VampireParser<'config> {
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VampireLanguage> for VampireParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VampireLanguage>) -> ParseOutput<'a, VampireLanguage> {
        let lexer = VampireLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                state.bump()
            }
            Ok(state.finish_at(checkpoint, VampireElementType::Root))
        })
    }
}
