use crate::parser::{State, TypeScriptParser};
use oak_core::{GreenNode, parser::pratt::PrattParser, source::Source};

impl<'config> TypeScriptParser<'config> {
    pub(crate) fn parse_jsx_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, crate::language::TypeScriptLanguage> {
        use crate::lexer::token_type::TypeScriptTokenType::*;
        let cp = state.checkpoint();

        if self.eat(state, Less) {
            if self.eat(state, Greater) {
                // Fragment
                self.parse_jsx_children(state);
                self.expect(state, Less).ok();
                self.expect(state, Slash).ok();
                self.expect(state, Greater).ok();
                state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::JsxFragment.into())
            }
            else {
                let is_self_closing = self.parse_jsx_opening_element_or_self_closing(state);
                if !is_self_closing {
                    self.parse_jsx_children(state);
                    if self.eat(state, Less) {
                        self.eat(state, Slash);
                        self.parse_jsx_name(state);
                        self.expect(state, Greater).ok();
                    }
                    state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::JsxElement.into())
                }
                else {
                    state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::JsxSelfClosingElement.into())
                }
            }
        }
        else {
            state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::Error.into())
        }
    }

    pub(crate) fn parse_jsx_opening_element_or_self_closing<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        use crate::lexer::token_type::TypeScriptTokenType::*;
        self.parse_jsx_name(state);
        while state.not_at_end() && !self.at(state, Greater) && !self.at(state, Slash) {
            self.parse_jsx_attribute(state)
        }
        if self.eat(state, Slash) {
            self.expect(state, Greater).ok();
            true
        }
        else {
            self.expect(state, Greater).ok();
            false
        }
    }

    pub(crate) fn parse_jsx_name<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::token_type::TypeScriptTokenType::*;
        self.expect(state, IdentifierName).ok();
        while self.eat(state, Dot) {
            self.expect(state, IdentifierName).ok();
        }
    }

    pub(crate) fn parse_jsx_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::token_type::TypeScriptTokenType::*;
        let cp = state.checkpoint();
        if self.eat(state, LeftBrace) {
            self.expect(state, DotDotDot).ok();
            PrattParser::parse(state, 0, self);
            self.expect(state, RightBrace).ok();
            state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::JsxSpreadAttribute);
        }
        else {
            self.expect(state, IdentifierName).ok();
            if self.eat(state, Equal) {
                if self.at(state, StringLiteral) {
                    state.bump();
                }
                else if self.eat(state, LeftBrace) {
                    PrattParser::parse(state, 0, self);
                    self.expect(state, RightBrace).ok();
                }
            }
            state.finish_at(cp, crate::parser::element_type::TypeScriptElementType::JsxAttribute);
        }
    }

    pub(crate) fn parse_jsx_children<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::token_type::TypeScriptTokenType::*;
        while state.not_at_end() && !self.at(state, Less) {
            // Very simplified JSX text/child parsing
            state.bump();
        }
    }
}
