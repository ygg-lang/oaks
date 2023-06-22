use crate::{kind::WolframSyntaxKind, language::WolframLanguage, lexer::WolframLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

/// Wolfram Parser
#[derive(Debug, Clone)]
pub struct WolframParser<'config> {
    config: &'config WolframLanguage,
}

impl<'config> WolframParser<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<WolframLanguage> for WolframParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<WolframLanguage>) -> ParseOutput<'a, WolframLanguage> {
        let lexer = WolframLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                // Very simplified parsing for now
                state.advance();
            }

            let root = state.finish_at(checkpoint, WolframSyntaxKind::Root.into());
            Ok(root)
        })
    }
}
