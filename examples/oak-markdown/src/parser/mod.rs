use crate::{kind::MarkdownSyntaxKind, language::MarkdownLanguage};
use oak_core::{GreenNode, OakError, Parser, ParserState, source::Source};

pub(crate) type State<'a, S> = ParserState<'a, MarkdownLanguage, S>;

pub struct MarkdownParser<'config> {
    pub(crate) config: &'config MarkdownLanguage,
}

impl<'config> MarkdownParser<'config> {
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MarkdownLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            state.advance();
        }

        let root = state.finish_at(checkpoint, MarkdownSyntaxKind::Root.into());
        Ok(root)
    }
}

impl<'config> Parser<MarkdownLanguage> for MarkdownParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[oak_core::TextEdit], cache: &'a mut impl oak_core::ParseCache<MarkdownLanguage>) -> oak_core::ParseOutput<'a, MarkdownLanguage> {
        let lexer = crate::lexer::MarkdownLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
