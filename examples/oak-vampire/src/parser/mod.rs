/// Element type definitions for the Vampire parser.
pub mod element_type;
/// Re-export of VampireElementType from the element_type module.
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

/// Parser for the Vampire language.
pub struct VampireParser<'config> {
    pub(crate) config: &'config VampireLanguage,
}

impl<'config> VampireParser<'config> {
    /// Creates a new VampireParser with the given language configuration.
    pub fn new(config: &'config VampireLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VampireLanguage> for VampireParser<'config> {
    /// Parses the input text and produces a green tree for the Vampire language.
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
