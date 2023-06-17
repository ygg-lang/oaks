use crate::{kind::NginxSyntaxKind, language::NginxLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, NginxLanguage>;

pub struct NginxLexer<'config> {
    config: &'config NginxLanguage,
}

impl<'config> NginxLexer<'config> {
    pub fn new(config: &'config NginxLanguage) -> Self {
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
            state.add_token(NginxSyntaxKind::Whitespace, start_pos, state.get_position());
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
    fn lex_comment(&self, state: &mut State) -> bool {
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
    fn lex_string(&self, state: &mut State) -> bool {
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
                    if state.peek().is_some() {
                        state.advance(state.peek().unwrap().len_utf8());
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
    fn lex_number(&self, state: &mut State) -> bool {
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
    fn lex_path(&self, state: &mut State) -> bool {
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
    fn lex_url(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 检查是否以 http:// https:// 开
        let text = source.get_text_at(state.get_position()).unwrap_or("");
        if text.starts_with("http://") || text.starts_with("https://") {
            let scheme_len = if text.starts_with("https://") { 8 } else { 7 };
            state.advance(scheme_len);

            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric()
                    || ch == '.'
                    || ch == '/'
                    || ch == ':'
                    || ch == '-'
                    || ch == '_'
                    || ch == '?'
                    || ch == '&'
                    || ch == '='
                {
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
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
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
            let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
            let token_kind = match text {
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

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
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
}

impl<'config> Lexer<NginxLanguage> for NginxLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<NginxSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_url(&mut state, source) {
                continue;
            }

            if self.lex_path(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NginxSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(NginxSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
