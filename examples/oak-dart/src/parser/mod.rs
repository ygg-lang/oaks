pub mod element_type;

use crate::{
    language::DartLanguage,
    lexer::{DartLexer, token_type::DartTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, DartLanguage, S>;

pub struct DartParser<'config> {
    pub(crate) config: &'config DartLanguage,
}

impl<'config> DartParser<'config> {
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<DartLanguage> for DartParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DartLanguage>) -> ParseOutput<'a, DartLanguage> {
        let lexer = DartLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
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
        })
    }
}
