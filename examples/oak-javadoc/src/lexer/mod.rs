use crate::{kind::JavadocSyntaxKind, language::JavadocLanguage};
use alloc::string::String;
use oak_core::{
    Lexer, SourceText,
    lexer::{LexOutput, LexerState},
};

type State<'input> = LexerState<'input, JavadocLanguage>;

/// Javadoc 词法分析
pub struct JavadocLexer<'config> {
    _config: &'config JavadocLanguage,
}

impl<'config> JavadocLexer<'config> {
    /// 创建新的 Javadoc lexer
    pub fn new(config: &'config JavadocLanguage) -> Self {
        Self { _config: config }
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
            state.add_token(JavadocSyntaxKind::Whitespace, start_pos, state.get_position());
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

    /// 处理 Javadoc 注释开
    fn lex_comment_start(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                if let Some('*') = source.get_char_at(start_pos + 2) {
                    state.advance(3);
                    state.add_token(JavadocSyntaxKind::CommentStart, start_pos, state.get_position());
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

    /// 处理 Javadoc 注释结束
    fn lex_comment_end(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(JavadocSyntaxKind::CommentEnd, start_pos, state.get_position());
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

    /// 处理 Javadoc 标签
    fn lex_tag(&self, state: &mut State) -> bool {
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

            // 检查是否为已知Javadoc 标签
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
    fn lex_html_tag(&self, state: &mut State) -> bool {
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
    fn lex_text(&self, state: &mut State) -> bool {
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
    fn lex_asterisk(&self, state: &mut State) -> bool {
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
    fn lex(&self, source: &SourceText) -> LexOutput<JavadocSyntaxKind> {
        let mut state = State::new(source);

        loop {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment_start(&mut state, source) {
                continue;
            }

            if self.lex_comment_end(&mut state, source) {
                continue;
            }

            if self.lex_tag(&mut state) {
                continue;
            }

            if self.lex_html_tag(&mut state) {
                continue;
            }

            if self.lex_asterisk(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，检查是否到达文件末尾
            if let Some(ch) = state.peek() {
                // 跳过当前字符并标记为错误
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(JavadocSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 到达文件末尾，退出循环
                break;
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(JavadocSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
