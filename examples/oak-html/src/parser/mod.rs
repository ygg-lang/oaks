use crate::{kind::HtmlSyntaxKind, language::HtmlLanguage, lexer::HtmlLexer};
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, HtmlLanguage, S>;

pub struct HtmlParser<'config> {
    pub(crate) _config: Option<&'config HtmlLanguage>,
}

impl<'config> HtmlParser<'config> {
    pub fn new(config: &'config HtmlLanguage) -> Self {
        Self { _config: Some(config) }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, HtmlLanguage>, OakError> {
        let checkpoint = state.checkpoint();

        while state.not_at_end() {
            match state.peek_kind() {
                Some(HtmlSyntaxKind::TagOpen) => self.parse_tag(state)?,
                Some(HtmlSyntaxKind::Doctype) => {
                    state.bump();
                }
                Some(HtmlSyntaxKind::Comment) => {
                    state.bump();
                }
                _ => {
                    state.bump();
                }
            }
        }

        Ok(state.finish_at(checkpoint, HtmlSyntaxKind::Document.into()))
    }

    fn parse_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::HtmlSyntaxKind::*;
        let cp = state.checkpoint();
        state.expect(TagOpen).ok();
        state.expect(TagName).ok();

        while state.not_at_end() && !matches!(state.peek_kind(), Some(TagClose) | Some(TagSelfClose)) {
            if state.at(AttributeName) {
                let _attr_cp = state.checkpoint();
                state.bump(); // AttributeName
                if state.eat(Equal) {
                    state.eat(Quote);
                    state.eat(AttributeValue);
                    state.eat(Quote);
                }
                // 这里没有具体的 Attribute 节点类型，暂不 finish
            }
            else {
                state.advance();
            }
        }

        if state.eat(TagSelfClose) {
            // 自闭合标签
        }
        else if state.eat(TagClose) {
            // 这里应该递归解析子节点，直到遇到对应的结束标签
            // 简化处理：跳过直到结束标签
            while state.not_at_end() && !state.at(TagSlashOpen) {
                if state.at(TagOpen) {
                    self.parse_tag(state)?;
                }
                else {
                    state.advance();
                }
            }
            if state.eat(TagSlashOpen) {
                state.eat(TagName);
                state.expect(TagClose).ok();
            }
        }

        state.finish_at(cp, HtmlSyntaxKind::Element.into());
        Ok(())
    }
}

impl<'config> Parser<HtmlLanguage> for HtmlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<HtmlLanguage>) -> ParseOutput<'a, HtmlLanguage> {
        let default_config = HtmlLanguage::default();
        let config = self._config.unwrap_or(&default_config);
        let lexer = HtmlLexer::new(config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
