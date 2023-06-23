use oak_core::{
    OakError, Source, TextEdit, TokenType,
    parser::{ParseCache, Parser, ParserState},
};

pub mod element_type;
use crate::{
    language::VonLanguage,
    lexer::{VonLexer, VonTokenType},
};
pub use element_type::VonElementType;

pub(crate) type State<'a, S> = ParserState<'a, VonLanguage, S>;

pub struct VonParser<'config> {
    pub(crate) config: &'config VonLanguage,
}

impl<'config> VonParser<'config> {
    pub fn new(config: &'config VonLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        state.incremental_node(VonElementType::Value, |state| {
            let token = if let Some(t) = state.current() {
                if t.kind == VonTokenType::Eof {
                    return Err(state.unexpected_eof());
                }
                t
            }
            else {
                return Err(state.unexpected_eof());
            };

            match token.kind {
                VonTokenType::LeftBrace => self.parse_object(state),
                VonTokenType::LeftBracket => self.parse_array(state),
                VonTokenType::Identifier => self.parse_enum(state),
                VonTokenType::StringLiteral | VonTokenType::NumberLiteral | VonTokenType::BoolLiteral | VonTokenType::NullLiteral => {
                    state.bump();
                    Ok(())
                }
                _ => {
                    state.record_unexpected_token(format!("{:?}", token.kind));
                    Err(state.errors.last().unwrap().clone())
                }
            }
        })
    }

    fn parse_enum<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VonElementType::Enum, |state| {
            if !state.eat(VonTokenType::Identifier) {
                state.record_expected("variant");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }

            self.skip_trivia(state);
            if state.at(VonTokenType::LeftBrace) || state.at(VonTokenType::LeftBracket) {
                self.parse_value(state)?;
            }
            Ok(())
        })
    }

    pub(crate) fn parse_object_inner<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let mut first = true;
        while state.not_at_end() {
            self.skip_trivia(state);
            if state.at(VonTokenType::RightBrace) {
                break;
            }

            if state.at(VonTokenType::Eof) {
                break;
            }

            if !first {
                // 逗号是可选的，但如果存在则吃掉它
                if state.eat(VonTokenType::Comma) {
                    self.skip_trivia(state);
                    if state.at(VonTokenType::RightBrace) || state.at(VonTokenType::Eof) {
                        break;
                    }
                }
            }
            first = false;

            // 检查是否真的是一个 ObjectEntry (以 Identifier 或 StringLiteral 开始)
            if !state.at(VonTokenType::Identifier) && !state.at(VonTokenType::StringLiteral) {
                break;
            }

            self.parse_object_entry(state)?;
            self.skip_trivia(state);
        }
        Ok(())
    }

    fn parse_object<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VonElementType::Object, |state| {
            if !state.eat(VonTokenType::LeftBrace) {
                state.record_expected("{");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }

            self.parse_object_inner(state)?;

            if !state.eat(VonTokenType::RightBrace) {
                if state.at(VonTokenType::Eof) || !state.not_at_end() {
                    return Err(state.unexpected_eof());
                }
                state.record_expected("}");
            }
            Ok(())
        })
    }

    fn parse_object_entry<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VonElementType::ObjectEntry, |state| {
            if state.at(VonTokenType::Identifier) || state.at(VonTokenType::StringLiteral) {
                state.bump();
            }
            else {
                state.record_expected("key");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }

            self.skip_trivia(state);
            if state.at(VonTokenType::Eq) {
                state.bump();
            }
            else {
                state.record_expected("=");
            }
            self.skip_trivia(state);
            // 确保在尝试解析值之前没有到达 EOF 或 }
            if state.at(VonTokenType::RightBrace) || state.at(VonTokenType::Eof) {
                state.record_expected("value");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }
            self.parse_value(state)?;
            Ok(())
        })
    }

    fn parse_array<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(VonElementType::Array, |state| {
            if !state.eat(VonTokenType::LeftBracket) {
                state.record_expected("[");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }

            let mut first = true;
            while state.not_at_end() {
                self.skip_trivia(state);
                if state.at(VonTokenType::RightBracket) {
                    break;
                }

                if state.at(VonTokenType::Eof) {
                    return Err(state.unexpected_eof());
                }

                if !first {
                    // 逗号是可选的
                    if state.eat(VonTokenType::Comma) {
                        self.skip_trivia(state);
                        if state.at(VonTokenType::RightBracket) {
                            break;
                        }
                    }
                }
                first = false;

                state.incremental_node(VonElementType::ArrayElement, |state| self.parse_value(state))?;
                self.skip_trivia(state);
            }

            if !state.eat(VonTokenType::RightBracket) {
                if state.at(VonTokenType::Eof) || !state.not_at_end() {
                    return Err(state.unexpected_eof());
                }
                state.record_expected("]");
            }
            Ok(())
        })
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(token) = state.current() {
            if token.kind.is_ignored() {
                state.bump();
            }
            else {
                break;
            }
        }
    }
}

impl<'config> Parser<VonLanguage> for VonParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VonLanguage>) -> oak_core::ParseOutput<'a, VonLanguage> {
        let lexer = VonLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            self.skip_trivia(state);

            // 检查是否是隐式根对象（不以 { 或 [ 开始）
            if state.at(VonTokenType::LeftBrace) || state.at(VonTokenType::LeftBracket) {
                let _ = self.parse_value(state);
            }
            else if state.at(VonTokenType::Identifier) {
                // 如果以标识符开始，可能是隐式对象或 Enum
                // 我们先尝试解析为隐式对象，如果失败则回退（这里简单处理为隐式对象）
                let _ = state.incremental_node(VonElementType::Object, |state| self.parse_object_inner(state));
            }
            else if state.at(VonTokenType::Eof) {
                // 空文件
            }
            else {
                // 其他情况，尝试作为普通值解析
                let _ = self.parse_value(state);
            }

            while state.not_at_end() {
                if let Some(token) = state.current() {
                    if token.kind.is_ignored() {
                        state.bump();
                        continue;
                    }
                }
                break;
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::VonElementType::Root))
        })
    }
}
