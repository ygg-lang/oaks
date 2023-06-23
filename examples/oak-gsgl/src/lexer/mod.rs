#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::GsglLanguage, lexer::token_type::GsglTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, GsglLanguage>;

/// GSGL lexer.
#[derive(Clone, Debug)]
pub struct GsglLexer<'config> {
    _config: &'config GsglLanguage,
}

impl<'config> GsglLexer<'config> {
    pub fn new(config: &'config GsglLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let start = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator_or_delimiter(state) {
                continue;
            }

            // If no method handles the current character, create an error token and advance
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GsglTokenType::Error, start, state.get_position());
            }
            else {
                break;
            }
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        if state.get_position() > start {
            state.add_token(GsglTokenType::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some('\n') {
            state.advance(1);
            state.add_token(GsglTokenType::Newline, start, state.get_position());
            true
        }
        else if state.peek() == Some('\r') && state.peek_next_n(1) == Some('\n') {
            state.advance(2);
            state.add_token(GsglTokenType::Newline, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('/') && state.peek_next_n(1) == Some('/') {
            // Single-line comment
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(1);
            }
            state.add_token(GsglTokenType::Comment, start, state.get_position());
            true
        }
        else if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            // Multi-line comment
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(1);
            }
            state.add_token(GsglTokenType::Comment, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('"') {
            state.advance(1); // Consume start quote

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // Consume end quote
                    state.add_token(GsglTokenType::String, start, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // Consume escape character
                    if state.peek().is_some() {
                        state.advance(1); // Consume escaped character
                    }
                }
                else {
                    state.advance(1);
                }
            }

            // Unterminated string
            state.add_token(GsglTokenType::String, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('\'') {
            state.advance(1); // Consume start single quote

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // Consume escape character
                    if state.peek().is_some() {
                        state.advance(1); // Consume escaped character
                    }
                }
                else if ch != '\'' {
                    state.advance(1); // Consume character
                }
            }

            if state.peek() == Some('\'') {
                state.advance(1); // Consume end single quote
            }

            state.add_token(GsglTokenType::String, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // Consume digits
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // Check for decimal point
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // Consume '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // Check for scientific notation
                if matches!(state.peek(), Some('e') | Some('E')) {
                    state.advance(1);
                    if matches!(state.peek(), Some('+') | Some('-')) {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // Check for float suffix
                if matches!(state.peek(), Some('f') | Some('F')) {
                    state.advance(1);
                }

                state.add_token(GsglTokenType::Number, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in(oak_core::Range { start, end });
                let kind = match text.as_ref() {
                    "shader" => GsglTokenType::Shader,
                    "vertex" => GsglTokenType::Vertex,
                    "fragment" => GsglTokenType::Fragment,
                    "geometry" => GsglTokenType::Geometry,
                    "compute" => GsglTokenType::Compute,
                    "uniform" => GsglTokenType::Uniform,
                    "attribute" => GsglTokenType::Attribute,
                    "varying" => GsglTokenType::Varying,
                    "in" => GsglTokenType::In,
                    "out" => GsglTokenType::Out,
                    "inout" => GsglTokenType::Inout,
                    "const" => GsglTokenType::Const,
                    "struct" => GsglTokenType::Struct,
                    "if" => GsglTokenType::If,
                    "else" => GsglTokenType::Else,
                    "for" => GsglTokenType::For,
                    "while" => GsglTokenType::While,
                    "do" => GsglTokenType::Do,
                    "break" => GsglTokenType::Break,
                    "continue" => GsglTokenType::Continue,
                    "return" => GsglTokenType::Return,
                    "discard" => GsglTokenType::Discard,
                    "true" => GsglTokenType::True,
                    "false" => GsglTokenType::False,
                    "float" => GsglTokenType::Float,
                    "int" => GsglTokenType::Int,
                    "bool" => GsglTokenType::Bool,
                    "vec2" => GsglTokenType::Vec2,
                    "vec3" => GsglTokenType::Vec3,
                    "vec4" => GsglTokenType::Vec4,
                    "mat2" => GsglTokenType::Mat2,
                    "mat3" => GsglTokenType::Mat3,
                    "mat4" => GsglTokenType::Mat4,
                    "sampler2D" => GsglTokenType::Sampler2D,
                    "samplerCube" => GsglTokenType::SamplerCube,
                    "void" => GsglTokenType::Void,
                    _ => GsglTokenType::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Check for two-character operators
        if let (Some(ch1), Some(ch2)) = (state.peek(), state.peek_next_n(1)) {
            let two_char = format!("{}{}", ch1, ch2);
            let kind = match two_char.as_str() {
                "+=" => Some(GsglTokenType::PlusAssign),
                "-=" => Some(GsglTokenType::MinusAssign),
                "*=" => Some(GsglTokenType::StarAssign),
                "/=" => Some(GsglTokenType::SlashAssign),
                "==" => Some(GsglTokenType::Eq),
                "!=" => Some(GsglTokenType::Ne),
                "<=" => Some(GsglTokenType::Le),
                ">=" => Some(GsglTokenType::Ge),
                "&&" => Some(GsglTokenType::And),
                "||" => Some(GsglTokenType::Or),
                "<<" => Some(GsglTokenType::LeftShift),
                ">>" => Some(GsglTokenType::RightShift),
                _ => None,
            };

            if let Some(kind) = kind {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators and delimiters
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => Some(GsglTokenType::Plus),
                '-' => Some(GsglTokenType::Minus),
                '*' => Some(GsglTokenType::Star),
                '/' => Some(GsglTokenType::Slash),
                '%' => Some(GsglTokenType::Percent),
                '=' => Some(GsglTokenType::Assign),
                '!' => Some(GsglTokenType::Not),
                '<' => Some(GsglTokenType::Lt),
                '>' => Some(GsglTokenType::Gt),
                '&' => Some(GsglTokenType::BitAnd),
                '|' => Some(GsglTokenType::BitOr),
                '^' => Some(GsglTokenType::BitXor),
                '~' => Some(GsglTokenType::BitNot),
                '?' => Some(GsglTokenType::Question),
                ':' => Some(GsglTokenType::Colon),
                '#' => Some(GsglTokenType::Hash),
                ';' => Some(GsglTokenType::Semicolon),
                ',' => Some(GsglTokenType::Comma),
                '.' => Some(GsglTokenType::Dot),
                '(' => Some(GsglTokenType::LeftParen),
                ')' => Some(GsglTokenType::RightParen),
                '[' => Some(GsglTokenType::LeftBracket),
                ']' => Some(GsglTokenType::RightBracket),
                '{' => Some(GsglTokenType::LeftBrace),
                '}' => Some(GsglTokenType::RightBrace),
                _ => None,
            };

            if let Some(kind) = kind {
                state.advance(ch.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }
}

impl<'config> Lexer<GsglLanguage> for GsglLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<GsglLanguage>) -> LexOutput<GsglLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
