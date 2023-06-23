#![doc = include_str!("readme.md")]
pub mod element_type;

use crate::{
    language::ScalaLanguage,
    lexer::{ScalaLexer, token_type::ScalaTokenType},
    parser::element_type::ScalaElementType,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, ScalaLanguage, S>;

pub struct ScalaParser<'config> {
    pub(crate) config: &'config ScalaLanguage,
}

impl<'config> ScalaParser<'config> {
    pub fn new(config: &'config ScalaLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ScalaLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance()
        }

        Ok(state.finish_at(checkpoint, ScalaElementType::SourceFile.into()))
    }
}

impl<'config> Parser<ScalaLanguage> for ScalaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ScalaLanguage>) -> ParseOutput<'a, ScalaLanguage> {
        let lexer = ScalaLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
