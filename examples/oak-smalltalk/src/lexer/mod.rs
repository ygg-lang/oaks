use crate::{kind::SmalltalkKind, language::SmalltalkLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, SmalltalkLanguage>;

#[derive(Clone)]
pub struct SmalltalkLexer<'config> {
    config: &'config SmalltalkLanguage,
}

impl<'config> SmalltalkLexer<'config> {
    pub fn new(config: &'config SmalltalkLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            // 错误处理：如果没有匹配任何规则，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SmalltalkKind::Error, start_pos, state.get_position());
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(SmalltalkKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(SmalltalkKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行符
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SmalltalkKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SmalltalkKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(SmalltalkKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符
    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(SmalltalkKind::Identifier, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(SmalltalkKind::Number, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标点符号
    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => SmalltalkKind::LeftParen,
                ')' => SmalltalkKind::RightParen,
                '[' => SmalltalkKind::LeftBracket,
                ']' => SmalltalkKind::RightBracket,
                '{' => SmalltalkKind::LeftBrace,
                '}' => SmalltalkKind::RightBrace,
                '.' => SmalltalkKind::Dot,
                ';' => SmalltalkKind::Semicolon,
                ',' => SmalltalkKind::Comma,
                '+' => SmalltalkKind::Plus,
                '-' => SmalltalkKind::Minus,
                '*' => SmalltalkKind::Star,
                '/' => SmalltalkKind::Slash,
                '=' => SmalltalkKind::Equal,
                '<' => SmalltalkKind::Less,
                '>' => SmalltalkKind::Greater,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<SmalltalkLanguage> for SmalltalkLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<SmalltalkLanguage>,
    ) -> LexOutput<SmalltalkLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
