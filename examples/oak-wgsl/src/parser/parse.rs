use crate::{
    kind::WgslSyntaxKind,
    language::WgslLanguage,
    parser::{State, WgslParser},
};
use oak_core::{OakError, source::Source, tree::GreenNode};

impl<'config> WgslParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, WgslLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.at(WgslSyntaxKind::FnKw) {
                self.parse_function(state);
            }
            else if state.at(WgslSyntaxKind::StructKw) {
                self.parse_struct(state);
            }
            else if state.at(WgslSyntaxKind::VarKw) || state.at(WgslSyntaxKind::LetKw) {
                self.parse_variable(state);
            }
            else {
                state.bump();
            }
        }
        Ok(state.finish_at(cp, WgslSyntaxKind::Root.into()))
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslSyntaxKind::FnKw).ok();
        state.expect(WgslSyntaxKind::Identifier).ok();
        state.expect(WgslSyntaxKind::LeftParen).ok();
        // Simplified param parsing
        while state.not_at_end() && !state.at(WgslSyntaxKind::RightParen) {
            state.bump();
        }
        state.expect(WgslSyntaxKind::RightParen).ok();

        if state.eat(WgslSyntaxKind::Arrow) {
            state.expect(WgslSyntaxKind::Identifier).ok();
        }

        self.parse_block(state);
        state.finish_at(cp, WgslSyntaxKind::Function.into());
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslSyntaxKind::StructKw).ok();
        state.expect(WgslSyntaxKind::Identifier).ok();
        state.expect(WgslSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslSyntaxKind::RightBrace) {
            state.bump();
        }
        state.expect(WgslSyntaxKind::RightBrace).ok();
        state.finish_at(cp, WgslSyntaxKind::Struct.into());
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if state.at(WgslSyntaxKind::VarKw) {
            state.expect(WgslSyntaxKind::VarKw).ok();
        }
        else {
            state.expect(WgslSyntaxKind::LetKw).ok();
        }
        state.expect(WgslSyntaxKind::Identifier).ok();
        if state.eat(WgslSyntaxKind::Colon) {
            state.expect(WgslSyntaxKind::Identifier).ok();
        }
        if state.eat(WgslSyntaxKind::Assign) {
            // Simplified expression parsing
            while state.not_at_end() && !state.at(WgslSyntaxKind::Semicolon) {
                state.bump();
            }
        }
        state.expect(WgslSyntaxKind::Semicolon).ok();
        state.finish_at(cp, WgslSyntaxKind::Variable.into());
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslSyntaxKind::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslSyntaxKind::RightBrace) {
            state.bump();
        }
        state.expect(WgslSyntaxKind::RightBrace).ok();
        state.finish_at(cp, WgslSyntaxKind::Block.into());
    }
}
