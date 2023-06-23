#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::BashElementType;

use crate::{
    language::BashLanguage,
    lexer::{BashLexer, BashTokenType},
};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, BashLanguage, S>;

pub struct BashParser<'config> {
    pub(crate) _config: &'config BashLanguage,
}

impl<'config> BashParser<'config> {
    pub fn new(config: &'config BashLanguage) -> Self {
        Self { _config: config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(BashTokenType::Keyword) {
            // Need to check which keyword, but our Lexer currently marks all as Keyword
            // For now, just advance and mark as a generic statement if we don't have detailed keyword types
            let checkpoint = state.checkpoint();
            state.bump();
            while state.not_at_end() && !state.at(BashTokenType::Newline) && !state.at(BashTokenType::Delimiter) {
                state.bump()
            }
            state.finish_at(checkpoint, BashElementType::CommandStatement);
        }
        else {
            self.parse_command(state)?
        }
        Ok(())
    }

    pub(crate) fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() && !state.at(BashTokenType::Newline) && !state.at(BashTokenType::Delimiter) {
            state.bump()
        }
        state.finish_at(checkpoint, BashElementType::CommandStatement);
        Ok(())
    }
}

impl<'config> Parser<BashLanguage> for BashParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<BashLanguage>) -> oak_core::ParseOutput<'a, BashLanguage> {
        let lexer = BashLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() && !state.at(BashTokenType::Eof) {
                if state.at(BashTokenType::Newline) || state.at(BashTokenType::Delimiter) {
                    state.bump()
                }
                else {
                    self.parse_statement(state).ok();
                }
            }

            Ok(state.finish_at(checkpoint, BashElementType::Root))
        })
    }
}
