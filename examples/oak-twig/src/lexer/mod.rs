use crate::{kind::TwigSyntaxKind, language::TwigLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

#[derive(Clone, Debug)]
pub struct TwigLexer<'config> {
    /// 语言配置
    _config: &'config TwigLanguage,
}

type State<'a, S> = LexerState<'a, S, TwigLanguage>;

impl<'config> Lexer<TwigLanguage> for TwigLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<TwigLanguage>) -> LexOutput<TwigLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TwigLexer<'config> {
    /// 创建新的 Twig 词法分析器
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { _config: config }
    }
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_punctuation(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let mut found = false;

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() {
                state.advance(ch.len_utf8());
                found = true;
            }
            else {
                break;
            }
        }

        if found {
            state.add_token(TwigSyntaxKind::Whitespace, start, state.get_position());
        }

        found
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("{#") {
            while state.not_at_end() {
                if state.consume_if_starts_with("#}") {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(TwigSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(TwigSyntaxKind::String, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(TwigSyntaxKind::Number, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 双字符操作符
        if rest.starts_with("{{") {
            state.advance(2);
            state.add_token(TwigSyntaxKind::DoubleLeftBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with("}}") {
            state.advance(2);
            state.add_token(TwigSyntaxKind::DoubleRightBrace, start, state.get_position());
            return true;
        }
        if rest.starts_with("{%") {
            state.advance(2);
            state.add_token(TwigSyntaxKind::LeftBracePercent, start, state.get_position());
            return true;
        }
        if rest.starts_with("%}") {
            state.advance(2);
            state.add_token(TwigSyntaxKind::PercentRightBrace, start, state.get_position());
            return true;
        }

        // 单字符操作符
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '{' => TwigSyntaxKind::LeftBrace,
                '}' => TwigSyntaxKind::RightBrace,
                '(' => TwigSyntaxKind::LeftParen,
                ')' => TwigSyntaxKind::RightParen,
                '[' => TwigSyntaxKind::LeftBracket,
                ']' => TwigSyntaxKind::RightBracket,
                ',' => TwigSyntaxKind::Comma,
                '.' => TwigSyntaxKind::Dot,
                ':' => TwigSyntaxKind::Colon,
                ';' => TwigSyntaxKind::Semicolon,
                '|' => TwigSyntaxKind::Pipe,
                '=' => TwigSyntaxKind::Eq,
                '+' => TwigSyntaxKind::Plus,
                '-' => TwigSyntaxKind::Minus,
                '*' => TwigSyntaxKind::Star,
                '/' => TwigSyntaxKind::Slash,
                '%' => TwigSyntaxKind::Percent,
                '!' => TwigSyntaxKind::Bang,
                '?' => TwigSyntaxKind::Question,
                '<' => TwigSyntaxKind::Lt,
                '>' => TwigSyntaxKind::Gt,
                '&' => TwigSyntaxKind::Amp,
                '^' => TwigSyntaxKind::Caret,
                '~' => TwigSyntaxKind::Tilde,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());

                // 检查是否为布尔关键字
                let kind = match text.as_ref() {
                    "true" | "false" => TwigSyntaxKind::Boolean,
                    _ => TwigSyntaxKind::Identifier,
                };
                state.add_token(kind, start, end);
                return true;
            }
        }
        false
    }
}
