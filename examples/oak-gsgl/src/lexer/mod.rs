use crate::{GsglLanguage, syntax::GsglSyntaxKind};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, GsglLanguage>;

/// GSGL 词法分析器
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

            // 如果没有任何方法处理当前字符，创建错误 token 并前进
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GsglSyntaxKind::Error, start, state.get_position());
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
            state.add_token(GsglSyntaxKind::Whitespace, start, state.get_position());
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
            state.add_token(GsglSyntaxKind::Newline, start, state.get_position());
            true
        }
        else if state.peek() == Some('\r') && state.peek_next_n(1) == Some('\n') {
            state.advance(2);
            state.add_token(GsglSyntaxKind::Newline, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('/') && state.peek_next_n(1) == Some('/') {
            // 单行注释
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(1);
            }
            state.add_token(GsglSyntaxKind::Comment, start, state.get_position());
            true
        }
        else if state.peek() == Some('/') && state.peek_next_n(1) == Some('*') {
            // 多行注释
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(1);
            }
            state.add_token(GsglSyntaxKind::Comment, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('"') {
            state.advance(1); // 消费开始的引号

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 消费结束的引号
                    state.add_token(GsglSyntaxKind::String, start, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // 消费转义字符
                    if state.peek().is_some() {
                        state.advance(1); // 消费被转义的字符
                    }
                }
                else {
                    state.advance(1);
                }
            }

            // 未闭合的字符串
            state.add_token(GsglSyntaxKind::String, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.peek() == Some('\'') {
            state.advance(1); // 消费开始的单引号

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1); // 消费转义字符
                    if state.peek().is_some() {
                        state.advance(1); // 消费被转义的字符
                    }
                }
                else if ch != '\'' {
                    state.advance(1); // 消费字符
                }
            }

            if state.peek() == Some('\'') {
                state.advance(1); // 消费结束的单引号
            }

            state.add_token(GsglSyntaxKind::String, start, state.get_position());
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
                // 消费数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // 消费 '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查科学记数法
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

                // 检查浮点后缀
                if matches!(state.peek(), Some('f') | Some('F')) {
                    state.advance(1);
                }

                state.add_token(GsglSyntaxKind::Number, start, state.get_position());
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
                    "shader" => GsglSyntaxKind::Shader,
                    "vertex" => GsglSyntaxKind::Vertex,
                    "fragment" => GsglSyntaxKind::Fragment,
                    "geometry" => GsglSyntaxKind::Geometry,
                    "compute" => GsglSyntaxKind::Compute,
                    "uniform" => GsglSyntaxKind::Uniform,
                    "attribute" => GsglSyntaxKind::Attribute,
                    "varying" => GsglSyntaxKind::Varying,
                    "in" => GsglSyntaxKind::In,
                    "out" => GsglSyntaxKind::Out,
                    "inout" => GsglSyntaxKind::Inout,
                    "const" => GsglSyntaxKind::Const,
                    "struct" => GsglSyntaxKind::Struct,
                    "if" => GsglSyntaxKind::If,
                    "else" => GsglSyntaxKind::Else,
                    "for" => GsglSyntaxKind::For,
                    "while" => GsglSyntaxKind::While,
                    "do" => GsglSyntaxKind::Do,
                    "break" => GsglSyntaxKind::Break,
                    "continue" => GsglSyntaxKind::Continue,
                    "return" => GsglSyntaxKind::Return,
                    "discard" => GsglSyntaxKind::Discard,
                    "true" => GsglSyntaxKind::True,
                    "false" => GsglSyntaxKind::False,
                    "float" => GsglSyntaxKind::Float,
                    "int" => GsglSyntaxKind::Int,
                    "bool" => GsglSyntaxKind::Bool,
                    "vec2" => GsglSyntaxKind::Vec2,
                    "vec3" => GsglSyntaxKind::Vec3,
                    "vec4" => GsglSyntaxKind::Vec4,
                    "mat2" => GsglSyntaxKind::Mat2,
                    "mat3" => GsglSyntaxKind::Mat3,
                    "mat4" => GsglSyntaxKind::Mat4,
                    "sampler2D" => GsglSyntaxKind::Sampler2D,
                    "samplerCube" => GsglSyntaxKind::SamplerCube,
                    "void" => GsglSyntaxKind::Void,
                    _ => GsglSyntaxKind::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operator_or_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 检查双字符操作符
        if let (Some(ch1), Some(ch2)) = (state.peek(), state.peek_next_n(1)) {
            let two_char = format!("{}{}", ch1, ch2);
            let kind = match two_char.as_str() {
                "+=" => Some(GsglSyntaxKind::PlusAssign),
                "-=" => Some(GsglSyntaxKind::MinusAssign),
                "*=" => Some(GsglSyntaxKind::StarAssign),
                "/=" => Some(GsglSyntaxKind::SlashAssign),
                "==" => Some(GsglSyntaxKind::Eq),
                "!=" => Some(GsglSyntaxKind::Ne),
                "<=" => Some(GsglSyntaxKind::Le),
                ">=" => Some(GsglSyntaxKind::Ge),
                "&&" => Some(GsglSyntaxKind::And),
                "||" => Some(GsglSyntaxKind::Or),
                "<<" => Some(GsglSyntaxKind::LeftShift),
                ">>" => Some(GsglSyntaxKind::RightShift),
                _ => None,
            };

            if let Some(kind) = kind {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        // 单字符操作符和分隔符
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => Some(GsglSyntaxKind::Plus),
                '-' => Some(GsglSyntaxKind::Minus),
                '*' => Some(GsglSyntaxKind::Star),
                '/' => Some(GsglSyntaxKind::Slash),
                '%' => Some(GsglSyntaxKind::Percent),
                '=' => Some(GsglSyntaxKind::Assign),
                '!' => Some(GsglSyntaxKind::Not),
                '<' => Some(GsglSyntaxKind::Lt),
                '>' => Some(GsglSyntaxKind::Gt),
                '&' => Some(GsglSyntaxKind::BitAnd),
                '|' => Some(GsglSyntaxKind::BitOr),
                '^' => Some(GsglSyntaxKind::BitXor),
                '~' => Some(GsglSyntaxKind::BitNot),
                '?' => Some(GsglSyntaxKind::Question),
                ':' => Some(GsglSyntaxKind::Colon),
                ';' => Some(GsglSyntaxKind::Semicolon),
                ',' => Some(GsglSyntaxKind::Comma),
                '.' => Some(GsglSyntaxKind::Dot),
                '(' => Some(GsglSyntaxKind::LeftParen),
                ')' => Some(GsglSyntaxKind::RightParen),
                '[' => Some(GsglSyntaxKind::LeftBracket),
                ']' => Some(GsglSyntaxKind::RightBracket),
                '{' => Some(GsglSyntaxKind::LeftBrace),
                '}' => Some(GsglSyntaxKind::RightBrace),
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
