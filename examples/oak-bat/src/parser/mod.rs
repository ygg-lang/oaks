#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::BatElementType;

use crate::{
    language::BatLanguage,
    lexer::{BatLexer, BatTokenType},
};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, BatLanguage, S>;

pub struct BatParser<'config> {
    pub(crate) _config: &'config BatLanguage,
}

impl<'config> BatParser<'config> {
    pub fn new(config: &'config BatLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<BatLanguage> for BatParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<BatLanguage>) -> oak_core::ParseOutput<'a, BatLanguage> {
        let lexer = BatLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() && !state.at(BatTokenType::Eof) {
                state.bump()
            }
            Ok(state.finish_at(checkpoint, BatElementType::Root))
        })
    }
}
