use crate::{
    language::WgslLanguage,
    lexer::token_type::WgslTokenType,
    parser::{State, WgslParser, element_type::WgslElementType},
};
use oak_core::{OakError, source::Source, tree::GreenNode};

impl<'config> WgslParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, WgslLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.at(WgslTokenType::FnKw) {
                self.parse_function(state);
            }
            else if state.at(WgslTokenType::StructKw) {
                self.parse_struct(state);
            }
            else if state.at(WgslTokenType::VarKw) || state.at(WgslTokenType::LetKw) {
                self.parse_variable(state);
            }
            else {
                state.bump();
            }
        }
        Ok(state.finish_at(cp, WgslElementType::Root))
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::FnKw).ok();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::LeftParen).ok();
        // Simplified param parsing
        while state.not_at_end() && !state.at(WgslTokenType::RightParen) {
            state.bump()
        }
        state.expect(WgslTokenType::RightParen).ok();

        if state.eat(WgslTokenType::Arrow) {
            state.expect(WgslTokenType::Identifier).ok();
        }

        self.parse_block(state);
        state.finish_at(cp, WgslElementType::Function);
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::StructKw).ok();
        state.expect(WgslTokenType::Identifier).ok();
        state.expect(WgslTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslTokenType::RightBrace) {
            state.bump()
        }
        state.expect(WgslTokenType::RightBrace).ok();
        state.finish_at(cp, WgslElementType::Struct);
    }

    fn parse_variable<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if state.at(WgslTokenType::VarKw) {
            state.expect(WgslTokenType::VarKw).ok();
        }
        else {
            state.expect(WgslTokenType::LetKw).ok();
        }
        state.expect(WgslTokenType::Identifier).ok();
        if state.eat(WgslTokenType::Colon) {
            state.expect(WgslTokenType::Identifier).ok();
        }
        if state.eat(WgslTokenType::Assign) {
            // Simplified expression parsing
            while state.not_at_end() && !state.at(WgslTokenType::Semicolon) {
                state.bump()
            }
        }
        state.expect(WgslTokenType::Semicolon).ok();
        state.finish_at(cp, WgslElementType::Variable);
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(WgslTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(WgslTokenType::RightBrace) {
            state.bump()
        }
        state.expect(WgslTokenType::RightBrace).ok();
        state.finish_at(cp, WgslElementType::Block);
    }
}
