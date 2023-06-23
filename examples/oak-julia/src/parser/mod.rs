pub mod element_type;

use crate::{language::JuliaLanguage, lexer::JuliaLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, JuliaLanguage, S>;

pub struct JuliaParser<'config> {
    pub(crate) config: &'config JuliaLanguage,
}

impl<'config> JuliaParser<'config> {
    pub fn new(config: &'config JuliaLanguage) -> Self {
        Self { config }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.not_at_end() {
            if let Some(kind) = state.peek_kind() {
                if kind.is_trivia() {
                    state.bump();
                    continue;
                }
            }
            break;
        }
    }
}

impl<'config> Parser<JuliaLanguage> for JuliaParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JuliaLanguage>) -> ParseOutput<'a, JuliaLanguage> {
        let lexer = JuliaLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
