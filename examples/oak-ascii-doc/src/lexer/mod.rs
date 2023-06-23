#![doc = include_str!("readme.md")]
pub mod token_type;

pub use token_type::AsciiDocTokenType;

use crate::language::AsciiDocLanguage;
use oak_core::{
    OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, AsciiDocLanguage>;

static ASCIIDOC_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static ASCIIDOC_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "", block_end: "", nested_blocks: false });

#[derive(Clone)]
pub struct AsciiDocLexer<'config> {
    _config: &'config AsciiDocLanguage,
}

impl<'config> Lexer<AsciiDocLanguage> for AsciiDocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<AsciiDocLanguage>) -> LexOutput<AsciiDocLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> AsciiDocLexer<'config> {
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要词法分析逻辑
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ASCIIDOC_WHITESPACE.scan(state, AsciiDocTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        ASCIIDOC_COMMENT.scan(state, AsciiDocTokenType::Comment, AsciiDocTokenType::Comment)
    }

    /// 处理标题 (= Title, == Subtitle, etc.)
    fn lex_header<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.starts_with("======") {
            state.advance(6);
            state.add_token(AsciiDocTokenType::Header6, start, state.get_position());
            return true;
        }
        if state.starts_with("=====") {
            state.advance(5);
            state.add_token(AsciiDocTokenType::Header5, start, state.get_position());
            return true;
        }
        if state.starts_with("====") {
            state.advance(4);
            state.add_token(AsciiDocTokenType::Header4, start, state.get_position());
            return true;
        }
        if state.starts_with("===") {
            state.advance(3);
            state.add_token(AsciiDocTokenType::Header3, start, state.get_position());
            return true;
        }
        if state.starts_with("==") {
            state.advance(2);
            state.add_token(AsciiDocTokenType::Header2, start, state.get_position());
            return true;
        }
        if state.starts_with("=") {
            state.advance(1);
            state.add_token(AsciiDocTokenType::Header1, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理粗体 **text**
    fn lex_bold<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.starts_with("**") {
            state.advance(2);
            state.add_token(AsciiDocTokenType::BoldMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理斜体 *text*
    fn lex_italic<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '*' {
                state.advance(1);
                state.add_token(AsciiDocTokenType::ItalicMarker, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理等宽字体 `text`
    fn lex_monospace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '`' {
                state.advance(1);
                state.add_token(AsciiDocTokenType::MonospaceMarker, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理代码块 ----
    fn lex_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("----") {
            state.advance(4);
            state.add_token(AsciiDocTokenType::CodeBlockMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理链接 link:url[text]
    fn lex_link<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        if rest.starts_with("link:") {
            state.advance(5);
            state.add_token(AsciiDocTokenType::LinkMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理列表项 *, -, +
    fn lex_list_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            match ch {
                '*' | '-' | '+' => {
                    state.advance(1);
                    state.add_token(AsciiDocTokenType::ListMarker, start, state.get_position());
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '[' => AsciiDocTokenType::LeftBracket,
                ']' => AsciiDocTokenType::RightBracket,
                '(' => AsciiDocTokenType::LeftParen,
                ')' => AsciiDocTokenType::RightParen,
                ':' => AsciiDocTokenType::Colon,
                ',' => AsciiDocTokenType::Comma,
                '.' => AsciiDocTokenType::Dot,
                '\n' => AsciiDocTokenType::Newline,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    /// 处理普通文本
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch.is_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == ' ' { state.advance(ch.len_utf8()) } else { break }
                }

                state.add_token(AsciiDocTokenType::Text, start, state.get_position());
                return true;
            }
        }
        false
    }
}
