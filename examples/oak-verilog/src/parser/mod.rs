#![doc = include_str!("readme.md")]

pub mod element_type;

pub use crate::lexer::token_type::VerilogKind as VerilogElementType;

use crate::{language::VerilogLanguage, lexer::VerilogLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, VerilogLanguage, S>;

pub struct VerilogParser<'config> {
    pub(crate) config: &'config VerilogLanguage,
}

impl<'config> VerilogParser<'config> {
    pub fn new(config: &'config VerilogLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<VerilogLanguage> for VerilogParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VerilogLanguage>) -> oak_core::ParseOutput<'a, VerilogLanguage> {
        let lexer = VerilogLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
