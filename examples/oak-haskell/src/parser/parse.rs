use crate::{kind::HaskellSyntaxKind, language::HaskellLanguage, parser::HaskellParser};
use oak_core::{GreenNode, errors::OakError, parser::ParserState, source::Source};

type State<'a, S> = ParserState<'a, HaskellLanguage, S>;

impl<'config> HaskellParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, HaskellLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            self.parse_item(state)?;
        }

        Ok(state.finish_at(checkpoint, HaskellSyntaxKind::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        // Skip for now to satisfy compiler
        state.bump();
        Ok(())
    }
}
