use crate::{kind::MsilSyntaxKind, language::MsilLanguage};
use oak_core::{
    IncrementalCache, Lexer,
    lexer::{LexOutput, LexerState},
    source::Source,
};

#[derive(Clone)]
pub struct MsilLexer<'config> {
    config: &'config MsilLanguage,
}

impl<'config> MsilLexer<'config> {
    pub fn new(config: &'config MsilLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
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
            state.add_token(MsilSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(MsilSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(MsilSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                // 行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(MsilSyntaxKind::CommentToken, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_identifier<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' && ch != '.' {
                return false;
            }

            // 收集标识符字符
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 检查是否是关键字
            let text = state.get_text_in((start_pos..state.get_position()).into());
            let token_kind = match text {
                ".assembly" => MsilSyntaxKind::AssemblyKeyword,
                "extern" => MsilSyntaxKind::ExternKeyword,
                ".module" => MsilSyntaxKind::ModuleKeyword,
                ".class" => MsilSyntaxKind::ClassKeyword,
                ".method" => MsilSyntaxKind::MethodKeyword,
                "public" => MsilSyntaxKind::PublicKeyword,
                "private" => MsilSyntaxKind::PrivateKeyword,
                "static" => MsilSyntaxKind::StaticKeyword,
                _ => MsilSyntaxKind::IdentifierToken,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
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

            // 处理小数点
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数点
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

            state.add_token(MsilSyntaxKind::NumberToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串
    fn lex_string<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1); // 跳过开始引号

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1); // 跳过结束引号
                    break;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(_) = state.peek() {
                        state.advance(1); // 跳过被转义的字符
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(MsilSyntaxKind::StringToken, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter<S: Source>(&self, state: &mut LexerState<S, MsilLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => MsilSyntaxKind::LeftBrace,
                '}' => MsilSyntaxKind::RightBrace,
                '(' => MsilSyntaxKind::LeftParen,
                ')' => MsilSyntaxKind::RightParen,
                '[' => MsilSyntaxKind::LeftBracket,
                ']' => MsilSyntaxKind::RightBracket,
                '.' => MsilSyntaxKind::Dot,
                ':' => MsilSyntaxKind::Colon,
                ';' => MsilSyntaxKind::Semicolon,
                ',' => MsilSyntaxKind::Comma,
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

impl<'config> Lexer<MsilLanguage> for MsilLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<MsilLanguage>,
    ) -> LexOutput<MsilLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);

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

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(MsilSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(MsilSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}
