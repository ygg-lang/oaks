use crate::{
    kind::{VueLanguage, VueSyntaxKind},
    lexer::VueLexer,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    lexer::Lexer,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::{Source, SourceText},
};

pub(crate) type State<'a, S> = ParserState<'a, VueLanguage, S>;

pub struct VueParser<'config> {
    _config: Option<&'config VueLanguage>,
}

impl<'config> VueParser<'config> {
    pub fn new() -> Self {
        Self { _config: None }
    }

    pub fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        PrattParser::parse(state, 0, self)
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, VueLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.at(VueSyntaxKind::TemplateStart) {
                self.parse_special_block(state, VueSyntaxKind::TemplateStart, "template", true);
            }
            else if state.at(VueSyntaxKind::ScriptStart) {
                self.parse_special_block(state, VueSyntaxKind::ScriptStart, "script", false);
            }
            else if state.at(VueSyntaxKind::StyleStart) {
                self.parse_special_block(state, VueSyntaxKind::StyleStart, "style", false);
            }
            else if state.at(VueSyntaxKind::Lt) {
                self.parse_element(state);
            }
            else {
                state.bump();
            }
        }
        Ok(state.finish_at(cp, VueSyntaxKind::Root.into()))
    }

    fn parse_special_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, start_kind: VueSyntaxKind, tag_name: &str, recursive: bool) {
        let cp = state.checkpoint();
        state.expect(start_kind).ok();

        while state.not_at_end() && !state.at(VueSyntaxKind::Gt) {
            self.parse_attribute(state);
        }

        state.expect(VueSyntaxKind::Gt).ok();

        if tag_name == "script" {
            self.parse_program(state);
        }
        else if recursive {
            while state.not_at_end() && !state.at(VueSyntaxKind::LtSlash) {
                self.parse_node(state);
            }
        }
        else {
            // Raw content
            while state.not_at_end() && !state.at(VueSyntaxKind::LtSlash) {
                state.bump();
            }
        }

        if state.at(VueSyntaxKind::LtSlash) {
            state.expect(VueSyntaxKind::LtSlash).ok();
            state.expect(VueSyntaxKind::Identifier).ok();
            state.expect(VueSyntaxKind::Gt).ok();
        }

        state.finish_at(cp, VueSyntaxKind::Element.into());
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if state.at(VueSyntaxKind::Lt) {
            self.parse_element(state);
        }
        else if state.at(VueSyntaxKind::InterpolationStart) {
            self.parse_interpolation(state);
        }
        else if state.at(VueSyntaxKind::Comment) {
            let cp = state.checkpoint();
            state.bump();
            state.finish_at(cp, VueSyntaxKind::CommentNode.into());
        }
        else {
            let cp = state.checkpoint();
            while state.not_at_end() && !state.at(VueSyntaxKind::Lt) && !state.at(VueSyntaxKind::InterpolationStart) {
                state.bump();
            }
            state.finish_at(cp, VueSyntaxKind::TextNode.into());
        }
    }

    fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VueSyntaxKind::Lt).ok();
        state.expect(VueSyntaxKind::Identifier).ok();

        while state.not_at_end() && !state.at(VueSyntaxKind::Gt) && !state.at(VueSyntaxKind::SelfClosingEnd) {
            self.parse_attribute(state);
        }

        if state.eat(VueSyntaxKind::SelfClosingEnd) {
            state.finish_at(cp, VueSyntaxKind::Element.into());
            return;
        }

        state.expect(VueSyntaxKind::Gt).ok();

        while state.not_at_end() && !state.at(VueSyntaxKind::LtSlash) {
            self.parse_node(state);
        }

        if state.at(VueSyntaxKind::LtSlash) {
            state.expect(VueSyntaxKind::LtSlash).ok();
            state.expect(VueSyntaxKind::Identifier).ok();
            state.expect(VueSyntaxKind::Gt).ok();
        }

        state.finish_at(cp, VueSyntaxKind::Element.into());
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        let mut is_directive = false;
        let mut directive_name = String::new();

        if state.at(VueSyntaxKind::At) {
            state.bump();
            is_directive = true;
            directive_name = "v-on".to_string();
        }
        else if state.at(VueSyntaxKind::Colon) {
            state.bump();
            is_directive = true;
            directive_name = "v-bind".to_string();
        }
        else if state.at(VueSyntaxKind::Hash) {
            state.bump();
            is_directive = true;
            directive_name = "v-slot".to_string();
        }

        let name_cp = state.checkpoint();
        if state.at(VueSyntaxKind::Identifier) {
            let text = state.peek_text().map(|c| c.to_string()).unwrap_or_default();
            if text.starts_with("v-") {
                is_directive = true;
                directive_name = text;
            }
            state.bump();
        }

        if state.at(VueSyntaxKind::Colon) {
            state.bump();
            state.expect(VueSyntaxKind::Identifier).ok();
        }

        while state.at(VueSyntaxKind::Dot) {
            let mod_cp = state.checkpoint();
            state.bump();
            state.expect(VueSyntaxKind::Identifier).ok();
            state.finish_at(mod_cp, VueSyntaxKind::Modifier.into());
        }
        state.finish_at(name_cp, VueSyntaxKind::AttributeName.into());

        if state.eat(VueSyntaxKind::Eq) {
            let val_cp = state.checkpoint();
            if is_directive {
                self.parse_directive_value(state, &directive_name);
            }
            else if state.at(VueSyntaxKind::StringLiteral) {
                state.bump();
            }
            else {
                self.parse_expression(state);
            }
            state.finish_at(val_cp, VueSyntaxKind::AttributeValue.into());
        }

        let kind = if is_directive { VueSyntaxKind::Directive } else { VueSyntaxKind::Attribute };
        state.finish_at(cp, kind.into());
    }

    fn parse_directive_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, directive_name: &str) {
        if state.at(VueSyntaxKind::StringLiteral) {
            if let Some(text) = state.peek_text() {
                if text.len() >= 2 {
                    let inner_text = &text[1..text.len() - 1];
                    let inner_source = SourceText::new(inner_text.to_string());

                    let lexer = VueLexer::new();
                    let mut temp_cache = oak_core::parser::ParseSession::default();
                    let _lex_output = lexer.lex(&inner_source, &[], &mut temp_cache);

                    // Use the standardized ParserState constructor for nested parsing
                    let mut inner_state = state.nested();

                    if directive_name == "v-for" {
                        self.parse_v_for_expression(&mut inner_state);
                    }
                    else {
                        self.parse_expression(&mut inner_state);
                    }
                }
            }
            state.bump();
        }
    }

    fn parse_v_for_expression<'a, S: Source + ?Sized>(&self, _state: &mut State<'a, S>) {
        // Implementation for v-for expression parsing
    }

    fn parse_interpolation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VueSyntaxKind::InterpolationStart).ok();
        self.parse_expression(state);
        state.expect(VueSyntaxKind::InterpolationEnd).ok();
        state.finish_at(cp, VueSyntaxKind::Interpolation.into());
    }

    fn parse_program<'a, S: Source + ?Sized>(&self, _state: &mut State<'a, S>) {
        // Implementation for script content parsing
    }
}

impl<'config> Pratt<VueLanguage> for VueParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        use VueSyntaxKind::*;
        let cp = state.checkpoint();
        if state.at(LeftParen) {
            state.expect(LeftParen).ok();
            PrattParser::parse(state, 0, self);
            state.expect(RightParen).ok();
            state.finish_at(cp, Expression.into())
        }
        else if state.at(Identifier) {
            state.expect(Identifier).ok();
            state.finish_at(cp, Identifier.into())
        }
        else if state.at(StringLiteral) {
            state.expect(StringLiteral).ok();
            state.finish_at(cp, Literal.into())
        }
        else if state.at(NumberLiteral) {
            state.expect(NumberLiteral).ok();
            state.finish_at(cp, Literal.into())
        }
        else {
            state.bump();
            state.finish_at(cp, Error.into())
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        use VueSyntaxKind::*;
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Bang | Minus => unary(state, kind.into(), 90, UnaryExpr.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, VueLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, VueLanguage>> {
        use VueSyntaxKind::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Dot | LeftBracket | LeftParen => (100, Associativity::Left),
            Star | Slash => (80, Associativity::Left),
            Plus | Minus => (70, Associativity::Left),
            Lt | Gt => (60, Associativity::Left),
            EqEq => (50, Associativity::Left),
            And => (40, Associativity::Left),
            Or => (30, Associativity::Left),
            Eq => (10, Associativity::Right),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, MemberExpr.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, MemberExpr.into()))
            }
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                if !state.at(RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(Comma) {
                            break;
                        }
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CallExpr.into()))
            }
            _ => Some(binary(state, left, kind.into(), prec, assoc, BinaryExpr.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<VueLanguage> for VueParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VueLanguage>) -> ParseOutput<'a, VueLanguage> {
        let lexer = VueLexer::new();
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
