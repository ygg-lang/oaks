pub mod element_type;

use crate::{language::JsonLanguage, lexer::token_type::JsonTokenType, parser::element_type::JsonElementType};
use oak_core::{OakError, Parser, ParserState, Source, TextEdit, TokenType};

pub(crate) type State<'a, S> = ParserState<'a, JsonLanguage, S>;

/// JSON 语言解析器
pub struct JsonParser<'config> {
    /// 语言配置
    pub(crate) config: &'config JsonLanguage,
}

impl<'config> JsonParser<'config> {
    pub fn new(config: &'config JsonLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_value<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        let token = if let Some(t) = state.current() {
            if t.kind == JsonTokenType::Eof {
                return Err(state.unexpected_eof());
            }
            t
        }
        else {
            return Err(state.unexpected_eof());
        };

        let kind = match token.kind {
            JsonTokenType::LeftBrace => JsonTokenType::Object,
            JsonTokenType::LeftBracket => JsonTokenType::Array,
            JsonTokenType::StringLiteral => JsonTokenType::String,
            JsonTokenType::NumberLiteral => JsonTokenType::Number,
            JsonTokenType::BooleanLiteral => JsonTokenType::Boolean,
            JsonTokenType::NullLiteral => JsonTokenType::Null,
            _ => {
                if self.config.bare_keys && token.kind == JsonTokenType::BareKey {
                    state.record_unexpected_token(format!("{:?}", token.kind));
                    return Err(state.errors.last().unwrap().clone());
                }
                state.record_unexpected_token(format!("{:?}", token.kind));
                return Err(state.errors.last().unwrap().clone());
            }
        };

        state.incremental_node(kind.into(), |state| match kind {
            JsonTokenType::Object => self.parse_object(state),
            JsonTokenType::Array => self.parse_array(state),
            _ => {
                state.bump();
                Ok(())
            }
        })
    }

    fn parse_object<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if !state.eat(JsonTokenType::LeftBrace) {
            state.record_expected("{");
            return Err(state.errors.last().cloned().expect("Error should have been recorded"));
        }

        let mut first = true;
        while state.not_at_end() {
            self.skip_trivia(state);
            if state.at(JsonTokenType::RightBrace) {
                break;
            }

            if state.at(JsonTokenType::Eof) {
                return Err(state.unexpected_eof());
            }

            if !first {
                if !state.eat(JsonTokenType::Comma) {
                    state.record_expected(",");
                    // Don't break here to try recover
                    break;
                }

                self.skip_trivia(state);
                if state.at(JsonTokenType::RightBrace) {
                    if !self.config.trailing_comma {
                        state.record_trailing_comma_not_allowed();
                        return Err(state.errors.last().cloned().expect("Error should have been recorded"));
                    }
                    break;
                }
            }
            first = false;

            self.parse_object_entry(state)?;
            self.skip_trivia(state);
        }

        if !state.eat(JsonTokenType::RightBrace) {
            // Check if we are at EOF, if so return unexpected EOF
            if state.at(JsonTokenType::Eof) || !state.not_at_end() {
                return Err(state.unexpected_eof());
            }
            state.record_expected("}");
            return Err(state.errors.last().cloned().expect("Error should have been recorded"));
        }
        Ok(())
    }

    fn parse_object_entry<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        state.incremental_node(JsonElementType::ObjectEntry, |state| {
            if state.at(JsonTokenType::StringLiteral) || (self.config.bare_keys && state.at(JsonTokenType::BareKey)) {
                state.bump();
            }
            else {
                state.record_expected("key");
                return Err(state.errors.last().cloned().expect("Error should have been recorded"));
            }

            self.skip_trivia(state);
            if !state.eat(JsonTokenType::Colon) {
                state.record_expected(":");
            }
            self.skip_trivia(state);
            self.parse_value(state)?;
            Ok(())
        })
    }

    fn parse_array<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if !state.eat(JsonTokenType::LeftBracket) {
            state.record_expected("[");
            return Err(state.errors.last().cloned().expect("Error should have been recorded"));
        }

        let mut first = true;
        while state.not_at_end() {
            self.skip_trivia(state);
            if state.at(JsonTokenType::RightBracket) {
                break;
            }

            if state.at(JsonTokenType::Eof) {
                return Err(state.unexpected_eof());
            }

            if !first {
                if !state.eat(JsonTokenType::Comma) {
                    state.record_expected(",");
                    break;
                }

                self.skip_trivia(state);
                if state.at(JsonTokenType::RightBracket) {
                    if !self.config.trailing_comma {
                        state.record_trailing_comma_not_allowed();
                        return Err(state.errors.last().cloned().expect("Error should have been recorded"));
                    }
                    break;
                }
            }
            first = false;

            self.parse_value(state)?;
            self.skip_trivia(state);
        }

        if !state.eat(JsonTokenType::RightBracket) {
            if state.at(JsonTokenType::Eof) || !state.not_at_end() {
                return Err(state.unexpected_eof());
            }
            state.record_expected("]");
            return Err(state.errors.last().cloned().expect("Error should have been recorded"));
        }
        Ok(())
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

impl<'config> Parser<JsonLanguage> for JsonParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<JsonLanguage>) -> oak_core::ParseOutput<'a, JsonLanguage> {
        let lexer = crate::lexer::JsonLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            let res = self.parse_value(state);
            if res.is_err() {
                // If parsing fails, we might still want to return a partial tree or an error tree
                // But parse_value handles errors by returning Result<(), OakError>
                // And it records errors in state.errors
            }

            // Ensure we consume all remaining trivia/whitespace
            while state.not_at_end() {
                if let Some(token) = state.current() {
                    if token.kind.is_ignored() {
                        state.bump(); // Changed from state.advance() to state.bump() to include in tree
                        continue;
                    }
                }
                break;
            }

            let root = state.finish_at(checkpoint, crate::parser::element_type::JsonElementType::Root);
            Ok(root)
        })
    }
}
