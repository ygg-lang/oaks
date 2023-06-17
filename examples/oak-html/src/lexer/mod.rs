use crate::{kind::HtmlSyntaxKind, language::HtmlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, HtmlLanguage>;

pub struct HtmlLexer<'config> {
    config: &'config HtmlLanguage,
}

impl<'config> HtmlLexer<'config> {
    pub fn new(config: &'config HtmlLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(HtmlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(HtmlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释 <!-- ... -->
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = source.get_char_at(start_pos + 1) {
                if let Some('-') = source.get_char_at(start_pos + 2) {
                    if let Some('-') = source.get_char_at(start_pos + 3) {
                        // 找到注释开<!--
                        state.advance(4);

                        // 查找注释结束 -->
                        while let Some(ch) = state.peek() {
                            if ch == '-' {
                                if let Some('-') = source.get_char_at(state.get_position() + 1) {
                                    if let Some('>') = source.get_char_at(state.get_position() + 2) {
                                        state.advance(3);
                                        state.add_token(HtmlSyntaxKind::Comment, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            state.advance(ch.len_utf8());
                        }

                        // 未闭合的注释
                        state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 处理 DOCTYPE 声明
    fn lex_doctype(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = source.get_char_at(start_pos + 1) {
                // 检查是否是 DOCTYPE
                let doctype_str = "DOCTYPE";
                let mut matches = true;
                for (i, expected_ch) in doctype_str.chars().enumerate() {
                    if let Some(actual_ch) = source.get_char_at(start_pos + 2 + i) {
                        if actual_ch.to_ascii_uppercase() != expected_ch {
                            matches = false;
                            break;
                        }
                    }
                    else {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    state.advance(2 + doctype_str.len());

                    // 跳过>
                    while let Some(ch) = state.peek() {
                        if ch == '>' {
                            state.advance(1);
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }

                    state.add_token(HtmlSyntaxKind::Doctype, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理 CDATA
    fn lex_cdata(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = source.get_char_at(start_pos + 1) {
                if let Some('[') = source.get_char_at(start_pos + 2) {
                    // 检查是否是 CDATA
                    let cdata_str = "CDATA[";
                    let mut matches = true;
                    for (i, expected_ch) in cdata_str.chars().enumerate() {
                        if let Some(actual_ch) = source.get_char_at(start_pos + 3 + i) {
                            if actual_ch != expected_ch {
                                matches = false;
                                break;
                            }
                        }
                        else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        state.advance(3 + cdata_str.len());

                        // 查找 CDATA 结束 ]]>
                        while let Some(ch) = state.peek() {
                            if ch == ']' {
                                if let Some(']') = source.get_char_at(state.get_position() + 1) {
                                    if let Some('>') = source.get_char_at(state.get_position() + 2) {
                                        state.advance(3);
                                        state.add_token(HtmlSyntaxKind::CData, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            state.advance(ch.len_utf8());
                        }

                        // 未闭合的 CDATA
                        state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 处理处理指令 <?xml ... ?>
    fn lex_processing_instruction(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('?') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                // 查找处理指令结束 ?>
                while let Some(ch) = state.peek() {
                    if ch == '?' {
                        if let Some('>') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            state.add_token(HtmlSyntaxKind::ProcessingInstruction, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                // 未闭合的处理指令
                state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// 处理标签
    fn lex_tag_name(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(name_ch) = state.peek() {
                    if name_ch.is_ascii_alphanumeric() || name_ch == '-' || name_ch == '_' || name_ch == ':' {
                        state.advance(name_ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(HtmlSyntaxKind::TagName, start_pos, state.get_position());
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

    /// 处理

    fn lex_attribute_value(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(HtmlSyntaxKind::AttributeValue, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' {
                        break; // 属性值不能跨
                    }
                    state.advance(ch.len_utf8());
                }

                // 未闭合的

                state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
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

    /// 处理实体引用
    fn lex_entity_ref(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('&') = state.peek() {
            state.advance(1);

            if let Some('#') = state.peek() {
                // 字符引用 &#123; &#x1A;
                state.advance(1);

                if let Some('x') = state.peek() {
                    // 十六进制字符引用
                    state.advance(1);
                    while let Some(hex_ch) = state.peek() {
                        if hex_ch.is_ascii_hexdigit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 十进制字符引
                    while let Some(digit_ch) = state.peek() {
                        if digit_ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                if let Some(';') = state.peek() {
                    state.advance(1);
                    state.add_token(HtmlSyntaxKind::CharRef, start_pos, state.get_position());
                }
                else {
                    state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                }
                true
            }
            else {
                // 实体引用 &amp; &lt;
                while let Some(entity_ch) = state.peek() {
                    if entity_ch.is_ascii_alphanumeric() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                if let Some(';') = state.peek() {
                    state.advance(1);
                    state.add_token(HtmlSyntaxKind::EntityRef, start_pos, state.get_position());
                }
                else {
                    state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
                }
                true
            }
        }
        else {
            false
        }
    }

    /// 处理文本内容
    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == '<' || ch == '&' || ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(HtmlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标签和操作符
    fn lex_tag_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '<' => {
                    if let Some('/') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        HtmlSyntaxKind::TagSlashOpen
                    }
                    else {
                        state.advance(1);
                        HtmlSyntaxKind::TagOpen
                    }
                }
                '>' => {
                    state.advance(1);
                    HtmlSyntaxKind::TagClose
                }
                '/' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        HtmlSyntaxKind::TagSelfClose
                    }
                    else {
                        return false;
                    }
                }
                '=' => {
                    state.advance(1);
                    HtmlSyntaxKind::Equal
                }
                '"' | '\'' => {
                    state.advance(1);
                    HtmlSyntaxKind::Quote
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<HtmlLanguage> for HtmlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<HtmlSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_doctype(&mut state, source) {
                continue;
            }

            if self.lex_cdata(&mut state, source) {
                continue;
            }

            if self.lex_processing_instruction(&mut state, source) {
                continue;
            }

            if self.lex_entity_ref(&mut state) {
                continue;
            }

            if self.lex_attribute_value(&mut state) {
                continue;
            }

            if self.lex_tag_name(&mut state) {
                continue;
            }

            if self.lex_tag_operator(&mut state, source) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(HtmlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(HtmlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
