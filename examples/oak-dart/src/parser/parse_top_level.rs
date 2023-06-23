use crate::{
    language::DartLanguage,
    lexer::token_type::DartTokenType,
    parser::{DartParser, State},
};
use oak_core::{GreenNode, OakError, source::Source};

impl<'config> DartParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, DartLanguage>, OakError> {
        let cp = (0, 0); // Ensure the root node includes initial trivia skipped during state initialization
        while state.not_at_end() {
            if state.at(DartTokenType::Class) {
                let class_cp = state.checkpoint();
                state.bump(); // class
                if state.at(DartTokenType::Identifier) {
                    state.bump(); // Name
                }
                if state.at(DartTokenType::LeftBrace) {
                    state.bump();
                    while state.not_at_end() && !state.at(DartTokenType::RightBrace) {
                        state.bump();
                    }
                    state.eat(DartTokenType::RightBrace);
                }
                state.finish_at(class_cp, crate::parser::element_type::DartElementType::ClassDeclaration);
            }
            else if state.at(DartTokenType::Void) || state.at(DartTokenType::Int) || state.at(DartTokenType::Dynamic) {
                let cp = state.checkpoint();
                state.bump(); // type

                if state.at(DartTokenType::Identifier) {
                    state.bump(); // Name
                }

                if state.at(DartTokenType::LeftParen) {
                    // Function declaration
                    state.bump();
                    while state.not_at_end() && !state.at(DartTokenType::RightParen) {
                        state.bump();
                    }
                    state.eat(DartTokenType::RightParen);

                    if state.at(DartTokenType::LeftBrace) {
                        state.bump();
                        while state.not_at_end() && !state.at(DartTokenType::RightBrace) {
                            state.bump();
                        }
                        state.eat(DartTokenType::RightBrace);
                    }
                    state.finish_at(cp, crate::parser::element_type::DartElementType::FunctionDeclaration);
                }
                else {
                    // Variable declaration
                    if state.eat(DartTokenType::Equal) {
                        while state.not_at_end() && !state.at(DartTokenType::Semicolon) {
                            state.bump();
                        }
                    }
                    state.eat(DartTokenType::Semicolon);
                    state.finish_at(cp, crate::parser::element_type::DartElementType::VariableDeclaration);
                }
            }
            else {
                state.bump();
            }
        }

        Ok(state.finish_at(cp, crate::parser::element_type::DartElementType::Root))
    }
}
