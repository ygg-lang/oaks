use crate::{kind::RSyntaxKind, language::RLanguage};
use oak_core::{Lexer, LexerCache, LexerState, Range, lexer::LexOutput, source::Source};

type State<'s, S> = LexerState<'s, S, RLanguage>;

#[derive(Clone)]
pub struct RLexer<'config> {
    _config: &'config RLanguage,
}

impl<'config> Lexer<RLanguage> for RLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<RLanguage>) -> LexOutput<RLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RLexer<'config> {
    pub fn new(_config: &'config RLanguage) -> Self {
        Self { _config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_other(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }

    /// 跳过空白符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                return true;
            }
        }
        false
    }

    /// 处理注释
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some('#') = state.current() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行尾
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(RSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(quote) = state.current() {
            if quote == '"' || quote == '\'' {
                let start_pos = state.get_position();
                state.advance(1); // 跳过引号

                while let Some(ch) = state.current() {
                    if ch == quote {
                        state.advance(1); // 跳过结束引号
                        state.add_token(RSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    if ch == '\\' {
                        state.advance(1);
                        if let Some(escaped) = state.current() {
                            state.advance(escaped.len_utf8());
                            continue;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                // 未闭合字符串
                state.add_token(RSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                let start_pos = state.get_position();
                let mut has_dot = false;

                while let Some(c) = state.current() {
                    if c.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if c == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else if (c == 'e' || c == 'E') && !state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit() || c == '+' || c == '-') {
                        break;
                    }
                    else if c == 'e' || c == 'E' {
                        state.advance(1);
                        if let Some(next) = state.current() {
                            if next == '+' || next == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(digit) = state.current() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else if c == 'L' {
                        state.advance(1);
                        state.add_token(RSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if c == 'i' {
                        state.advance(1);
                        state.add_token(RSyntaxKind::FloatLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        break;
                    }
                }

                let kind = if has_dot { RSyntaxKind::FloatLiteral } else { RSyntaxKind::IntegerLiteral };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_alphabetic() || ch == '.' || ch == '_' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());

                while let Some(c) = state.current() {
                    if c.is_alphanumeric() || c == '.' || c == '_' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in(Range { start: start_pos, end: state.get_position() });
                let kind = match text.as_ref() {
                    "if" => RSyntaxKind::If,
                    "else" => RSyntaxKind::Else,
                    "for" => RSyntaxKind::For,
                    "in" => RSyntaxKind::In,
                    "while" => RSyntaxKind::While,
                    "repeat" => RSyntaxKind::Repeat,
                    "next" => RSyntaxKind::Next,
                    "break" => RSyntaxKind::Break,
                    "function" => RSyntaxKind::Function,
                    "TRUE" => RSyntaxKind::True,
                    "FALSE" => RSyntaxKind::False,
                    "NULL" => RSyntaxKind::Null,
                    "Inf" => RSyntaxKind::Inf,
                    "NaN" => RSyntaxKind::NaN,
                    "NA" => RSyntaxKind::NA,
                    "NA_integer_" => RSyntaxKind::NaInteger,
                    "NA_real_" => RSyntaxKind::NaReal,
                    "NA_complex_" => RSyntaxKind::NaComplex,
                    "NA_character_" => RSyntaxKind::NaCharacter,
                    _ => RSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理操作符
    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.current() {
            match ch {
                '<' => {
                    state.advance(1);
                    if let Some('-') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::LeftArrow, start_pos, state.get_position());
                        return true;
                    }
                    if let Some('<') = state.current() {
                        state.advance(1);
                        if let Some('-') = state.current() {
                            state.advance(1);
                            state.add_token(RSyntaxKind::DoubleLeftArrow, start_pos, state.get_position());
                            return true;
                        }
                    }
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::LessEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Less, start_pos, state.get_position());
                    return true;
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.current() {
                        state.advance(1);
                        if let Some('>') = state.current() {
                            state.advance(1);
                            state.add_token(RSyntaxKind::DoubleRightArrow, start_pos, state.get_position());
                            return true;
                        }
                        state.add_token(RSyntaxKind::RightArrow, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Minus, start_pos, state.get_position());
                    return true;
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::EqualEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Equal, start_pos, state.get_position());
                    return true;
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::NotEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Not, start_pos, state.get_position());
                    return true;
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::GreaterEqual, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Greater, start_pos, state.get_position());
                    return true;
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::AndAnd, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::And, start_pos, state.get_position());
                    return true;
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::OrOr, start_pos, state.get_position());
                        return true;
                    }
                    if let Some('>') = state.current() {
                        state.advance(1);
                        state.add_token(RSyntaxKind::Pipe, start_pos, state.get_position());
                        return true;
                    }
                    state.add_token(RSyntaxKind::Or, start_pos, state.get_position());
                    return true;
                }
                '%' => {
                    state.advance(1);
                    while let Some(c) = state.current() {
                        state.advance(c.len_utf8());
                        if c == '%' {
                            state.add_token(RSyntaxKind::Operator, start_pos, state.get_position());
                            return true;
                        }
                    }
                    // 未闭合的操作符
                    state.add_token(RSyntaxKind::Operator, start_pos, state.get_position());
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    /// 处理单字符标记
    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            let kind = match ch {
                '(' => Some(RSyntaxKind::LeftParen),
                ')' => Some(RSyntaxKind::RightParen),
                '[' => Some(RSyntaxKind::LeftBracket),
                ']' => Some(RSyntaxKind::RightBracket),
                '{' => Some(RSyntaxKind::LeftBrace),
                '}' => Some(RSyntaxKind::RightBrace),
                ',' => Some(RSyntaxKind::Comma),
                ';' => Some(RSyntaxKind::Semicolon),
                '+' => Some(RSyntaxKind::Plus),
                '*' => Some(RSyntaxKind::Star),
                '/' => Some(RSyntaxKind::Slash),
                '^' => Some(RSyntaxKind::Caret),
                '$' => Some(RSyntaxKind::Dollar),
                '@' => Some(RSyntaxKind::At),
                '~' => Some(RSyntaxKind::Tilde),
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.current() {
                        state.advance(1);
                        if let Some(':') = state.current() {
                            state.advance(1);
                            Some(RSyntaxKind::TripleColon)
                        }
                        else {
                            Some(RSyntaxKind::DoubleColon)
                        }
                    }
                    else {
                        return {
                            state.add_token(RSyntaxKind::Colon, start_pos, state.get_position());
                            true
                        };
                    }
                }
                '?' => Some(RSyntaxKind::Question),
                _ => None,
            };

            if let Some(k) = kind {
                if !matches!(k, RSyntaxKind::TripleColon | RSyntaxKind::DoubleColon) {
                    state.advance(1);
                }
                state.add_token(k, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理其他字符
    fn lex_other<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            let len = ch.len_utf8();
            state.advance(len);
            state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }
        false
    }
}
