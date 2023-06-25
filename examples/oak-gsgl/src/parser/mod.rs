/// Element type definitions.
pub mod element_type;

use crate::language::GsglLanguage;
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, GsglLanguage, S>;

/// Parser for the GSGL language.
pub struct GsglParser {
    /// The language configuration.
    pub(crate) config: GsglLanguage,
}

impl GsglParser {
    /// Creates a new `GsglParser`.
    pub fn new(config: GsglLanguage) -> Self {
        Self { config }
    }
}

impl Parser<GsglLanguage> for GsglParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<GsglLanguage>) -> ParseOutput<'a, GsglLanguage> {
        let lexer = crate::lexer::GsglLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(crate::lexer::token_type::GsglTokenType::Struct) {
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

            Ok(state.finish_at(checkpoint, crate::parser::element_type::GsglElementType::Root))
        })
    }
}

impl GsglParser {
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(crate::lexer::token_type::GsglTokenType::Whitespace) || state.at(crate::lexer::token_type::GsglTokenType::Newline) || state.at(crate::lexer::token_type::GsglTokenType::Comment) {
            state.bump();
        }
    }

    fn is_at_type<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> bool {
        match state.current().map(|t| t.kind) {
            Some(k) => {
                let val = k as u8;
                // Basic types are from Float (84) to Void (107)
                (val >= 84 && val <= 107) || k == crate::lexer::token_type::GsglTokenType::Identifier
            }
            None => false,
        }
    }

    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(crate::lexer::token_type::GsglTokenType::Struct).ok();
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::GsglTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::GsglTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(crate::lexer::token_type::GsglTokenType::RightBrace) {
                if self.is_at_type(state) {
                    self.parse_variable_declaration(state);
                }
                else {
                    state.advance();
                }
                self.skip_trivia(state);
            }
            if state.at(crate::lexer::token_type::GsglTokenType::RightBrace) {
                state.bump();
            }
        }

        if state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
            state.bump();
        }

        state.finish_at(cp, crate::parser::element_type::GsglElementType::StructDecl);
    }

    fn parse_declaration_or_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();

        // type
        state.bump();
        self.skip_trivia(state);

        // name
        if state.at(crate::lexer::token_type::GsglTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);

        if state.at(crate::lexer::token_type::GsglTokenType::LeftParen) {
            // function
            self.parse_parameter_list(state);
            self.skip_trivia(state);

            if state.at(crate::lexer::token_type::GsglTokenType::LeftBrace) {
                self.parse_block(state);
            }
            else if state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
                state.bump();
            }
            state.finish_at(cp, crate::parser::element_type::GsglElementType::FunctionDecl);
        }
        else {
            // variable
            while state.not_at_end() && !state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
                state.bump();
            }
            state.finish_at(cp, crate::parser::element_type::GsglElementType::VariableDecl);
        }
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.bump(); // type
        self.skip_trivia(state);
        if state.at(crate::lexer::token_type::GsglTokenType::Identifier) {
            state.bump();
        }
        self.skip_trivia(state);
        if state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
            state.bump();
        }
        state.finish_at(cp, crate::parser::element_type::GsglElementType::VariableDecl);
    }

    fn parse_parameter_list<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        state.expect(crate::lexer::token_type::GsglTokenType::LeftParen).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(crate::lexer::token_type::GsglTokenType::RightParen) {
            self.parse_parameter(state);
            self.skip_trivia(state);
            if state.at(crate::lexer::token_type::GsglTokenType::Comma) {
                state.bump();
                self.skip_trivia(state);
            }
        }

        if state.at(crate::lexer::token_type::GsglTokenType::RightParen) {
            state.bump();
        }
    }

    fn parse_parameter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if self.is_at_type(state) {
            state.bump(); // type
            self.skip_trivia(state);
        }
        if state.at(crate::lexer::token_type::GsglTokenType::Identifier) {
            state.bump(); // name
        }
        state.finish_at(cp, crate::parser::element_type::GsglElementType::Parameter);
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(crate::lexer::token_type::GsglTokenType::LeftBrace).ok();
        self.skip_trivia(state);

        while state.not_at_end() && !state.at(crate::lexer::token_type::GsglTokenType::RightBrace) {
            self.parse_statement(state);
            self.skip_trivia(state);
        }

        if state.at(crate::lexer::token_type::GsglTokenType::RightBrace) {
            state.bump();
        }
        state.finish_at(cp, crate::parser::element_type::GsglElementType::Block);
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        if state.at(crate::lexer::token_type::GsglTokenType::LeftBrace) {
            self.parse_block(state);
        }
        else {
            while state.not_at_end() && !state.at(crate::lexer::token_type::GsglTokenType::Semicolon) && !state.at(crate::lexer::token_type::GsglTokenType::RightBrace) {
                state.bump();
            }
            if state.at(crate::lexer::token_type::GsglTokenType::Semicolon) {
                state.bump();
            }
        }
        state.finish_at(cp, crate::parser::element_type::GsglElementType::Statement);
    }
}
