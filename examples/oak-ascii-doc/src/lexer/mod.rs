use crate::{kind::AsciiDocSyntaxKind, language::AsciiDocLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, AsciiDocLanguage>;

static ASCIIDOC_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ASCIIDOC_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });

#[derive(Clone)]
pub struct AsciiDocLexer<'config> {
    config: &'config AsciiDocLanguage,
}

impl<'config> Lexer<AsciiDocLanguage> for AsciiDocLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<AsciiDocLanguage>,
    ) -> LexOutput<AsciiDocLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> AsciiDocLexer<'config> {
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
        Self { config }
    }

    /// 主要词法分析逻辑
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_header(state) {
                continue;
            }

            if self.lex_bold(state) {
                continue;
            }

            if self.lex_italic(state) {
                continue;
            }

            if self.lex_monospace(state) {
                continue;
            }

            if self.lex_code_block(state) {
                continue;
            }

            if self.lex_link(state) {
                continue;
            }

            if self.lex_list_item(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(AsciiDocSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match ASCIIDOC_WHITESPACE.scan(state.rest(), state.get_position(), AsciiDocSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match ASCIIDOC_COMMENT.scan(state.rest(), state.get_position(), AsciiDocSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    /// 处理标题 (= Title, == Subtitle, etc.)
    fn lex_header<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("======") {
            state.advance(6);
            state.add_token(AsciiDocSyntaxKind::Header6, start, state.get_position());
            return true;
        }
        if rest.starts_with("=====") {
            state.advance(5);
            state.add_token(AsciiDocSyntaxKind::Header5, start, state.get_position());
            return true;
        }
        if rest.starts_with("====") {
            state.advance(4);
            state.add_token(AsciiDocSyntaxKind::Header4, start, state.get_position());
            return true;
        }
        if rest.starts_with("===") {
            state.advance(3);
            state.add_token(AsciiDocSyntaxKind::Header3, start, state.get_position());
            return true;
        }
        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(AsciiDocSyntaxKind::Header2, start, state.get_position());
            return true;
        }
        if rest.starts_with("=") {
            state.advance(1);
            state.add_token(AsciiDocSyntaxKind::Header1, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理粗体 **text**
    fn lex_bold<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("**") {
            state.advance(2);
            state.add_token(AsciiDocSyntaxKind::BoldMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理斜体 *text*
    fn lex_italic<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch == '*' {
                state.advance(1);
                state.add_token(AsciiDocSyntaxKind::ItalicMarker, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理等宽字体 `text`
    fn lex_monospace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch == '`' {
                state.advance(1);
                state.add_token(AsciiDocSyntaxKind::MonospaceMarker, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理代码块 ----
    fn lex_code_block<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("----") {
            state.advance(4);
            state.add_token(AsciiDocSyntaxKind::CodeBlockMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理链接 link:url[text]
    fn lex_link<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("link:") {
            state.advance(5);
            state.add_token(AsciiDocSyntaxKind::LinkMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理列表项 *, -, +
    fn lex_list_item<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            match ch {
                '*' | '-' | '+' => {
                    state.advance(1);
                    state.add_token(AsciiDocSyntaxKind::ListMarker, start, state.get_position());
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    /// 处理分隔符
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '[' => AsciiDocSyntaxKind::LeftBracket,
                ']' => AsciiDocSyntaxKind::RightBracket,
                '(' => AsciiDocSyntaxKind::LeftParen,
                ')' => AsciiDocSyntaxKind::RightParen,
                ':' => AsciiDocSyntaxKind::Colon,
                ',' => AsciiDocSyntaxKind::Comma,
                '.' => AsciiDocSyntaxKind::Dot,
                '\n' => AsciiDocSyntaxKind::Newline,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理普通文本
    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == ' ' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(AsciiDocSyntaxKind::Text, start, state.get_position());
                return true;
            }
        }
        false
    }
}
