use crate::{kind::NginxSyntaxKind, language::NginxLanguage};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, NginxLanguage>;

#[derive(Clone)]
pub struct NginxLexer<'config> {
    _config: &'config NginxLanguage,
}

impl<'config> NginxLexer<'config> {
    pub fn new(config: &'config NginxLanguage) -> Self {
        Self { _config: config }
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
            state.add_token(NginxSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NginxSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NginxSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(NginxSyntaxKind::CommentToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote != '"' && quote != '\'' {
                return false;
            }

            state.advance(1); // 跳过开始引
            while let Some(ch) = state.peek() {
                if ch == quote {
                    state.advance(1); // 跳过结束引号
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(c) = state.peek() {
                        state.advance(c.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NginxSyntaxKind::String, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }

            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 处理小数
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 处理单位后缀 (k, m, g, s, ms, etc.)
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() {
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphabetic() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            state.add_token(NginxSyntaxKind::Number, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理路径
    fn lex_path<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '/' || ch == '.' || ch == '-' || ch == '_' || ch == '*' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(NginxSyntaxKind::Path, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 URL
    fn lex_url<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否以 http:// https:// 开
        if state.starts_with("http://") || state.starts_with("https://") {
            let scheme_len = if state.starts_with("https://") { 8 } else { 7 };
            state.advance(scheme_len);

            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '.' || ch == '/' || ch == ':' || ch == '-' || ch == '_' || ch == '?' || ch == '&' || ch == '=' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(NginxSyntaxKind::Url, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' {
                return false;
            }

            // 收集标识符字
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 检查是否是关键
            let end_pos = state.get_position();
            let text = state.source().get_text_in(oak_core::Range { start: start_pos, end: end_pos });
            let token_kind = match text.as_ref() {
                "server" => NginxSyntaxKind::ServerKeyword,
                "location" => NginxSyntaxKind::LocationKeyword,
                "upstream" => NginxSyntaxKind::UpstreamKeyword,
                "http" => NginxSyntaxKind::HttpKeyword,
                "events" => NginxSyntaxKind::EventsKeyword,
                "listen" => NginxSyntaxKind::ListenKeyword,
                "server_name" => NginxSyntaxKind::ServerNameKeyword,
                "root" => NginxSyntaxKind::RootKeyword,
                "index" => NginxSyntaxKind::IndexKeyword,
                "proxy_pass" => NginxSyntaxKind::ProxyPassKeyword,
                _ => NginxSyntaxKind::Identifier,
            };

            state.add_token(token_kind, start_pos, end_pos);
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => NginxSyntaxKind::LeftBrace,
                '}' => NginxSyntaxKind::RightBrace,
                ';' => NginxSyntaxKind::Semicolon,
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

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let start_pos = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_url(state) {
                continue;
            }

            if self.lex_path(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            state.advance_if_dead_lock(start_pos);
            if state.get_position() > start_pos {
                state.add_token(NginxSyntaxKind::Error, start_pos, state.get_position());
            }
        }
        Ok(())
    }
}

impl<'config> Lexer<NginxLanguage> for NginxLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<NginxLanguage>) -> LexOutput<NginxLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}
