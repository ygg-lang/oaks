use crate::{kind::CsvSyntaxKind, language::CsvLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, SourceText, lexer::LexOutput, source::Source};

type State<'input> = LexerState<&'input SourceText, CsvLanguage>;

pub struct CsvLexer {
    field_separator: char,
    quote_char: char,
}

impl CsvLexer {
    pub fn new(_config: CsvLanguage) -> Self {
        Self { field_separator: ',', quote_char: '"' }
    }

    pub fn with_separator(mut self, separator: char) -> Self {
        self.field_separator = separator;
        self
    }

    pub fn with_quote_char(mut self, quote: char) -> Self {
        self.quote_char = quote;
        self
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();
        let mut found_whitespace = false;

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                found_whitespace = true;
            }
            else {
                break;
            }
        }

        if found_whitespace {
            state.add_token(CsvSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '\r' {
                state.advance(1);
                // 检查是否是 CRLF
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(CsvSyntaxKind::Newline, start_pos, state.get_position());
                true
            }
            else if ch == '\n' {
                state.advance(1);
                state.add_token(CsvSyntaxKind::Newline, start_pos, state.get_position());
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

    /// 处理带引号的字段
    fn lex_quoted_field(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == self.quote_char {
                state.advance(ch.len_utf8()); // 跳过开始引
                while let Some(ch) = state.peek() {
                    if ch == self.quote_char {
                        state.advance(ch.len_utf8());
                        // 检查是否是转义引号（双引号
                        if state.peek() == Some(self.quote_char) {
                            state.advance(self.quote_char.len_utf8()); // 跳过转义引号
                        }
                        else {
                            // 结束引号
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(CsvSyntaxKind::QuotedField, start_pos, state.get_position());
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

    /// 处理不带引号的字
    fn lex_unquoted_field(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();
        let mut found_content = false;

        while let Some(ch) = state.peek() {
            if ch == self.field_separator || ch == '\n' || ch == '\r' {
                break;
            }
            state.advance(ch.len_utf8());
            found_content = true;
        }

        if found_content {
            state.add_token(CsvSyntaxKind::UnquotedField, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字段分隔符（逗号
    fn lex_comma(&self, state: &mut State<'_>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == self.field_separator {
                state.advance(ch.len_utf8());
                state.add_token(CsvSyntaxKind::Comma, start_pos, state.get_position());
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
}

impl Lexer<CsvLanguage> for CsvLexer {
    fn lex(&self, source: impl Source) -> LexOutput<CsvLanguage> {
        let source_text = SourceText::new(source.get_text_in((0..source.length()).into()));
        let mut state = LexerState::new(&source_text);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comma(&mut state) {
                continue;
            }

            if self.lex_quoted_field(&mut state) {
                continue;
            }

            if self.lex_unquoted_field(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CsvSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(CsvSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }

    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<CsvLanguage>,
    ) -> LexOutput<CsvLanguage> {
        self.lex(source)
    }
}
