use crate::{kind::VhdlSyntaxKind, language::VhdlLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

#[derive(Clone, Debug)]
pub struct VhdlLexer<'config> {
    _config: &'config VhdlLanguage,
}

impl<'config> Lexer<VhdlLanguage> for VhdlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<VhdlLanguage>) -> LexOutput<VhdlLanguage> {
        let mut state = LexerState::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> VhdlLexer<'config> {
    pub fn new(config: &'config VhdlLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    state.advance(ch.len_utf8());
                }
                _ => break,
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

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
        let start_pos = state.get_position();

        // VHDL 行注释 --
        if state.consume_if_starts_with("--") {
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VhdlSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
        let start_pos = state.get_position();

        // VHDL 字符串字面量 "..."
        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    // 检查是否是双引号转义
                    if let Some('"') = state.peek() {
                        state.advance(1);
                        continue;
                    }
                    state.add_token(VhdlSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符串
            state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        // VHDL 字符字面量 '.'
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

            // 未闭合的字符字面量
            state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        // 位字符串字面量 B"...", O"...", X"..."
        if let Some(prefix) = state.peek() {
            if matches!(prefix, 'B' | 'O' | 'X' | 'b' | 'o' | 'x') {
                if let Some('"') = state.peek_next_n(1) {
                    state.advance(2); // 跳过前缀和引号

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

    /// 处理数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
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

                // 检查是否是基数字面量 (16#FF#)
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
                    // 未闭合的基数字面量
                    state.add_token(VhdlSyntaxKind::Error, start_pos, state.get_position());
                    return true;
                }

                // 检查是否是实数
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
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

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
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
                let text = state.get_text_in((start_pos..state.get_position()).into()).to_lowercase();
                let token_kind = match text.as_str() {
                    // VHDL 关键字
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
                    // 逻辑操作符
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
        }

        false
    }

    /// 处理操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                ':' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Assign
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Colon
                    }
                }
                '-' => {
                    if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Minus
                    }
                }
                '*' => {
                    if let Some('*') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Pow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Ne
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Slash
                    }
                }
                '=' => {
                    if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::DoubleArrow
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Eq
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Le
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Lt
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlSyntaxKind::Ge
                    }
                    else {
                        state.advance(1);
                        VhdlSyntaxKind::Gt
                    }
                }
                '+' => {
                    state.advance(1);
                    VhdlSyntaxKind::Plus
                }
                '&' => {
                    state.advance(1);
                    VhdlSyntaxKind::Ampersand
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

    /// 处理单字符标记
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, VhdlLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => VhdlSyntaxKind::LeftParen,
                ')' => VhdlSyntaxKind::RightParen,
                '[' => VhdlSyntaxKind::LeftBracket,
                ']' => VhdlSyntaxKind::RightBracket,
                ';' => VhdlSyntaxKind::Semicolon,
                ',' => VhdlSyntaxKind::Comma,
                '.' => VhdlSyntaxKind::Dot,
                '|' => VhdlSyntaxKind::Pipe,
                '#' => VhdlSyntaxKind::Hash,
                '@' => VhdlSyntaxKind::At,
                '?' => VhdlSyntaxKind::Question,
                '$' => VhdlSyntaxKind::Dollar,
                '%' => VhdlSyntaxKind::Percent,
                '^' => VhdlSyntaxKind::Caret,
                '~' => VhdlSyntaxKind::Tilde,
                '\\' => VhdlSyntaxKind::Backslash,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
