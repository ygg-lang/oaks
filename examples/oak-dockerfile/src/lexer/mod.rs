use crate::{kind::DockerfileSyntaxKind, language::DockerfileLanguage};
use core::range::Range;
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, DockerfileLanguage>;

pub struct DockerfileLexer<'config> {
    config: &'config DockerfileLanguage,
}

impl<'config> DockerfileLexer<'config> {
    pub fn new(config: &'config DockerfileLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(DockerfileSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(DockerfileSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(DockerfileSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(DockerfileSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和指令
    fn lex_identifier_or_instruction(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in(core::range::Range { start: start_pos, end: end_pos }).unwrap_or("");

                let token_kind = match text.to_uppercase().as_str() {
                    "FROM" => DockerfileSyntaxKind::From,
                    "RUN" => DockerfileSyntaxKind::Run,
                    "CMD" => DockerfileSyntaxKind::Cmd,
                    "LABEL" => DockerfileSyntaxKind::Label,
                    "MAINTAINER" => DockerfileSyntaxKind::Maintainer,
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
                    "AS" => DockerfileSyntaxKind::As,
                    "NONE" => DockerfileSyntaxKind::None,
                    "INTERVAL" => DockerfileSyntaxKind::Interval,
                    "TIMEOUT" => DockerfileSyntaxKind::Timeout,
                    "START-PERIOD" => DockerfileSyntaxKind::StartPeriod,
                    "RETRIES" => DockerfileSyntaxKind::Retries,
                    _ => DockerfileSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    let dot_pos = state.get_position();
                    state.advance(1);

                    if let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                    else {
                        // 回退点号
                        state.set_position(dot_pos);
                    }
                }

                state.add_token(DockerfileSyntaxKind::Number, start_pos, state.get_position());
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

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(DockerfileSyntaxKind::String, start_pos, state.get_position());
                        return true;
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

                // 未闭合的字符                state.add_token(DockerfileSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理路径（不带引号的路径
    fn lex_path(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '/' || ch == '.' || ch == '~' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || matches!(ch, '/' | '.' | '-' | '_' | ':' | '~') {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                if state.get_position() > start_pos {
                    state.add_token(DockerfileSyntaxKind::Path, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理操作符和分隔
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '=' => DockerfileSyntaxKind::Equal,
                ',' => DockerfileSyntaxKind::Comma,
                '[' => DockerfileSyntaxKind::LeftBracket,
                ']' => DockerfileSyntaxKind::RightBracket,
                '(' => DockerfileSyntaxKind::LeftParen,
                ')' => DockerfileSyntaxKind::RightParen,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理其他文本（作为标识符
    fn lex_other_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_whitespace() || matches!(ch, '=' | ',' | '[' | ']' | '(' | ')' | '#') {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(DockerfileSyntaxKind::Identifier, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<DockerfileLanguage> for DockerfileLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<DockerfileSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_or_instruction(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_path(&mut state) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            if self.lex_other_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DockerfileSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DockerfileSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
