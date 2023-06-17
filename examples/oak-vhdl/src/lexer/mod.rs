use crate::{kind::VhdlSyntaxKind, language::VhdlLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, VhdlLanguage>;

pub struct VhdlLexer<'config> {
    config: &'config VhdlLanguage,
}

impl<'config> VhdlLexer<'config> {
    pub fn new(config: &'config VhdlLanguage) -> Self {
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
            state.add_token(VhdlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(VhdlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VhdlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // VHDL 行注
        if let Some('-') = state.peek() {
            if let Some('-') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(VhdlSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // VHDL 字符串字面量 "..."
        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    // 检查是否是双引号转

                    if let Some('"') = state.peek() {
                        state.advance(1);
                        continue;
                    }
                    state.add_token(VhdlSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符

            state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        // VHDL 字符字面量
        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(VhdlSyntaxKind::CharLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 未闭合的字符字面

            state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() || digit == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是基数字面 (16#FF#)
                if let Some('#') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            state.advance(1);
                        }
                        else if ch == '#' {
                            state.advance(1);
                            state.add_token(VhdlSyntaxKind::BasedLiteral, start_pos, state.get_position());
                            return true;
                        }
                        else {
                            break;
                        }
                    }
                    // 未闭合的基数字面

                    state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
                    return true;
                }

                // 检查是否是实数
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() || digit == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }

                            // 检查科学计数法
                            if let Some(e) = state.peek() {
                                if e == 'e' || e == 'E' {
                                    state.advance(1);
                                    if let Some(sign) = state.peek() {
                                        if sign == '+' || sign == '-' {
                                            state.advance(1);
                                        }
                                    }
                                    while let Some(digit) = state.peek() {
                                        if digit.is_ascii_digit() || digit == '_' {
                                            state.advance(1);
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                            }

                            state.add_token(VhdlSyntaxKind::RealLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                state.add_token(VhdlSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理位字符串字面

    fn lex_bit_string(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 检查位字符串前缀 (B, O, X)
        if let Some(prefix) = state.peek() {
            if prefix == 'B' || prefix == 'O' || prefix == 'X' || prefix == 'b' || prefix == 'o' || prefix == 'x' {
                if let Some('"') = source.get_char_at(start_pos + 1) {
                    state.advance(2); // 跳过前缀和引

                    while let Some(ch) = state.peek() {
                        if ch == '"' {
                            state.advance(1);
                            state.add_token(VhdlSyntaxKind::BitStringLiteral, start_pos, state.get_position());
                            return true;
                        }
                        else if ch.is_ascii_alphanumeric() || ch == '_' {
                            state.advance(1);
                        }
                        else if ch == '\n' || ch == '\r' {
                            break;
                        }
                        else {
                            state.advance(ch.len_utf8());
                        }
                    }

                    // 未闭合的位字符串
                    state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// 处理标识符和关键

    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字 (VHDL 不区分大小写)
                if let Some(text) = source.get_text_in((start_pos..state.get_position()).into()) {
                    let text = text.to_lowercase();
                    let token_kind = match text.as_str() {
                        // VHDL 关键
                        "entity" => VhdlSyntaxKind::EntityKw,
                        "architecture" => VhdlSyntaxKind::ArchitectureKw,
                        "begin" => VhdlSyntaxKind::BeginKw,
                        "end" => VhdlSyntaxKind::EndKw,
                        "process" => VhdlSyntaxKind::ProcessKw,
                        "signal" => VhdlSyntaxKind::SignalKw,
                        "variable" => VhdlSyntaxKind::VariableKw,
                        "constant" => VhdlSyntaxKind::ConstantKw,
                        "component" => VhdlSyntaxKind::ComponentKw,
                        "port" => VhdlSyntaxKind::PortKw,
                        "map" => VhdlSyntaxKind::MapKw,
                        "generic" => VhdlSyntaxKind::GenericKw,
                        "library" => VhdlSyntaxKind::LibraryKw,
                        "use" => VhdlSyntaxKind::UseKw,
                        "package" => VhdlSyntaxKind::PackageKw,
                        "function" => VhdlSyntaxKind::FunctionKw,
                        "procedure" => VhdlSyntaxKind::ProcedureKw,
                        "type" => VhdlSyntaxKind::TypeKw,
                        "subtype" => VhdlSyntaxKind::SubtypeKw,
                        "record" => VhdlSyntaxKind::RecordKw,
                        "array" => VhdlSyntaxKind::ArrayKw,
                        "if" => VhdlSyntaxKind::IfKw,
                        "then" => VhdlSyntaxKind::ThenKw,
                        "else" => VhdlSyntaxKind::ElseKw,
                        "elsif" => VhdlSyntaxKind::ElsifKw,
                        "case" => VhdlSyntaxKind::CaseKw,
                        "when" => VhdlSyntaxKind::WhenKw,
                        "loop" => VhdlSyntaxKind::LoopKw,
                        "for" => VhdlSyntaxKind::ForKw,
                        "while" => VhdlSyntaxKind::WhileKw,
                        "exit" => VhdlSyntaxKind::ExitKw,
                        "next" => VhdlSyntaxKind::NextKw,
                        "return" => VhdlSyntaxKind::ReturnKw,
                        "wait" => VhdlSyntaxKind::WaitKw,
                        "until" => VhdlSyntaxKind::UntilKw,
                        "in" => VhdlSyntaxKind::InKw,
                        "out" => VhdlSyntaxKind::OutKw,
                        "inout" => VhdlSyntaxKind::InoutKw,
                        "buffer" => VhdlSyntaxKind::BufferKw,
                        "linkage" => VhdlSyntaxKind::LinkageKw,
                        "downto" => VhdlSyntaxKind::DowntoKw,
                        "to" => VhdlSyntaxKind::ToKw,
                        "generate" => VhdlSyntaxKind::GenerateKw,
                        "with" => VhdlSyntaxKind::WithKw,
                        "select" => VhdlSyntaxKind::SelectKw,
                        "all" => VhdlSyntaxKind::AllKw,
                        "others" => VhdlSyntaxKind::OthersKw,
                        "null" => VhdlSyntaxKind::NullKw,
                        "open" => VhdlSyntaxKind::OpenKw,
                        "is" => VhdlSyntaxKind::IsKw,
                        "of" => VhdlSyntaxKind::OfKw,
                        "range" => VhdlSyntaxKind::RangeKw,
                        "reverse_range" => VhdlSyntaxKind::ReverseRangeKw,
                        "attribute" => VhdlSyntaxKind::AttributeKw,
                        "alias" => VhdlSyntaxKind::AliasKw,
                        "file" => VhdlSyntaxKind::FileKw,
                        "access" => VhdlSyntaxKind::AccessKw,
                        "after" => VhdlSyntaxKind::AfterKw,
                        "assert" => VhdlSyntaxKind::AssertKw,
                        "report" => VhdlSyntaxKind::ReportKw,
                        "severity" => VhdlSyntaxKind::SeverityKw,
                        // 基本类型
                        "bit" => VhdlSyntaxKind::BitKw,
                        "bit_vector" => VhdlSyntaxKind::BitVectorKw,
                        "boolean" => VhdlSyntaxKind::BooleanKw,
                        "character" => VhdlSyntaxKind::CharacterKw,
                        "integer" => VhdlSyntaxKind::IntegerKw,
                        "natural" => VhdlSyntaxKind::NaturalKw,
                        "positive" => VhdlSyntaxKind::PositiveKw,
                        "real" => VhdlSyntaxKind::RealKw,
                        "string" => VhdlSyntaxKind::StringKw,
                        "time" => VhdlSyntaxKind::TimeKw,
                        "std_logic" => VhdlSyntaxKind::StdLogicKw,
                        "std_logic_vector" => VhdlSyntaxKind::StdLogicVectorKw,
                        "unsigned" => VhdlSyntaxKind::UnsignedKw,
                        "signed" => VhdlSyntaxKind::SignedKw,
                        // 逻辑操作
                        "and" => VhdlSyntaxKind::And,
                        "or" => VhdlSyntaxKind::Or,
                        "nand" => VhdlSyntaxKind::Nand,
                        "nor" => VhdlSyntaxKind::Nor,
                        "xor" => VhdlSyntaxKind::Xor,
                        "xnor" => VhdlSyntaxKind::Xnor,
                        "not" => VhdlSyntaxKind::Not,
                        "sll" => VhdlSyntaxKind::Sll,
                        "srl" => VhdlSyntaxKind::Srl,
                        "sla" => VhdlSyntaxKind::Sla,
                        "sra" => VhdlSyntaxKind::Sra,
                        "rol" => VhdlSyntaxKind::Rol,
                        "ror" => VhdlSyntaxKind::Ror,
                        "mod" => VhdlSyntaxKind::Mod,
                        "rem" => VhdlSyntaxKind::Rem,
                        "abs" => VhdlSyntaxKind::Abs,
                        _ => VhdlSyntaxKind::Identifier,
                    };

                    state.add_token(token_kind, start_pos, state.get_position());
                    return true;
                }
                else {
                    state.add_token(VhdlSyntaxKind::Identifier, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    VhdlSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VhdlSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    VhdlSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VhdlSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    VhdlSyntaxKind::Semicolon
                }
                ':' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Assign
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Colon
                    }
                }
                ',' => {
                    state.advance(1);
                    VhdlSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    VhdlSyntaxKind::Dot
                }
                '\'' => {
                    state.advance(1);
                    VhdlSyntaxKind::Apostrophe
                }
                '"' => {
                    state.advance(1);
                    VhdlSyntaxKind::Quote
                }
                '|' => {
                    state.advance(1);
                    VhdlSyntaxKind::Pipe
                }
                '_' => {
                    state.advance(1);
                    VhdlSyntaxKind::Underscore
                }
                '+' => {
                    state.advance(1);
                    VhdlSyntaxKind::Plus
                }
                '-' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Minus
                    }
                }
                '*' => {
                    if let Some('*') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Pow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Star
                    }
                }
                '/' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Ne
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Slash
                    }
                }
                '&' => {
                    state.advance(1);
                    VhdlSyntaxKind::Ampersand
                }
                '=' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::DoubleArrow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Eq
                    }
                }
                '<' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Le
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Lt
                    }
                }
                '>' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        VhdlSyntaxKind::Ge
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Gt
                    }
                }
                '#' => {
                    state.advance(1);
                    VhdlSyntaxKind::Hash
                }
                '@' => {
                    state.advance(1);
                    VhdlSyntaxKind::At
                }
                '?' => {
                    state.advance(1);
                    VhdlSyntaxKind::Question
                }
                '$' => {
                    state.advance(1);
                    VhdlSyntaxKind::Dollar
                }
                '%' => {
                    state.advance(1);
                    VhdlSyntaxKind::Percent
                }
                '^' => {
                    state.advance(1);
                    VhdlSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    VhdlSyntaxKind::Tilde
                }
                '\\' => {
                    state.advance(1);
                    VhdlSyntaxKind::Backslash
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

    /// 处理普通文

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | ';' | ':' | ',' | '.' | '\'' | '"' | '|' | '_' | '+'
                | '-' | '*' | '/' | '&' | '=' | '<' | '>' | '#' | '@' | '?' | '$' | '%' | '^' | '~' | '\\' => break,
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VhdlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VhdlLanguage> for VhdlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<VhdlSyntaxKind> {
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

            if self.lex_bit_string(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state, source) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(VhdlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
