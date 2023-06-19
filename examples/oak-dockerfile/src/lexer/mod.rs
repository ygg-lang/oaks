use crate::{kind::DockerfileSyntaxKind, language::DockerfileLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, DockerfileLanguage>;

static DOCKERFILE_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

#[derive(Clone)]
pub struct DockerfileLexer<'config> {
    config: &'config DockerfileLanguage,
}

impl<'config> Lexer<DockerfileLanguage> for DockerfileLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<DockerfileLanguage>,
    ) -> LexOutput<DockerfileLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> DockerfileLexer<'config> {
    pub fn new(config: &'config DockerfileLanguage) -> Self {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(DockerfileSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match DOCKERFILE_WHITESPACE.scan(state.rest(), state.get_position(), DockerfileSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 处理换行符
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_identifier_or_instruction<S: Source>(&self, state: &mut State<S>) -> bool {
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

                state.add_token(DockerfileSyntaxKind::Number, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理字符串
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
    fn lex_path<S: Source>(&self, state: &mut State<S>) -> bool {
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

    /// 处理操作符和分隔符
    fn lex_operators_and_delimiters<S: Source>(&self, state: &mut State<S>) -> bool {
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
    fn lex_other<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
            state.advance(ch.len_utf8());
            state.add_token(DockerfileSyntaxKind::Error, start, state.get_position());
            return true;
        }
        false
    }
}
