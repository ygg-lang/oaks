use crate::{kind::SmalltalkSyntaxKind, language::SmalltalkLanguage};
use oak_core::{
    OakError,
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, SmalltalkLanguage>;

#[derive(Clone)]
pub struct SmalltalkLexer<'config> {
    _config: &'config SmalltalkLanguage,
}

impl<'config> Lexer<SmalltalkLanguage> for SmalltalkLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], cache: &'a mut impl LexerCache<SmalltalkLanguage>) -> LexOutput<SmalltalkLanguage> {
        let relex_from = edits.iter().map(|e| e.span.start).min().unwrap_or(source.length());
        let mut state = LexerState::new_with_cache(source, relex_from, cache);
        if state.fully_reused() {
            let result = Ok(());
            return state.finish_with_cache(result, cache);
        }
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SmalltalkLexer<'config> {
    pub fn new(config: &'config SmalltalkLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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
                state.add_token(SmalltalkSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        state.add_eof();
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(SmalltalkSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行符
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SmalltalkSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SmalltalkSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

            state.add_token(SmalltalkSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

                state.add_token(SmalltalkSyntaxKind::Identifier, start_pos, state.get_position());
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
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

                state.add_token(SmalltalkSyntaxKind::Number, start_pos, state.get_position());
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
    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => SmalltalkSyntaxKind::LeftParen,
                ')' => SmalltalkSyntaxKind::RightParen,
                '[' => SmalltalkSyntaxKind::LeftBracket,
                ']' => SmalltalkSyntaxKind::RightBracket,
                '{' => SmalltalkSyntaxKind::LeftBrace,
                '}' => SmalltalkSyntaxKind::RightBrace,
                '.' => SmalltalkSyntaxKind::Dot,
                ';' => SmalltalkSyntaxKind::Semicolon,
                ',' => SmalltalkSyntaxKind::Comma,
                '+' => SmalltalkSyntaxKind::Plus,
                '-' => SmalltalkSyntaxKind::Minus,
                '*' => SmalltalkSyntaxKind::Star,
                '/' => SmalltalkSyntaxKind::Slash,
                '=' => SmalltalkSyntaxKind::Equal,
                '<' => SmalltalkSyntaxKind::Less,
                '>' => SmalltalkSyntaxKind::Greater,
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
