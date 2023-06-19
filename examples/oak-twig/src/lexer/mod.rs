use crate::{kind::TwigSyntaxKind, language::TwigLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

#[derive(Clone)]
pub struct TwigLexer<'config> {
    config: &'config TwigLanguage,
}

type State<S: Source> = LexerState<S, TwigLanguage>;

impl<'config> TwigLexer<'config> {
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Lexer<TwigLanguage> for TwigLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<TwigLanguage>,
    ) -> LexOutput<TwigLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TwigLexer<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(TwigSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Twig comment: {# ... #}
        if rest.starts_with("{#") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '#' && state.peek_next_n(1) == Some('}') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(TwigSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
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

    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let mut text = String::new();

                // 添加第一个字符
                text.push(ch);
                state.advance(ch.len_utf8());

                // 继续添加字母数字字符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();

                // 检查是否为布尔关键字
                let kind = match text.as_str() {
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
