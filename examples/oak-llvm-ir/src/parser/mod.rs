//! Parser implementation for LLVM IR.

/// Element types for the LLVM IR parser.
pub mod element_type;

use crate::{language::LLvmLanguage, lexer::LLvmLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

/// Parser for LLVM IR.
pub struct LLirParser<'config> {
    pub(crate) _config: &'config LLvmLanguage,
}

impl<'config> LLirParser<'config> {
    /// Creates a new LLVM IR parser with the given language configuration.
    pub fn new(config: &'config LLvmLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<LLvmLanguage> for LLirParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LLvmLanguage>) -> ParseOutput<'a, LLvmLanguage> {
        let lexer = LLvmLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
