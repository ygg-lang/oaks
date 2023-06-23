pub mod element_type;

use crate::{language::HandlebarsLanguage, lexer::HandlebarsLexer, parser::element_type::HandlebarsElementType};
use oak_core::{
    Parser,
    parser::{ParseCache, ParseOutput, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Handlebars 语言解析器
pub struct HandlebarsParser<'config> {
    /// 语言配置
    pub(crate) config: &'config HandlebarsLanguage,
}

impl<'config> HandlebarsParser<'config> {
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<HandlebarsLanguage> for HandlebarsParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HandlebarsLanguage>) -> ParseOutput<'a, HandlebarsLanguage> {
        let lexer = HandlebarsLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

mod parse_top_level;
