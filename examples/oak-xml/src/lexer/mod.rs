use crate::{kind::XmlSyntaxKind, language::XmlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, XmlLanguage>;

pub struct XmlLexer<'config> {
    config: &'config XmlLanguage,
}

impl<'config> XmlLexer<'config> {
    pub fn new(config: &'config XmlLanguage) -> Self {
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
            state.add_token(XmlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(XmlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(XmlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 XML 注释 <!-- ... -->
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                if let Some('-') = state.peek_next_n(2) {
                    if let Some('-') = state.peek_next_n(3) {
                        state.advance(4); // 跳过 <!--

                        // 查找注释结束 -->
                        while state.not_at_end() {
                            if let Some('-') = state.peek() {
                                if let Some('-') = state.peek_next_n(1) {
                                    if let Some('>') = state.peek_next_n(2) {
                                        state.advance(3); // 跳过 -->
                                        state.add_token(XmlSyntaxKind::Comment, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        // 未闭合的注释
                        state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }

        false
    }

    /// 处理 CDATA 段 <![CDATA[ ... ]]>
    fn lex_cdata(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                if let Some('[') = state.peek_next_n(2) {
                    // 检CDATA 开始标

                    let cdata_start = "CDATA[";
                    let mut matches = true;
                    for (i, expected_ch) in cdata_start.chars().enumerate() {
                        if let Some(actual_ch) = state.peek_next_n(3 + i) {
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
                        state.advance(3 + cdata_start.len()); // 跳过 <![CDATA[

                        // 查找 CDATA 结束 ]]>
                        while state.not_at_end() {
                            if let Some(']') = state.peek() {
                                if let Some(']') = state.peek_next_n(1) {
                                    if let Some('>') = state.peek_next_n(2) {
                                        state.advance(3); // 跳过 ]]>
                                        state.add_token(XmlSyntaxKind::CData, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        // 未闭合的 CDATA
                        state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
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
            if let Some('?') = state.peek_next_n(1) {
                state.advance(2); // 跳过 <?

                // 查找处理指令结束 ?>
                while state.not_at_end() {
                    if let Some('?') = state.peek() {
                        if let Some('>') = state.peek_next_n(1) {
                            state.advance(2); // 跳过 ?>
                            state.add_token(XmlSyntaxKind::ProcessingInstruction, start_pos, state.get_position());
                            return true;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 未闭合的处理指令
                state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理 DOCTYPE 声明
    fn lex_doctype(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                // 检DOCTYPE 关键

                let doctype_keyword = "DOCTYPE";
                let mut matches = true;
                for (i, expected_ch) in doctype_keyword.chars().enumerate() {
                    if let Some(actual_ch) = state.peek_next_n(2 + i) {
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
                    state.advance(2 + doctype_keyword.len()); // 跳过 <!DOCTYPE

                    let mut bracket_depth = 0;
                    // 查找 DOCTYPE 结束
                    while state.not_at_end() {
                        if let Some('[') = state.peek() {
                            bracket_depth += 1;
                            state.advance(1);
                        }
                        else if let Some(']') = state.peek() {
                            bracket_depth -= 1;
                            state.advance(1);
                        }
                        else if let Some('>') = state.peek() {
                            if bracket_depth == 0 {
                                state.advance(1); // 跳过 >
                                state.add_token(XmlSyntaxKind::DoctypeDeclaration, start_pos, state.get_position());
                                return true;
                            }
                            else {
                                state.advance(1);
                            }
                        }
                        else if let Some(ch) = state.peek() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    // 未闭合的 DOCTYPE
                    state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引

                while let Some(ch) = state.peek() {
                    if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        state.add_token(XmlSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '&' {
                        // 处理实体引用
                        let entity_start = state.get_position();
                        state.advance(1);
                        while let Some(entity_ch) = state.peek() {
                            if entity_ch == ';' {
                                state.advance(1);
                                break;
                            }
                            else if entity_ch.is_ascii_alphanumeric() || entity_ch == '#' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        // XML 属性值可以跨

                        state.advance(ch.len_utf8());
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符

                state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理实体引用
    fn lex_entity_reference(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('&') = state.peek() {
            state.advance(1);

            // 检查字符引&#...;
            if let Some('#') = state.peek() {
                state.advance(1);
                let mut has_digits = false;

                // 十六进制字符引用 &#x...;
                if let Some('x') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 十进制字符引&#...;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }

                if has_digits {
                    if let Some(';') = state.peek() {
                        state.advance(1);
                        state.add_token(XmlSyntaxKind::CharacterReference, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            else {
                // 命名实体引用 &name;
                let mut has_name = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() {
                        state.advance(1);
                        has_name = true;
                    }
                    else {
                        break;
                    }
                }

                if has_name {
                    if let Some(';') = state.peek() {
                        state.advance(1);
                        state.add_token(XmlSyntaxKind::EntityReference, start_pos, state.get_position());
                        return true;
                    }
                }
            }

            // 无效的实体引

            state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理标识符（标签名、属性名等）
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == ':' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == ':' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(XmlSyntaxKind::Identifier, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号
    fn lex_punctuation(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '<' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        XmlSyntaxKind::LeftAngleSlash
                    }
                    else {
                        XmlSyntaxKind::LeftAngle
                    }
                }
                '>' => {
                    state.advance(1);
                    XmlSyntaxKind::RightAngle
                }
                '/' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        XmlSyntaxKind::SlashRightAngle
                    }
                    else {
                        return false; // 单独/ 不是有效XML token
                    }
                }
                '=' => {
                    state.advance(1);
                    XmlSyntaxKind::Equals
                }
                '"' => {
                    state.advance(1);
                    XmlSyntaxKind::Quote
                }
                '\'' => {
                    state.advance(1);
                    XmlSyntaxKind::SingleQuote
                }
                '!' => {
                    state.advance(1);
                    XmlSyntaxKind::Exclamation
                }
                '?' => {
                    state.advance(1);
                    XmlSyntaxKind::Question
                }
                '&' => {
                    state.advance(1);
                    XmlSyntaxKind::Ampersand
                }
                ';' => {
                    state.advance(1);
                    XmlSyntaxKind::Semicolon
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

    /// 处理普通文本
    fn lex_text(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '<' | '>' | '=' | '"' | '\'' | '!' | '?' | '&' | ';' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(XmlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<XmlLanguage> for XmlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<XmlSyntaxKind> {
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

            if self.lex_cdata(&mut state, source) {
                continue;
            }

            if self.lex_processing_instruction(&mut state, source) {
                continue;
            }

            if self.lex_doctype(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state, source) {
                continue;
            }

            if self.lex_entity_reference(&mut state, source) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state, source) {
                continue;
            }

            if self.lex_text(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(XmlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(XmlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
