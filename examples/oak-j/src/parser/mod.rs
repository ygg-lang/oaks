#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::JElementType;

use crate::{language::JLanguage, lexer::JTokenType};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, JLanguage, S>;

pub struct JParser<'config> {
    pub(crate) config: &'config JLanguage,
}

impl<'config> JParser<'config> {
    pub fn new(config: &'config JLanguage) -> Self {
        Self { config }
    }

    /// 解析 J 源代码的内部逻辑
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a oak_core::GreenNode<'a, JLanguage>, OakError> {
        let root_cp = state.checkpoint();
        let unit_cp = state.checkpoint();

        while state.not_at_end() {
            if !self.parse_sentence(state) {
                state.advance();
            }
        }

        state.finish_at(unit_cp, JElementType::CompilationUnit);
        let root = state.finish_at(root_cp, JElementType::Root);

        Ok(root)
    }

    /// 解析句子 (Sentence)
    fn parse_sentence<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let cp = state.checkpoint();

        // 简单的赋值或表达式解析
        let mut matched = false;
        if state.at(JTokenType::Identifier) && (state.peek_kind_at(1) == Some(JTokenType::IsGlobal) || state.peek_kind_at(1) == Some(JTokenType::IsLocal)) {
            matched = self.parse_assignment(state);
        }
        else {
            matched = self.parse_expression(state);
        }

        if matched {
            state.finish_at(cp, JElementType::Sentence);
        }
        matched
    }

    /// 解析赋值 (Assignment)
    fn parse_assignment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let cp = state.checkpoint();
        state.eat(JTokenType::Identifier);
        if state.at(JTokenType::IsGlobal) {
            state.eat(JTokenType::IsGlobal);
        }
        else {
            state.eat(JTokenType::IsLocal);
        }
        self.parse_expression(state);
        state.finish_at(cp, JElementType::Assignment);
        true
    }

    /// 解析表达式 (Expression)
    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let cp = state.checkpoint();
        let mut matched = false;

        while state.not_at_end() && !state.at(JTokenType::Newline) && !state.at(JTokenType::Eof) {
            if state.at(JTokenType::Identifier) {
                state.eat(JTokenType::Identifier);
                matched = true;
            }
            else if state.at(JTokenType::NumberLiteral) {
                state.eat(JTokenType::NumberLiteral);
                matched = true;
            }
            else if state.at(JTokenType::StringLiteral) {
                state.eat(JTokenType::StringLiteral);
                matched = true;
            }
            else if state.at(JTokenType::LeftParen) {
                let group_cp = state.checkpoint();
                state.eat(JTokenType::LeftParen);
                self.parse_expression(state);
                let _ = state.expect(JTokenType::RightParen);
                state.finish_at(group_cp, JElementType::Group);
                matched = true;
            }
            else {
                // 可能是操作符
                state.advance();
                matched = true;
            }
        }

        if matched {
            state.finish_at(cp, JElementType::Expression);
        }
        matched
    }
}

impl<'config> Parser<JLanguage> for JParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JLanguage>) -> ParseOutput<'a, JLanguage> {
        let lexer = crate::lexer::JLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.run(state))
    }
}
