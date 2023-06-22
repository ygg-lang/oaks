use crate::{language::LLvmLanguage, lexer::LlvmLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse;

pub struct LlirParser<'config> {
    pub(crate) _config: &'config LLvmLanguage,
}

impl<'config> LlirParser<'config> {
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<LLvmLanguage> for LlirParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LLvmLanguage>) -> ParseOutput<'a, LLvmLanguage> {
        let lexer = LlvmLexer;
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
