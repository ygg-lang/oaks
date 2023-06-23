use crate::{
    language::MsilLanguage,
    lexer::token_type::MsilTokenType,
    parser::{MsilParser, State},
};
use oak_core::{GreenNode, OakError};

impl<'config> MsilParser<'config> {
    fn skip_trivia<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if kind == MsilTokenType::Whitespace || kind == MsilTokenType::CommentToken {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    pub(crate) fn parse_root_internal<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, MsilLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            self.skip_trivia(state);
            if !state.not_at_end() {
                break;
            }

            if state.at(MsilTokenType::AssemblyKeyword) {
                self.parse_assembly(state);
            }
            else if state.at(MsilTokenType::ModuleKeyword) {
                self.parse_module(state);
            }
            else if state.at(MsilTokenType::ClassKeyword) {
                self.parse_class(state);
            }
            else {
                state.bump();
            }
        }
        Ok(state.finish_at(cp, crate::parser::element_type::MsilElementType::Root))
    }

    fn parse_assembly<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .assembly
        self.skip_trivia(state);

        let is_extern = if state.at(MsilTokenType::ExternKeyword) {
            state.bump(); // extern
            self.skip_trivia(state);
            true
        }
        else {
            false
        };

        if state.at(MsilTokenType::IdentifierToken) {
            let id_cp = state.checkpoint();
            state.bump();
            state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
            self.skip_trivia(state);
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        if is_extern {
            state.finish_at(cp, crate::parser::element_type::MsilElementType::AssemblyExtern);
        }
        else {
            state.finish_at(cp, crate::parser::element_type::MsilElementType::Assembly);
        }
    }

    fn parse_module<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .module
        self.skip_trivia(state);

        if state.at(MsilTokenType::IdentifierToken) {
            let id_cp = state.checkpoint();
            state.bump();
            state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
        }

        state.finish_at(cp, crate::parser::element_type::MsilElementType::Module);
    }

    fn parse_class<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .class
        self.skip_trivia(state);

        // Skip modifiers
        while state.at(MsilTokenType::PublicKeyword) || state.at(MsilTokenType::PrivateKeyword) || state.at(MsilTokenType::StaticKeyword) || state.at(MsilTokenType::Keyword) {
            state.bump();
            self.skip_trivia(state);
        }

        if state.at(MsilTokenType::IdentifierToken) {
            let id_cp = state.checkpoint();
            state.bump();
            state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
            self.skip_trivia(state);
        }

        // Handle extends
        if (state.at(MsilTokenType::IdentifierToken) || state.at(MsilTokenType::Keyword)) && state.peek_text().as_deref() == Some("extends") {
            state.bump();
            self.skip_trivia(state);
            while state.not_at_end() && !state.at(MsilTokenType::LeftBrace) {
                state.bump();
            }
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                self.skip_trivia(state);
                if !state.not_at_end() || state.at(MsilTokenType::RightBrace) {
                    break;
                }

                if state.at(MsilTokenType::MethodKeyword) {
                    self.parse_method(state);
                }
                else {
                    state.bump();
                }
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, crate::parser::element_type::MsilElementType::Class);
    }

    fn parse_method<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // .method
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(MsilTokenType::LeftBrace) {
            if state.at(MsilTokenType::IdentifierToken) {
                let id_cp = state.checkpoint();
                state.bump();
                state.finish_at(id_cp, crate::parser::element_type::MsilElementType::Identifier);
                self.skip_trivia(state);
            }
            else {
                state.bump();
            }
        }

        if state.at(MsilTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
            if state.at(MsilTokenType::RightBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, crate::parser::element_type::MsilElementType::Method);
    }
}
