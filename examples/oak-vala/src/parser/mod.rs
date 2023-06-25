/// Vala syntax element types.
pub mod element_type;

use crate::{language::ValaLanguage, lexer::ValaLexer, parser::element_type::ValaElementType};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

/// Vala parser state.
pub(crate) type State<'a, S> = ParserState<'a, ValaLanguage, S>;

/// Vala language parser.
pub struct ValaParser<'config> {
    pub(crate) config: &'config ValaLanguage,
}

impl<'config> ValaParser<'config> {
    /// Creates a new `ValaParser` with the given language configuration.
    pub fn new(config: &'config ValaLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<ValaLanguage> for ValaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ValaLanguage>) -> ParseOutput<'a, ValaLanguage> {
        let lexer = ValaLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.advance();
            }

            Ok(state.finish_at(checkpoint, ValaElementType::SourceFile))
        })
    }
}
