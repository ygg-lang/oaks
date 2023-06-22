use crate::{kind::TailwindSyntaxKind, language::TailwindLanguage, parser::State};
use oak_core::{errors::OakError, tree::GreenNode};

impl<'config> super::TailwindParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, TailwindLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            if state.at(TailwindSyntaxKind::DoubleLeftBrace) {
                self.parse_variable(state)?;
            }
            else if state.at(TailwindSyntaxKind::LeftBracePercent) {
                self.parse_tag(state)?;
            }
            else {
                state.advance();
            }
        }

        Ok(state.finish_at(checkpoint, TailwindSyntaxKind::Root))
    }

    fn parse_variable<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TailwindSyntaxKind::DoubleLeftBrace)?;

        while state.not_at_end() && !state.at(TailwindSyntaxKind::DoubleRightBrace) {
            state.advance();
        }

        state.expect(TailwindSyntaxKind::DoubleRightBrace)?;
        state.finish_at(checkpoint, TailwindSyntaxKind::Variable);
        Ok(())
    }

    fn parse_tag<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let checkpoint = state.checkpoint();
        state.expect(TailwindSyntaxKind::LeftBracePercent)?;

        while state.not_at_end() && !state.at(TailwindSyntaxKind::PercentRightBrace) {
            state.advance();
        }

        state.expect(TailwindSyntaxKind::PercentRightBrace)?;
        state.finish_at(checkpoint, TailwindSyntaxKind::Tag);
        Ok(())
    }
}
