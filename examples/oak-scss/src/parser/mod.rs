pub mod element_type;

use crate::{
    language::ScssLanguage,
    lexer::{ScssLexer, token_type::ScssTokenType},
    parser::element_type::ScssElementType,
};
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, ScssLanguage, S>;

/// Parser for the SCSS language.
pub struct ScssParser<'config> {
    pub(crate) config: &'config ScssLanguage,
}

impl<'config> ScssParser<'config> {
    /// Creates a new `ScssParser` with the given configuration.
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { config }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(ScssTokenType::Dollar) => self.parse_variable_declaration(state),
            Some(ScssTokenType::At) => self.parse_at_rule(state),
            Some(ScssTokenType::Identifier) | Some(ScssTokenType::Dot) | Some(ScssTokenType::Hash) | Some(ScssTokenType::Star) => self.parse_rule(state),
            Some(ScssTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, ScssElementType::Comment);
                Ok(())
            }
            _ => {
                state.advance();
                Ok(())
            }
        }
    }

    fn parse_at_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ScssTokenType::At)?;

        if let Some(ScssTokenType::Identifier) = state.peek_kind() {
            let name = state.peek_text().map(|s| s.into_owned());
            state.bump();

            match name.as_deref() {
                Some("mixin") => self.parse_mixin(state, cp),
                Some("include") => self.parse_include(state, cp),
                Some("function") => self.parse_function(state, cp),
                Some("return") => self.parse_return(state, cp),
                Some("import") => self.parse_import(state, cp),
                Some("if") => self.parse_if(state, cp),
                Some("for") => self.parse_for(state, cp),
                Some("each") => self.parse_each(state, cp),
                Some("while") => self.parse_while(state, cp),
                _ => {
                    // Generic at-rule
                    while state.not_at_end() && !state.at(ScssTokenType::Semicolon) && !state.at(ScssTokenType::LeftBrace) {
                        state.advance();
                    }
                    if state.at(ScssTokenType::LeftBrace) {
                        self.parse_block(state)?;
                    }
                    else {
                        state.eat(ScssTokenType::Semicolon);
                    }
                    state.finish_at(cp, ScssElementType::RuleSet);
                    Ok(())
                }
            }
        }
        else {
            state.finish_at(cp, ScssElementType::Error);
            Ok(())
        }
    }

    fn parse_mixin<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(ScssTokenType::Identifier)?;
        if state.at(ScssTokenType::LeftParen) {
            self.parse_parameters(state)?;
        }
        self.parse_block(state)?;
        state.finish_at(cp, ScssElementType::MixinDeclaration);
        Ok(())
    }

    fn parse_include<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(ScssTokenType::Identifier)?;
        if state.at(ScssTokenType::LeftParen) {
            self.parse_arguments(state)?;
        }
        if state.at(ScssTokenType::LeftBrace) {
            self.parse_block(state)?;
        }
        else {
            state.eat(ScssTokenType::Semicolon);
        }
        state.finish_at(cp, ScssElementType::IncludeStatement);
        Ok(())
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        state.expect(ScssTokenType::Identifier)?;
        self.parse_parameters(state)?;
        self.parse_block(state)?;
        state.finish_at(cp, ScssElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(ScssTokenType::Semicolon) {
            state.advance();
        }
        state.eat(ScssTokenType::Semicolon);
        state.finish_at(cp, ScssElementType::ImportStatement);
        Ok(())
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(ScssTokenType::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        // Handle else if / else
        state.finish_at(cp, ScssElementType::IfStatement);
        Ok(())
    }

    fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(ScssTokenType::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ScssElementType::ForStatement);
        Ok(())
    }

    fn parse_each<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(ScssTokenType::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ScssElementType::EachStatement);
        Ok(())
    }

    fn parse_while<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        while state.not_at_end() && !state.at(ScssTokenType::LeftBrace) {
            state.advance();
        }
        self.parse_block(state)?;
        state.finish_at(cp, ScssElementType::WhileStatement);
        Ok(())
    }

    fn parse_parameters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.expect(ScssTokenType::LeftParen)?;
        while state.not_at_end() && !state.at(ScssTokenType::RightParen) {
            state.advance();
        }
        state.expect(ScssTokenType::RightParen)?;
        Ok(())
    }

    fn parse_arguments<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.expect(ScssTokenType::LeftParen)?;
        while state.not_at_end() && !state.at(ScssTokenType::RightParen) {
            state.advance();
        }
        state.expect(ScssTokenType::RightParen)?;
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ScssTokenType::LeftBrace)?;
        while state.not_at_end() && !state.at(ScssTokenType::RightBrace) {
            if state.at(ScssTokenType::Identifier) && self.is_declaration(state) {
                self.parse_declaration(state)?;
            }
            else {
                self.parse_node(state)?;
            }
        }
        state.expect(ScssTokenType::RightBrace)?;
        state.finish_at(cp, ScssElementType::Block);
        Ok(())
    }

    fn is_declaration<'a, S: Source + ?Sized>(&self, state: &State<'a, S>) -> bool {
        // Simple lookahead for colon
        state.peek_kind_at(1) == Some(ScssTokenType::Colon)
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        let prop_cp = state.checkpoint();
        state.expect(ScssTokenType::Identifier)?;
        state.finish_at(prop_cp, ScssElementType::Property);

        state.expect(ScssTokenType::Colon)?;

        let val_cp = state.checkpoint();
        while state.not_at_end() && !state.at(ScssTokenType::Semicolon) && !state.at(ScssTokenType::RightBrace) {
            state.advance();
        }
        state.finish_at(val_cp, ScssElementType::ValueNode);

        state.eat(ScssTokenType::Semicolon);
        state.finish_at(cp, ScssElementType::Declaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(ScssTokenType::Dollar)?;
        state.expect(ScssTokenType::Identifier)?;
        state.expect(ScssTokenType::Colon)?;
        while state.not_at_end() && !state.at(ScssTokenType::Semicolon) {
            state.advance();
        }
        state.eat(ScssTokenType::Semicolon);
        state.finish_at(cp, ScssElementType::VariableDeclaration);
        Ok(())
    }

    fn parse_return<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, cp: (usize, usize)) -> Result<(), OakError> {
        let val_cp = state.checkpoint();
        while state.not_at_end() && !state.at(ScssTokenType::Semicolon) {
            state.advance();
        }
        state.finish_at(val_cp, ScssElementType::ValueNode);
        state.eat(ScssTokenType::Semicolon);
        state.finish_at(cp, ScssElementType::ReturnStatement);
        Ok(())
    }

    fn parse_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        // Parse selector
        let sel_cp = state.checkpoint();
        while state.not_at_end() && !state.at(ScssTokenType::LeftBrace) {
            state.advance();
        }
        state.finish_at(sel_cp, ScssElementType::Selector);

        if state.at(ScssTokenType::LeftBrace) {
            self.parse_block(state)?;
        }

        state.finish_at(cp, ScssElementType::RuleSet);
        Ok(())
    }
}

impl<'config> Parser<ScssLanguage> for ScssParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ScssLanguage>) -> ParseOutput<'a, ScssLanguage> {
        let lexer = ScssLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_node(state)?;
            }

            Ok(state.finish_at(checkpoint, ScssElementType::SourceFile))
        })
    }
}
