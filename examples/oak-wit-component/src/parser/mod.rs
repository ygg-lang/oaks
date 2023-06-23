pub mod element_type;

use crate::{
    language::WitLanguage,
    lexer::{WitLexer, token_type::WitTokenType},
    parser::element_type::WitElementType,
};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser},
    source::Source,
};

// type WitToken = Token<WitTokenType>;

/// WIT Parser
pub struct WitParser<'config> {
    pub(crate) config: &'config WitLanguage,
}

impl<'config> WitParser<'config> {
    /// Creates a new WIT parser
    pub fn new(config: &'config WitLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WitLanguage> for WitParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<WitLanguage>) -> oak_core::ParseOutput<'a, WitLanguage> {
        let lexer = WitLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                let _token = state.advance();
                // TODO: 完整的 WIT 解析逻辑
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::WitElementType::Root))
        })
    }
}
