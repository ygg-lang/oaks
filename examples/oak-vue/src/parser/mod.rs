use crate::{
    kind::{VueLanguage, VueSyntaxKind},
    lexer::VueLexer,
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    lexer::Lexer,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, unary},
    },
    source::{Source, SourceText},
};

pub(crate) type State<'a, S> = ParserState<'a, VueLanguage, S>;

pub struct VueParser<'config> {
    _config: &'config VueLanguage,
}

impl<'config> VueParser<'config> {
    pub fn new(config: &'config VueLanguage) -> Self {
        Self { _config: config }
    }

    pub fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        PrattParser::parse(state, 0, self)
    }

    pub fn parse_expression_only<'a, S: Source + ?Sized>(&self, source: &'a S, session: &'a mut oak_core::parser::ParseSession<VueLanguage>) -> oak_core::parser::ParseOutput<'a, VueLanguage> {
        let lexer = VueLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, source, &[], session, |state| {
            let cp = state.checkpoint();
            self.parse_expression(state);
            Ok(state.finish_at(cp, VueSyntaxKind::Root.into()))
        })
    }

    pub fn parse_v_for_only<'a, S: Source + ?Sized>(&self, source: &'a S, session: &'a mut oak_core::parser::ParseSession<VueLanguage>) -> oak_core::parser::ParseOutput<'a, VueLanguage> {
        let lexer = VueLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, source, &[], session, |state| {
            let cp = state.checkpoint();
            self.parse_v_for_expression(state);
            Ok(state.finish_at(cp, VueSyntaxKind::Root.into()))
        })
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
            else {
                self.parse_node(state);
            }
        }
        Ok(state.finish_at(cp, VueSyntaxKind::Root.into()))
    }

    fn parse_special_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, start_kind: VueSyntaxKind, tag_name: &str, recursive: bool) {
        let cp = state.checkpoint();
        let tag_cp = state.checkpoint();
        state.expect(start_kind).ok();

        while state.not_at_end() && !state.at(VueSyntaxKind::Gt) {
            if state.at(VueSyntaxKind::Whitespace) {
                state.bump();
                continue;
            }
            self.parse_attribute(state);
        }

        state.expect(VueSyntaxKind::Gt).ok();
        let tag_node = state.finish_at(tag_cp, VueSyntaxKind::Tag.into());
        state.sink.restore(tag_cp.1);
        state.push_child(tag_node);

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
            let close_cp = state.checkpoint();
            state.expect(VueSyntaxKind::LtSlash).ok();
            state.expect(VueSyntaxKind::Identifier).ok();
            state.expect(VueSyntaxKind::Gt).ok();
            let close_node = state.finish_at(close_cp, VueSyntaxKind::CloseTag.into());
            state.sink.restore(close_cp.1);
            state.push_child(close_node);
        }

        let node = state.finish_at(cp, VueSyntaxKind::Element.into());
        state.sink.restore(cp.1);
        state.push_child(node);
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        if state.at(VueSyntaxKind::Lt) || state.at(VueSyntaxKind::ScriptStart) || state.at(VueSyntaxKind::StyleStart) || state.at(VueSyntaxKind::TemplateStart) || state.at(VueSyntaxKind::DocTypeStart) {
            self.parse_element(state);
        }
        else if state.at(VueSyntaxKind::InterpolationStart) {
            self.parse_interpolation(state);
        }
        else if state.at(VueSyntaxKind::Comment) {
            let cp = state.checkpoint();
            state.bump();
            let node = state.finish_at(cp, VueSyntaxKind::CommentNode.into());
            state.sink.restore(cp.1);
            state.push_child(node);
        }
        else if state.at(VueSyntaxKind::LtSlash) {
            // This is a closing tag at a position where we expect a node.
            // We should NOT consume it as text. The caller (parse_element) will handle it.
            return;
        }
        else {
            let cp = state.checkpoint();
            while state.not_at_end()
                && !state.at(VueSyntaxKind::Lt)
                && !state.at(VueSyntaxKind::LtSlash)
                && !state.at(VueSyntaxKind::ScriptStart)
                && !state.at(VueSyntaxKind::StyleStart)
                && !state.at(VueSyntaxKind::TemplateStart)
                && !state.at(VueSyntaxKind::DocTypeStart)
                && !state.at(VueSyntaxKind::InterpolationStart)
            {
                state.bump();
            }
            let node = state.finish_at(cp, VueSyntaxKind::TextNode.into());
            state.sink.restore(cp.1);
            state.push_child(node);
        }
    }

    fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        let tag_cp = state.checkpoint();

        let mut tag_name = String::new();
        let mut start_kind = VueSyntaxKind::Lt;

        if state.at(VueSyntaxKind::Lt) {
            state.expect(VueSyntaxKind::Lt).ok();
            if state.at(VueSyntaxKind::Identifier) {
                tag_name = state.peek_text().unwrap_or_default().to_lowercase();
                state.bump();
            }
        }
        else if state.at(VueSyntaxKind::ScriptStart) {
            start_kind = VueSyntaxKind::ScriptStart;
            state.bump();
            tag_name = "script".to_string();
        }
        else if state.at(VueSyntaxKind::StyleStart) {
            start_kind = VueSyntaxKind::StyleStart;
            state.bump();
            tag_name = "style".to_string();
        }
        else if state.at(VueSyntaxKind::TemplateStart) {
            start_kind = VueSyntaxKind::TemplateStart;
            state.bump();
            tag_name = "template".to_string();
        }
        else if state.at(VueSyntaxKind::DocTypeStart) {
            // Special handling for DOCTYPE
            state.bump();
            while state.not_at_end() && !state.at(VueSyntaxKind::Gt) {
                state.bump();
            }
            state.expect(VueSyntaxKind::Gt).ok();
            let node = state.finish_at(cp, VueSyntaxKind::Element.into());
            state.sink.restore(cp.1);
            state.push_child(node);
            return;
        }

        // Handle special blocks immediately
        if tag_name == "script" || tag_name == "style" || tag_name == "template" {
            // Backtrack to use parse_special_block
            state.sink.restore(cp.1); // Restore the sink to before the start token
            self.parse_special_block(state, start_kind, &tag_name, tag_name == "template");
            return;
        }

        while state.not_at_end() && !state.at(VueSyntaxKind::Gt) && !state.at(VueSyntaxKind::SelfClosingEnd) {
            if state.at(VueSyntaxKind::Whitespace) {
                state.bump();
                continue;
            }
            self.parse_attribute(state);
        }

        if state.eat(VueSyntaxKind::SelfClosingEnd) {
            let tag_node = state.finish_at(tag_cp, VueSyntaxKind::Tag.into());
            state.sink.restore(tag_cp.1);
            state.push_child(tag_node);

            let node = state.finish_at(cp, VueSyntaxKind::Element.into());
            state.sink.restore(cp.1);
            state.push_child(node);
            return;
        }

        state.expect(VueSyntaxKind::Gt).ok();
        let tag_node = state.finish_at(tag_cp, VueSyntaxKind::Tag.into());
        state.sink.restore(tag_cp.1);
        state.push_child(tag_node);

        // Void elements should not have children
        let void_elements = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr", "!doctype"];
        if void_elements.contains(&tag_name.as_str()) {
            let node = state.finish_at(cp, VueSyntaxKind::Element.into());
            state.sink.restore(cp.1);
            state.push_child(node);
            return;
        }

        while state.not_at_end() && !state.at(VueSyntaxKind::LtSlash) {
            self.parse_node(state);
        }

        if state.at(VueSyntaxKind::LtSlash) {
            let close_cp = state.checkpoint();
            state.expect(VueSyntaxKind::LtSlash).ok();
            state.expect(VueSyntaxKind::Identifier).ok();
            state.expect(VueSyntaxKind::Gt).ok();
            let close_node = state.finish_at(close_cp, VueSyntaxKind::CloseTag.into());
            state.sink.restore(close_cp.1);
            state.push_child(close_node);
        }

        let node = state.finish_at(cp, VueSyntaxKind::Element.into());
        state.sink.restore(cp.1);
        state.push_child(node);
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

                    let lexer = VueLexer::new(self._config);
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

    pub fn parse_v_for_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        self.skip_whitespace(state);

        // Parse the left side (pattern)
        let pattern_cp = state.checkpoint();
        if state.at(VueSyntaxKind::LeftParen) {
            state.bump();
            self.skip_whitespace(state);
            state.expect(VueSyntaxKind::Identifier).ok();
            self.skip_whitespace(state);
            if state.eat(VueSyntaxKind::Comma) {
                self.skip_whitespace(state);
                state.expect(VueSyntaxKind::Identifier).ok();
                self.skip_whitespace(state);
            }
            state.expect(VueSyntaxKind::RightParen).ok();
        }
        else {
            state.expect(VueSyntaxKind::Identifier).ok();
        }
        state.finish_at(pattern_cp, VueSyntaxKind::Pattern.into());

        self.skip_whitespace(state);

        // Parse "in" or "of"
        let kind = if state.at(VueSyntaxKind::In) {
            state.bump();
            VueSyntaxKind::ForInExpr
        }
        else if state.at(VueSyntaxKind::Of) {
            state.bump();
            VueSyntaxKind::ForOfExpr
        }
        else {
            // Fallback if neither "in" nor "of" is found
            state.finish_at(cp, VueSyntaxKind::Error.into());
            return;
        };

        self.skip_whitespace(state);

        // Parse the right side (expression)
        let expr = self.parse_expression(state);
        state.push_child(expr);

        state.finish_at(cp, kind.into());
    }

    fn parse_interpolation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let cp = state.checkpoint();
        state.expect(VueSyntaxKind::InterpolationStart).ok();
        self.skip_whitespace(state);
        let expr = self.parse_expression(state);
        state.push_child(expr);
        self.skip_whitespace(state);
        state.expect(VueSyntaxKind::InterpolationEnd).ok();
        state.finish_at(cp, VueSyntaxKind::Interpolation.into());
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(VueSyntaxKind::Whitespace) {
            state.bump();
        }
    }

    fn parse_program<'a, S: Source + ?Sized>(&self, _state: &mut State<'a, S>) {
        // Implementation for script content parsing
    }
}

impl<'config> Pratt<VueLanguage> for VueParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        use VueSyntaxKind::*;
        let cp = state.checkpoint();
        self.skip_whitespace(state);
        let node = if state.at(LeftParen) {
            state.expect(LeftParen).ok();
            let inner = PrattParser::parse(state, 0, self);
            state.push_child(inner);
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
        };

        state.sink.restore(cp.1);
        node
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, VueLanguage> {
        use VueSyntaxKind::*;
        let start_cp = state.checkpoint();
        self.skip_whitespace(state);
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => {
                state.restore(start_cp);
                return self.primary(state);
            }
        };

        match kind {
            Bang | Minus => {
                state.restore(start_cp);
                let cp = state.checkpoint();
                self.skip_whitespace(state);
                let node = unary(state, kind.into(), 90, UnaryExpr.into(), |s, p| PrattParser::parse(s, p, self));
                state.sink.restore(cp.1);
                node
            }
            _ => {
                state.restore(start_cp);
                self.primary(state)
            }
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, VueLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, VueLanguage>> {
        use VueSyntaxKind::*;
        let start_cp = state.checkpoint();
        self.skip_whitespace(state);
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => {
                state.restore(start_cp);
                return None;
            }
        };

        let (prec, assoc) = match kind {
            Dot | LeftBracket | LeftParen => (100, Associativity::Left),
            Star | Slash => (80, Associativity::Left),
            Plus | Minus => (70, Associativity::Left),
            Lt | Gt => (60, Associativity::Left),
            EqEq => (50, Associativity::Left),
            And => (40, Associativity::Left),
            Or => (30, Associativity::Left),
            Eq => (10, Associativity::Right),
            _ => {
                state.restore(start_cp);
                return None;
            }
        };

        if prec < min_precedence {
            state.restore(start_cp);
            return None;
        }

        // Re-position left and whitespace:
        // 1. Restore to before whitespace
        state.restore(start_cp);
        // 2. Take checkpoint for the new infix node
        let op_cp = state.checkpoint();
        // 3. Push left as the first child
        state.push_child(left);
        // 4. Re-skip whitespace so it becomes a child after left
        self.skip_whitespace(state);

        let node = match kind {
            Dot => {
                state.expect(Dot).ok();
                self.skip_whitespace(state);
                state.expect(Identifier).ok();
                state.finish_at(op_cp, MemberExpr.into())
            }
            LeftBracket => {
                state.expect(LeftBracket).ok();
                self.skip_whitespace(state);
                let inner = PrattParser::parse(state, 0, self);
                state.push_child(inner);
                self.skip_whitespace(state);
                state.expect(RightBracket).ok();
                state.finish_at(op_cp, MemberExpr.into())
            }
            LeftParen => {
                state.expect(LeftParen).ok();
                self.skip_whitespace(state);
                if !state.at(RightParen) {
                    loop {
                        let arg = PrattParser::parse(state, 0, self);
                        state.push_child(arg);
                        self.skip_whitespace(state);
                        if state.expect(Comma).is_ok() {
                            self.skip_whitespace(state);
                        }
                        else {
                            break;
                        }
                    }
                }
                state.expect(RightParen).ok();
                state.finish_at(op_cp, CallExpr.into())
            }
            _ => {
                state.expect(kind).ok();
                let next_prec = match assoc {
                    Associativity::Left => prec + 1,
                    Associativity::Right => prec,
                    Associativity::None => prec + 1,
                };
                let right = {
                    self.skip_whitespace(state);
                    PrattParser::parse(state, next_prec, self)
                };
                state.push_child(right);
                state.finish_at(op_cp, BinaryExpr.into())
            }
        };

        state.sink.restore(start_cp.1);
        Some(node)
    }
}

impl<'config> Parser<VueLanguage> for VueParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VueLanguage>) -> ParseOutput<'a, VueLanguage> {
        let lexer = VueLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
