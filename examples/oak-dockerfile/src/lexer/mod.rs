use crate::{kind::DockerfileSyntaxKind, language::DockerfileLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, DockerfileLanguage>;

static DOCKERFILE_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone)]
pub struct DockerfileLexer<'config> {
    _config: &'config DockerfileLanguage,
}

impl<'config> Lexer<DockerfileLanguage> for DockerfileLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DockerfileLanguage>) -> LexOutput<DockerfileLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DockerfileLexer<'config> {
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
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

            if self.lex_identifier_or_instruction(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_path(state) {
                continue;
            }

            if self.lex_operators_and_delimiters(state) {
                continue;
            }

            if self.lex_other(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        DOCKERFILE_WHITESPACE.scan(state, DockerfileSyntaxKind::Whitespace)
    }

    /// 处理换行符
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(DockerfileSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(DockerfileSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理注释
    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.peek() == Some('#') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DockerfileSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理标识符或指令
    fn lex_identifier_or_instruction<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

                let end_pos = state.get_position();
                let text = state.get_text_in((start..end_pos).into());

                // 检查是否是 Dockerfile 指令
                let kind = match text.to_uppercase().as_str() {
                    "FROM" => DockerfileSyntaxKind::From,
                    "RUN" => DockerfileSyntaxKind::Run,
                    "CMD" => DockerfileSyntaxKind::Cmd,
                    "LABEL" => DockerfileSyntaxKind::Label,
                    "EXPOSE" => DockerfileSyntaxKind::Expose,
                    "ENV" => DockerfileSyntaxKind::Env,
                    "ADD" => DockerfileSyntaxKind::Add,
                    "COPY" => DockerfileSyntaxKind::Copy,
                    "ENTRYPOINT" => DockerfileSyntaxKind::Entrypoint,
                    "VOLUME" => DockerfileSyntaxKind::Volume,
                    "USER" => DockerfileSyntaxKind::User,
                    "WORKDIR" => DockerfileSyntaxKind::Workdir,
                    "ARG" => DockerfileSyntaxKind::Arg,
                    "ONBUILD" => DockerfileSyntaxKind::Onbuild,
                    "STOPSIGNAL" => DockerfileSyntaxKind::Stopsignal,
                    "HEALTHCHECK" => DockerfileSyntaxKind::Healthcheck,
                    "SHELL" => DockerfileSyntaxKind::Shell,
                    "MAINTAINER" => DockerfileSyntaxKind::Maintainer,
                    "AS" => DockerfileSyntaxKind::As,
                    "NONE" => DockerfileSyntaxKind::None,
                    "INTERVAL" => DockerfileSyntaxKind::Interval,
                    "TIMEOUT" => DockerfileSyntaxKind::Timeout,
                    "START_PERIOD" => DockerfileSyntaxKind::StartPeriod,
                    "RETRIES" => DockerfileSyntaxKind::Retries,
                    _ => DockerfileSyntaxKind::Identifier,
                };

                state.add_token(kind, start, end_pos);
                return true;
            }
        }
        false
    }

    /// 处理数字
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

                state.add_token(DockerfileSyntaxKind::Number, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
                        if state.peek().is_some() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(DockerfileSyntaxKind::String, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理路径
    fn lex_path<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            if ch == '/' || ch == '.' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '/' || ch == '.' || ch == '-' || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                state.add_token(DockerfileSyntaxKind::Path, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理运算符和分隔符
    fn lex_operators_and_delimiters<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '=' => DockerfileSyntaxKind::Equal,
                ':' => DockerfileSyntaxKind::Colon,
                '{' => DockerfileSyntaxKind::LeftBrace,
                '}' => DockerfileSyntaxKind::RightBrace,
                '[' => DockerfileSyntaxKind::LeftBracket,
                ']' => DockerfileSyntaxKind::RightBracket,
                '(' => DockerfileSyntaxKind::LeftParen,
                ')' => DockerfileSyntaxKind::RightParen,
                ',' => DockerfileSyntaxKind::Comma,
                ';' => DockerfileSyntaxKind::Semicolon,
                '$' => DockerfileSyntaxKind::Dollar,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理其他字符
    fn lex_other<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(DockerfileSyntaxKind::Error, start, state.get_position());
            return true;
        }
        false
    }
}
