#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::MarkdownLanguage, lexer::token_type::MarkdownTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, MarkdownLanguage>;

/// Lexer for Markdown language.
#[derive(Clone, Debug)]
pub struct MarkdownLexer<'config> {
    _config: &'config MarkdownLanguage,
}

impl<'config> MarkdownLexer<'config> {
    /// Creates a new MarkdownLexer with the given configuration.
    pub fn new(config: &'config MarkdownLanguage) -> Self {
        Self { _config: config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        if self._config.allow_indented_code_blocks && self.lex_indented_code_block(state) {
                            continue;
                        }
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '$' if self._config.allow_math => {
                        if self.lex_math(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '^' if self._config.allow_sub_superscript || self._config.allow_footnotes => {
                        if self._config.allow_footnotes && self.lex_footnote(state) {
                            continue;
                        }
                        if self._config.allow_sub_superscript && self.lex_sub_superscript(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '#' => {
                        if self._config.allow_headings && self.lex_heading(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '`' => {
                        if self._config.allow_fenced_code_blocks && self.lex_code_block(state) {
                            continue;
                        }
                        if self.lex_inline_code(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '~' => {
                        if self.lex_code_block(state) {
                            continue;
                        }
                        if self._config.allow_strikethrough && self.lex_strikethrough(state) {
                            continue;
                        }
                        if self._config.allow_sub_superscript && self.lex_sub_superscript(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '*' | '_' => {
                        if self._config.allow_horizontal_rules && self.lex_horizontal_rule(state) {
                            continue;
                        }
                        if self._config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        if self.lex_emphasis(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '-' => {
                        if self._config.allow_front_matter && self.lex_front_matter(state) {
                            continue;
                        }
                        if self._config.allow_horizontal_rules && self.lex_horizontal_rule(state) {
                            continue;
                        }
                        if self._config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '+' => {
                        if self._config.allow_lists && self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '!' => {
                        if self.lex_link_or_image(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '[' => {
                        if self._config.allow_task_lists && self.lex_task_marker(state) {
                            continue;
                        }
                        if self.lex_link_or_image(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '>' => {
                        if self._config.allow_blockquotes && self.lex_blockquote(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    '|' if self._config.allow_tables => {
                        self.lex_special_char(state);
                    }
                    '0'..='9' => {
                        if self.lex_list_marker(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '<' => {
                        if self._config.allow_html && self.lex_html_tag(state) {
                            continue;
                        }
                        if self._config.allow_xml && self.lex_xml_tag(state) {
                            continue;
                        }
                        self.lex_special_char(state);
                    }
                    ']' | '(' | ')' | '|' | '.' | ':' | '\\' => {
                        self.lex_special_char(state);
                    }
                    _ => {
                        if self.lex_text(state) {
                            continue;
                        }
                        // 如果所有规则都不匹配，跳过当前字符并标记为错误
                        let start_pos = state.get_position();
                        state.advance(ch.len_utf8());
                        state.add_token(MarkdownTokenType::Error, start_pos, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(MarkdownTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(MarkdownTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(MarkdownTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标题
    fn lex_heading<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        if let Some('#') = state.peek() {
            let mut level = 0;
            let mut pos = start_pos;

            // 计算 # 的数量
            while let Some('#') = state.source().get_char_at(pos) {
                level += 1;
                pos += 1;
                if level > 6 {
                    return false; // 超过6级标题，不是有效标题
                }
            }

            // 检查 # 后面是否有空白
            if let Some(ch) = state.source().get_char_at(pos) {
                if ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r' {
                    return false;
                }
            }

            state.advance(level);

            let heading_kind = match level {
                1 => MarkdownTokenType::Heading1,
                2 => MarkdownTokenType::Heading2,
                3 => MarkdownTokenType::Heading3,
                4 => MarkdownTokenType::Heading4,
                5 => MarkdownTokenType::Heading5,
                6 => MarkdownTokenType::Heading6,
                _ => return false,
            };

            state.add_token(heading_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理内联代码
    fn lex_inline_code<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 内联代码不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(MarkdownTokenType::InlineCode, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理代码
    fn lex_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // 检查是否是 ``` ~~~
        let fence_char = if let Some('`') = state.peek() {
            '`'
        }
        else if let Some('~') = state.peek() {
            '~'
        }
        else {
            return false;
        };

        let mut fence_count = 0;
        let mut pos = start_pos;

        // 计算围栏字符数量
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == fence_char {
                fence_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if fence_count < 3 {
            return false; // 至少需要3个围栏字符
        }

        state.advance(fence_count);
        state.add_token(MarkdownTokenType::CodeFence, start_pos, state.get_position());

        // 处理语言标识
        let lang_start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            else if ch != ' ' && ch != '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > lang_start {
            state.add_token(MarkdownTokenType::CodeLanguage, lang_start, state.get_position());
        }

        true
    }

    /// 处理强调和加
    fn lex_emphasis<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let marker_char = if let Some('*') = state.peek() {
            '*'
        }
        else if let Some('_') = state.peek() {
            '_'
        }
        else {
            return false;
        };

        let mut marker_count = 0;
        let mut pos = start_pos;

        // 计算标记字符数量
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == marker_char {
                marker_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if marker_count == 0 {
            return false;
        }

        state.advance(marker_count);

        let token_kind = if marker_count >= 2 { MarkdownTokenType::Strong } else { MarkdownTokenType::Emphasis };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// 处理删除
    fn lex_strikethrough<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('~') = state.peek() {
            if let Some('~') = state.source().get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(MarkdownTokenType::Strikethrough, start_pos, state.get_position());
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

    /// 处理链接和图
    fn lex_link_or_image<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是图片 ![
        let is_image = if let Some('!') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some('[') = state.peek() {
            state.advance(1);

            let token_kind = if is_image { MarkdownTokenType::Image } else { MarkdownTokenType::Link };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            if is_image {
                // 回退感叹
                state.set_position(start_pos);
            }
            false
        }
    }

    /// 处理列表标记
    fn lex_list_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false; // 前面有非空白字符
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '-' | '*' | '+' => {
                    // 无序列表
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                    false
                }
                '0'..='9' => {
                    // 有序列表
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() { state.advance(1) } else { break }
                    }

                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                state.add_token(MarkdownTokenType::ListMarker, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }

                    state.set_position(start_pos);
                    false
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理任务列表
    fn lex_task_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('[') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == 'x' || ch == 'X' {
                    state.advance(1);
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(MarkdownTokenType::TaskMarker, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// 处理 HTML 标签或注释
    fn lex_html_tag<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        self.lex_any_tag(state, MarkdownTokenType::HtmlTag, MarkdownTokenType::HtmlComment)
    }

    /// 处理 XML 标签或注释
    fn lex_xml_tag<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        self.lex_any_tag(state, MarkdownTokenType::XmlTag, MarkdownTokenType::XmlComment)
    }

    /// 通用的标签处理逻辑
    fn lex_any_tag<S: Source + ?Sized>(&self, state: &mut State<S>, tag_kind: MarkdownTokenType, comment_kind: MarkdownTokenType) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            state.advance(1);

            // 检查是否是注释 <!-- -->
            if let Some('!') = state.peek() {
                if state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('-') {
                    state.advance(3);
                    let mut found_end = false;
                    while let Some(ch) = state.peek() {
                        if ch == '-' && state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('>') {
                            state.advance(3);
                            found_end = true;
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    if found_end {
                        state.add_token(comment_kind, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            // 正常的标签解析
            let mut found_end = false;
            let mut in_string = None; // 记录是否在引号内

            while let Some(ch) = state.peek() {
                if let Some(quote) = in_string {
                    if ch == quote {
                        in_string = None;
                    }
                }
                else {
                    if ch == '>' {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '"' || ch == '\'' {
                        in_string = Some(ch);
                    }
                }
                state.advance(ch.len_utf8());
            }

            if found_end {
                state.add_token(tag_kind, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理引用
    fn lex_blockquote<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some('>') = state.peek() {
            state.advance(1);
            state.add_token(MarkdownTokenType::BlockquoteMarker, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理水平分隔
    fn lex_horizontal_rule<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.source().get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            if ch == '-' || ch == '*' || ch == '_' {
                let rule_char = ch;
                let mut count = 0;
                let mut pos = start_pos;

                // 计算连续的分隔符数量
                while let Some(current_ch) = state.source().get_char_at(pos) {
                    if current_ch == rule_char {
                        count += 1;
                        pos += 1
                    }
                    else if current_ch == ' ' || current_ch == '\t' {
                        pos += 1; // 允许空格
                    }
                    else {
                        break;
                    }
                }

                if count >= 3 {
                    // 检查到行尾
                    while let Some(current_ch) = state.source().get_char_at(pos) {
                        if current_ch == '\n' || current_ch == '\r' {
                            break;
                        }
                        else if current_ch == ' ' || current_ch == '\t' {
                            pos += 1
                        }
                        else {
                            return false; // 行尾有其他字
                        }
                    }

                    state.set_position(pos);
                    state.add_token(MarkdownTokenType::HorizontalRule, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理数学公式
    fn lex_math<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);
            let mut is_block = false;

            if let Some('$') = state.peek() {
                state.advance(1);
                is_block = true;
            }

            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '$' {
                    if is_block {
                        if let Some('$') = state.source().get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            found_end = true;
                            break;
                        }
                    }
                    else {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                }
                state.advance(ch.len_utf8())
            }

            if found_end {
                let kind = if is_block { MarkdownTokenType::MathBlock } else { MarkdownTokenType::MathInline };
                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理前置数据
    fn lex_front_matter<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 必须在文件开头
        if start_pos != 0 {
            return false;
        }

        if state.peek() == Some('-') && state.source().get_char_at(1) == Some('-') && state.source().get_char_at(2) == Some('-') {
            state.advance(3);
            // 寻找结束标记 ---
            let mut found_end = false;
            while state.not_at_end() {
                if state.peek() == Some('\n') || state.peek() == Some('\r') {
                    state.advance(1);
                    if state.peek() == Some('\n') {
                        state.advance(1)
                    }
                    if state.peek() == Some('-') && state.source().get_char_at(state.get_position() + 1) == Some('-') && state.source().get_char_at(state.get_position() + 2) == Some('-') {
                        state.advance(3);
                        found_end = true;
                        break;
                    }
                }
                else {
                    state.advance(1)
                }
            }

            if found_end {
                state.add_token(MarkdownTokenType::FrontMatter, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理脚注
    fn lex_footnote<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('^') = state.peek() {
            // 检查是否是 [^...
            let check_pos = start_pos;
            if check_pos > 0 && state.source().get_char_at(check_pos - 1) == Some('[') {
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        // 检查是否是定义 [^...]:
                        if state.peek() == Some(':') {
                            state.advance(1);
                            state.add_token(MarkdownTokenType::FootnoteDefinition, start_pos - 1, state.get_position())
                        }
                        else {
                            state.add_token(MarkdownTokenType::FootnoteReference, start_pos - 1, state.get_position())
                        }
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// 处理上标和下标
    fn lex_sub_superscript<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let marker = ch;
            if marker == '^' || marker == '~' {
                state.advance(1);
                let mut found_end = false;
                while let Some(next_ch) = state.peek() {
                    if next_ch == marker {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if next_ch == ' ' || next_ch == '\t' || next_ch == '\n' || next_ch == '\r' {
                        break;
                    }
                    state.advance(next_ch.len_utf8())
                }

                if found_end {
                    let kind = if marker == '^' { MarkdownTokenType::Superscript } else { MarkdownTokenType::Subscript };
                    state.add_token(kind, start_pos, state.get_position());
                    true
                }
                else {
                    state.set_position(start_pos);
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

    /// 处理缩进代码块
    fn lex_indented_code_block<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 必须在行首
        if start_pos > 0 {
            if let Some(prev_char) = state.source().get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // 检查缩进（4个空格或1个制表符）
        let mut indent_count = 0;
        let mut pos = start_pos;
        while let Some(ch) = state.source().get_char_at(pos) {
            if ch == ' ' {
                indent_count += 1;
                pos += 1;
                if indent_count >= 4 {
                    break;
                }
            }
            else if ch == '\t' {
                indent_count = 4;
                pos += 1;
                break;
            }
            else {
                break;
            }
        }

        if indent_count >= 4 {
            state.set_position(pos);
            state.add_token(MarkdownTokenType::CodeBlock, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理特殊字符
    fn lex_special_char<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => MarkdownTokenType::LBracket,
                ']' => MarkdownTokenType::RBracket,
                '(' => MarkdownTokenType::LParen,
                ')' => MarkdownTokenType::RParen,
                '<' => MarkdownTokenType::Less,
                '>' => MarkdownTokenType::Greater,
                '*' => MarkdownTokenType::Asterisk,
                '_' => MarkdownTokenType::Underscore,
                '`' => MarkdownTokenType::Backtick,
                '~' => MarkdownTokenType::Tilde,
                '#' => MarkdownTokenType::Hash,
                '|' => MarkdownTokenType::Pipe,
                '-' => MarkdownTokenType::Dash,
                '+' => MarkdownTokenType::Plus,
                '.' => MarkdownTokenType::Dot,
                ':' => MarkdownTokenType::Colon,
                '!' => MarkdownTokenType::Exclamation,
                '\\' => MarkdownTokenType::Escape,
                '$' => MarkdownTokenType::Dollar,
                '^' => MarkdownTokenType::Caret,
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

    /// 处理普通文
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停止
            match ch {
                ' ' | '\t' | '\n' | '\r' | '#' | '*' | '_' | '`' | '~' | '[' | ']' | '(' | ')' | '<' | '>' | '|' | '-' | '+' | '.' | ':' | '!' | '\\' | '$' | '^' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(MarkdownTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<MarkdownLanguage> for MarkdownLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<MarkdownLanguage>) -> LexOutput<MarkdownLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> MarkdownLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<MarkdownLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
