pub mod element_type;
pub use element_type::XmlElementType;

use crate::{language::XmlLanguage, lexer::XmlLexer};
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::Source,
};

mod parse_top_level;

pub struct XmlParser<'config> {
    pub(crate) config: &'config XmlLanguage,
}

impl<'config> XmlParser<'config> {
    pub fn new(config: &'config XmlLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<XmlLanguage> for XmlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<XmlLanguage>) -> ParseOutput<'a, XmlLanguage> {
        let lexer = XmlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
