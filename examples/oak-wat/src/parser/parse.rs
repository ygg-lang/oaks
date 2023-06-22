use crate::{
    kind::WatSyntaxKind,
    language::WatLanguage,
    parser::{State, WatParser},
};
use oak_core::{OakError, source::Source, tree::GreenNode};

impl<'config> WatParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, WatLanguage>, OakError> {
        let cp = state.checkpoint();
        state.bump(); // Start node implicitly via first token or manually if needed, but here we just process tokens

        while state.not_at_end() {
            if state.at(WatSyntaxKind::LeftParen) {
                self.parse_item(state);
            }
            else {
                state.bump();
            }
        }
        Ok(state.finish_at(cp, WatSyntaxKind::Root))
    }

    fn parse_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WatSyntaxKind::LeftParen).ok();

        if state.at(WatSyntaxKind::ModuleKw) {
            state.expect(WatSyntaxKind::ModuleKw).ok();
            while state.not_at_end() && !state.at(WatSyntaxKind::RightParen) {
                if state.at(WatSyntaxKind::LeftParen) {
                    self.parse_item(state);
                }
                else {
                    state.bump();
                }
            }
            state.expect(WatSyntaxKind::RightParen).ok();
            state.finish_at(cp, WatSyntaxKind::Module);
        }
        else {
            // Simplified: just skip other items for now
            while state.not_at_end() && !state.at(WatSyntaxKind::RightParen) {
                state.bump();
            }
            state.expect(WatSyntaxKind::RightParen).ok();
            state.finish_at(cp, WatSyntaxKind::Item);
        }
    }
}
