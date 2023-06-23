pub mod element_type;

use crate::{language::MermaidLanguage, lexer::MermaidLexer, parser::element_type::MermaidElementType};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

type State<'a, S> = ParserState<'a, MermaidLanguage, S>;

/// Mermaid parser implementation.
pub struct MermaidParser<'a> {
    pub(crate) _language: &'a MermaidLanguage,
}

impl<'a> MermaidParser<'a> {
    pub fn new(language: &'a MermaidLanguage) -> Self {
        Self { _language: language }
    }
}

impl<'p> Parser<MermaidLanguage> for MermaidParser<'p> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<MermaidLanguage>) -> ParseOutput<'a, MermaidLanguage> {
        let lexer = MermaidLexer::new(self._language);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'p> MermaidParser<'p> {
    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MermaidLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.not_at_end() {
            state.bump();
        }

        Ok(state.finish_at(cp, MermaidElementType::Root))
    }
}
