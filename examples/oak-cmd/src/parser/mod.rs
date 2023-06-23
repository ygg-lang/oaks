#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::CmdElementType;

use crate::{
    language::CmdLanguage,
    lexer::{CmdLexer, CmdTokenType},
};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, CmdLanguage, S>;

pub struct CmdParser<'config> {
    pub(crate) _config: &'config CmdLanguage,
}

impl<'config> CmdParser<'config> {
    pub fn new(config: &'config CmdLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<CmdLanguage> for CmdParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CmdLanguage>) -> oak_core::ParseOutput<'a, CmdLanguage> {
        let lexer = CmdLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() && !state.at(CmdTokenType::Eof) {
                state.bump()
            }
            Ok(state.finish_at(checkpoint, CmdElementType::Root))
        })
    }
}
