use crate::{kind::RSyntaxKind, language::RLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, RLanguage>;

#[derive(Clone)]
pub struct RLexer<'config> {
    _config: &'config (),
}

impl<'config> Lexer<RLanguage> for RLexer<'config> {
    fn lex(&self, source: impl Source) -> LexOutput<RLanguage> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_single_char_tokens(&mut state) {
                continue;
            }

            if self.lex_other(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，前进一个字符避免无限循环
            if let Some(ch) = state.current() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(RSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }

    fn lex_incremental(
        &self,
        source: impl Source,
        _offset: usize,
        _cache: IncrementalCache<'_, RLanguage>,
    ) -> LexOutput<RLanguage> {
        let mut state = LexerState::new_with_cache(source, _offset, _cache);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_single_char_tokens(&mut state) {
                continue;
            }

            if self.lex_other(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，前进一个字符避免无限循环
            if let Some(ch) = state.current() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(RSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}

impl<'config> RLexer<'config> {
    pub fn new(_config: &'config ()) -> Self {
        Self { _config }
    }

    /// 跳过空白符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                return true;
            }
        }
        false
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(quote) = state.current() {
            if quote == '"' || quote == '\'' {
                let start_pos = state.get_position();
                state.advance(1); // 跳过开始引号

                // 读取字符串内容
                while let Some(ch) = state.current() {
                    if ch == quote {
                        state.advance(1); // 跳过结束引号
                        state.add_token(RSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        // 处理转义字符
                        state.advance(1);
                        if let Some(_) = state.current() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符串
                state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                // 读取数字
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.current() {
                    state.advance(1);
                    while let Some(ch) = state.current() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(RSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_alphabetic() || ch == '_' || ch == '.' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());

                while let Some(ch) = state.current() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 简化关键字识别
                state.add_token(RSyntaxKind::Identifier, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => RSyntaxKind::Plus,
                '-' => RSyntaxKind::Minus,
                '*' => RSyntaxKind::Star,
                '/' => RSyntaxKind::Slash,
                '%' => RSyntaxKind::Percent,
                '^' => RSyntaxKind::Caret,
                '=' => RSyntaxKind::Equal,
                '<' => RSyntaxKind::Less,
                '>' => RSyntaxKind::Greater,
                '!' => RSyntaxKind::Not,
                '&' => RSyntaxKind::And,
                '|' => RSyntaxKind::Or,
                '~' => RSyntaxKind::Tilde,
                ':' => RSyntaxKind::Colon,
                '.' => RSyntaxKind::Dot,
                '$' => RSyntaxKind::Dollar,
                '@' => RSyntaxKind::At,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理单字符 token
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();

            let kind = match ch {
                '(' => RSyntaxKind::LeftParen,
                ')' => RSyntaxKind::RightParen,
                '{' => RSyntaxKind::LeftBrace,
                '}' => RSyntaxKind::RightBrace,
                '[' => RSyntaxKind::LeftBracket,
                ']' => RSyntaxKind::RightBracket,
                ',' => RSyntaxKind::Comma,
                ';' => RSyntaxKind::Semicolon,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }
        false
    }

    /// 处理其他字符
    fn lex_other<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());
            state.add_token(RSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }
        false
    }
}
