pub mod element_type;
pub use element_type::HlslElementType;

use crate::{language::HlslLanguage, lexer::HlslLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, HlslLanguage, S>;

pub struct HlslParser<'config> {
    pub(crate) config: &'config HlslLanguage,
}

impl<'config> HlslParser<'config> {
    pub fn new(config: &'config HlslLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<HlslLanguage> for HlslParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HlslLanguage>) -> ParseOutput<'a, HlslLanguage> {
        let lexer = HlslLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(crate::lexer::token_type::HlslTokenType::Struct) {
                    self.parse_struct(state);
                }
                else if self.is_at_type(state) {
                    self.parse_declaration_or_function(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }

            Ok(state.finish_at(checkpoint, HlslElementType::Root))
        })
    }
}

impl<'config> HlslParser<'config> {
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(crate::lexer::token_type::HlslTokenType::Whitespace) || state.at(crate::lexer::token_type::HlslTokenType::Newline) || state.at(crate::lexer::token_type::HlslTokenType::Comment) {
            state.bump();
        }
    }

    fn is_at_type<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> bool {
        match state.peek_kind() {
            Some(k) => {
                let val = k as u8;
                // Basic types are from Bool (42) to Double4x4 (92)
                (val >= 42 && val <= 92) || k == crate::lexer::token_type::HlslTokenType::Identifier
            }
            None => false,
        }
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(crate::lexer::token_type::HlslTokenType::Struct).ok();
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::HlslTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::HlslTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(crate::lexer::token_type::HlslTokenType::RightBrace) {
                if self.is_at_type(state) {
                    self.parse_variable_declaration(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }
            if state.at(crate::lexer::token_type::HlslTokenType::RightBrace) {
                state.bump();
            }
        }

        if state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
            state.bump();
        }

        state.finish_at(cp, HlslElementType::StructDeclaration);
    }

    fn parse_declaration_or_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        // type
        state.bump();
        self.skip_trivia(state);

        // name
        if state.at(crate::lexer::token_type::HlslTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::HlslTokenType::LeftParen) {
            // function
            self.parse_parameter_list(state);
            self.skip_trivia(state);

            // semantic
            if state.at(crate::lexer::token_type::HlslTokenType::Colon) {
                state.bump();
                self.skip_trivia(state);
                if state.at(crate::lexer::token_type::HlslTokenType::Identifier) {
                    state.bump();
                }
            }
            self.skip_trivia(state);

            if state.at(crate::lexer::token_type::HlslTokenType::LeftBrace) {
                self.parse_block(state);
            }
            else if state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
                state.bump();
            }
            state.finish_at(cp, HlslElementType::FunctionDeclaration);
        }
        else {
            // variable
            while state.not_at_end() && !state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
                state.bump();
            }
            state.finish_at(cp, HlslElementType::VariableDeclaration);
        }
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // type
        self.skip_trivia(state);
        if state.at(crate::lexer::token_type::HlslTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);
        if state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
            state.bump();
        }
        state.finish_at(cp, HlslElementType::VariableDeclaration);
    }

    fn parse_parameter_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(crate::lexer::token_type::HlslTokenType::LeftParen).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(crate::lexer::token_type::HlslTokenType::RightParen) {
            self.parse_parameter(state);
            self.skip_trivia(state);
            if state.at(crate::lexer::token_type::HlslTokenType::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        if state.at(crate::lexer::token_type::HlslTokenType::RightParen) {
            state.bump();
        }
        state.finish_at(cp, HlslElementType::ParameterList);
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if self.is_at_type(state) {
            state.bump(); // type
            self.skip_trivia(state);
        }
        if state.at(crate::lexer::token_type::HlslTokenType::Identifier) {
            state.bump(); // name
        }
        state.finish_at(cp, HlslElementType::Parameter);
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(crate::lexer::token_type::HlslTokenType::LeftBrace).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(crate::lexer::token_type::HlslTokenType::RightBrace) {
            self.parse_statement(state);
            self.skip_trivia(state);
        }

        if state.at(crate::lexer::token_type::HlslTokenType::RightBrace) {
            state.bump();
        }
        state.finish_at(cp, HlslElementType::Block);
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if state.at(crate::lexer::token_type::HlslTokenType::LeftBrace) {
            self.parse_block(state);
        }
        else {
            while state.not_at_end() && !state.at(crate::lexer::token_type::HlslTokenType::Semicolon) && !state.at(crate::lexer::token_type::HlslTokenType::RightBrace) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::HlslTokenType::Semicolon) {
                state.bump();
            }
        }
        state.finish_at(cp, HlslElementType::Statement);
    }
}
