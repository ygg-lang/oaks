use crate::{kind::JavadocSyntaxKind, language::JavadocLanguage};

use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, JavadocLanguage>;

/// Javadoc 词法分析
#[derive(Clone)]
pub struct JavadocLexer<'config> {
    _config: &'config JavadocLanguage,
}

impl<'config> JavadocLexer<'config> {
    /// 创建新的 Javadoc lexer
    pub fn new(config: &'config JavadocLanguage) -> Self {
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
            state.add_token(JavadocSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(JavadocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JavadocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 Javadoc 注释开始
    fn lex_comment_start<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);
                if let Some('*') = state.peek() {
                    state.advance(1);
                    state.add_token(JavadocSyntaxKind::CommentStart, start_pos, state.get_position());
                    true
                }
                else {
                    // 回退到开始位置
                    state.set_position(start_pos);
                    false
                }
            }
            else {
                // 回退到开始位置
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理 Javadoc 注释结束
    fn lex_comment_end<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);
                state.add_token(JavadocSyntaxKind::CommentEnd, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位置
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理 Javadoc 标签
    fn lex_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('@') = state.peek() {
            state.advance(1);
            let mut text = String::new();

            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    text.push(ch);
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 检查是否为已知 Javadoc 标签
            let kind = match text.as_str() {
                "param" => JavadocSyntaxKind::ParamTag,
                "return" => JavadocSyntaxKind::ReturnTag,
                "throws" => JavadocSyntaxKind::ThrowsTag,
                "exception" => JavadocSyntaxKind::ExceptionTag,
                "see" => JavadocSyntaxKind::SeeTag,
                "since" => JavadocSyntaxKind::SinceTag,
                "version" => JavadocSyntaxKind::VersionTag,
                "author" => JavadocSyntaxKind::AuthorTag,
                "deprecated" => JavadocSyntaxKind::DeprecatedTag,
                "link" => JavadocSyntaxKind::LinkTag,
                "linkplain" => JavadocSyntaxKind::LinkPlainTag,
                "code" => JavadocSyntaxKind::CodeTag,
                "literal" => JavadocSyntaxKind::LiteralTag,
                "value" => JavadocSyntaxKind::ValueTag,
                "inheritDoc" => JavadocSyntaxKind::InheritDocTag,
                _ => JavadocSyntaxKind::Tag,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 HTML 标签
    fn lex_html_tag<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            state.advance(1);
            let mut is_closing = false;

            // 检查是否为闭合标签
            if let Some('/') = state.peek() {
                is_closing = true;
                state.advance(1);
            }

            // 读取标签
            let mut tag_name = String::new();
            while let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch.is_ascii_digit() || ch == '-' {
                    tag_name.push(ch);
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 跳过
            while let Some(ch) = state.peek() {
                if ch == '>' {
                    state.advance(1);
                    break;
                }
                else if ch == '<' {
                    // 未闭合的标签
                    state.set_position(start_pos);
                    return false;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            let kind = if is_closing {
                JavadocSyntaxKind::HtmlEndTag
            }
            else {
                match tag_name.as_str() {
                    "p" => JavadocSyntaxKind::HtmlPTag,
                    "br" => JavadocSyntaxKind::HtmlBrTag,
                    "code" => JavadocSyntaxKind::HtmlCodeTag,
                    "pre" => JavadocSyntaxKind::HtmlPreTag,
                    "b" => JavadocSyntaxKind::HtmlBTag,
                    "i" => JavadocSyntaxKind::HtmlITag,
                    "em" => JavadocSyntaxKind::HtmlEmTag,
                    "strong" => JavadocSyntaxKind::HtmlStrongTag,
                    "ul" => JavadocSyntaxKind::HtmlUlTag,
                    "ol" => JavadocSyntaxKind::HtmlOlTag,
                    "li" => JavadocSyntaxKind::HtmlLiTag,
                    _ => JavadocSyntaxKind::HtmlTag,
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理文本内容
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == '@' || ch == '<' || ch == '*' || ch == '/' || ch == '\n' || ch == '\r' || ch == ' ' || ch == '\t' {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(JavadocSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理星号（注释行开始）
    fn lex_asterisk<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);
            state.add_token(JavadocSyntaxKind::Asterisk, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<JavadocLanguage> for JavadocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<JavadocLanguage>) -> LexOutput<JavadocLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JavadocLexer<'config> {
    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment_start(state) {
                continue;
            }

            if self.lex_comment_end(state) {
                continue;
            }

            if self.lex_tag(state) {
                continue;
            }

            if self.lex_html_tag(state) {
                continue;
            }

            if self.lex_asterisk(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // 如果所有规则都不匹配，检查是否到达文件末尾
            if let Some(ch) = state.peek() {
                // 跳过当前字符并标记为错误
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JavadocSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }
}
