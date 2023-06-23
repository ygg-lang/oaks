use crate::{
    language::ElixirLanguage,
    lexer::token_type::ElixirTokenType,
    parser::{ElixirParser, State, element_type::ElixirElementType},
};
use oak_core::{GreenNode, OakError, parser::PrattParser, source::Source};

impl<'config> ElixirParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, ElixirLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(ElixirTokenType::Defmodule) => {
                    self.parse_module(state);
                }
                Some(ElixirTokenType::Def) | Some(ElixirTokenType::Defp) => {
                    self.parse_function(state);
                }
                Some(_) => {
                    PrattParser::parse(state, 0, self);
                }
                None => break,
            }
            state.eat(ElixirTokenType::Semicolon);
            state.eat(ElixirTokenType::Newline);
        }

        Ok(state.finish_at(checkpoint, ElixirElementType::Root))
    }

    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let cp = state.checkpoint();
        state.expect(ElixirTokenType::Defmodule).ok();
        PrattParser::parse(state, 0, self); // Module name
        state.expect(ElixirTokenType::Do).ok();
        while state.not_at_end() && !state.at(ElixirTokenType::End) {
            match state.peek_kind() {
                Some(ElixirTokenType::Def) | Some(ElixirTokenType::Defp) => {
                    self.parse_function(state);
                }
                Some(_) => {
                    PrattParser::parse(state, 0, self);
                }
                None => break,
            }
            state.eat(ElixirTokenType::Semicolon);
            state.eat(ElixirTokenType::Newline);
        }
        state.expect(ElixirTokenType::End).ok();
        state.finish_at(cp, ElixirElementType::ModuleDefinition)
    }

    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, ElixirLanguage> {
        let cp = state.checkpoint();
        state.bump(); // def or defp
        PrattParser::parse(state, 0, self); // Function name and params
        if state.eat(ElixirTokenType::Do) {
            while state.not_at_end() && !state.at(ElixirTokenType::End) {
                PrattParser::parse(state, 0, self);
                state.eat(ElixirTokenType::Semicolon);
                state.eat(ElixirTokenType::Newline);
            }
            state.expect(ElixirTokenType::End).ok();
        }
        state.finish_at(cp, ElixirElementType::FunctionDefinition)
    }
}
