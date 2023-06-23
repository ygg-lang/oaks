#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::VhdlLanguage, lexer::token_type::VhdlTokenType};
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
                state.add_token(VhdlTokenType::Error, start_pos, state.get_position());
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
                ' ' | '\t' | '\n' | '\r' => state.advance(ch.len_utf8()),
                _ => break,
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VhdlTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(VhdlTokenType::Comment, start_pos, state.get_position());
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
                    state.add_token(VhdlTokenType::StringLiteral, start_pos, state.get_position());
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
            state.add_token(VhdlTokenType::Error, start_pos, state.get_position());
            return true;
        }

        // VHDL 字符字面量 '.'
        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(VhdlTokenType::CharLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 未闭合的字符字面量
            state.add_token(VhdlTokenType::Error, start_pos, state.get_position());
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
                            state.add_token(VhdlTokenType::BitStringLiteral, start_pos, state.get_position());
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
                    state.add_token(VhdlTokenType::Error, start_pos, state.get_position());
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
                            state.add_token(VhdlTokenType::BasedLiteral, start_pos, state.get_position());
                            return true;
                        }
                        else {
                            break;
                        }
                    }
                    // 未闭合的基数字面量
                    state.add_token(VhdlTokenType::Error, start_pos, state.get_position());
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

                            state.add_token(VhdlTokenType::RealLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                state.add_token(VhdlTokenType::IntegerLiteral, start_pos, state.get_position());
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
                    "entity" => VhdlTokenType::EntityKw,
                    "architecture" => VhdlTokenType::ArchitectureKw,
                    "begin" => VhdlTokenType::BeginKw,
                    "end" => VhdlTokenType::EndKw,
                    "process" => VhdlTokenType::ProcessKw,
                    "signal" => VhdlTokenType::SignalKw,
                    "variable" => VhdlTokenType::VariableKw,
                    "constant" => VhdlTokenType::ConstantKw,
                    "component" => VhdlTokenType::ComponentKw,
                    "port" => VhdlTokenType::PortKw,
                    "map" => VhdlTokenType::MapKw,
                    "generic" => VhdlTokenType::GenericKw,
                    "library" => VhdlTokenType::LibraryKw,
                    "use" => VhdlTokenType::UseKw,
                    "package" => VhdlTokenType::PackageKw,
                    "function" => VhdlTokenType::FunctionKw,
                    "procedure" => VhdlTokenType::ProcedureKw,
                    "type" => VhdlTokenType::TypeKw,
                    "subtype" => VhdlTokenType::SubtypeKw,
                    "record" => VhdlTokenType::RecordKw,
                    "array" => VhdlTokenType::ArrayKw,
                    "if" => VhdlTokenType::IfKw,
                    "then" => VhdlTokenType::ThenKw,
                    "else" => VhdlTokenType::ElseKw,
                    "elsif" => VhdlTokenType::ElsifKw,
                    "case" => VhdlTokenType::CaseKw,
                    "when" => VhdlTokenType::WhenKw,
                    "loop" => VhdlTokenType::LoopKw,
                    "for" => VhdlTokenType::ForKw,
                    "while" => VhdlTokenType::WhileKw,
                    "exit" => VhdlTokenType::ExitKw,
                    "next" => VhdlTokenType::NextKw,
                    "return" => VhdlTokenType::ReturnKw,
                    "wait" => VhdlTokenType::WaitKw,
                    "until" => VhdlTokenType::UntilKw,
                    "in" => VhdlTokenType::InKw,
                    "out" => VhdlTokenType::OutKw,
                    "inout" => VhdlTokenType::InoutKw,
                    "buffer" => VhdlTokenType::BufferKw,
                    "linkage" => VhdlTokenType::LinkageKw,
                    "downto" => VhdlTokenType::DowntoKw,
                    "to" => VhdlTokenType::ToKw,
                    "generate" => VhdlTokenType::GenerateKw,
                    "with" => VhdlTokenType::WithKw,
                    "select" => VhdlTokenType::SelectKw,
                    "all" => VhdlTokenType::AllKw,
                    "others" => VhdlTokenType::OthersKw,
                    "null" => VhdlTokenType::NullKw,
                    "open" => VhdlTokenType::OpenKw,
                    "is" => VhdlTokenType::IsKw,
                    "of" => VhdlTokenType::OfKw,
                    "range" => VhdlTokenType::RangeKw,
                    "reverse_range" => VhdlTokenType::ReverseRangeKw,
                    "attribute" => VhdlTokenType::AttributeKw,
                    "alias" => VhdlTokenType::AliasKw,
                    "file" => VhdlTokenType::FileKw,
                    "access" => VhdlTokenType::AccessKw,
                    "after" => VhdlTokenType::AfterKw,
                    "assert" => VhdlTokenType::AssertKw,
                    "report" => VhdlTokenType::ReportKw,
                    "severity" => VhdlTokenType::SeverityKw,
                    // 基本类型
                    "bit" => VhdlTokenType::BitKw,
                    "bit_vector" => VhdlTokenType::BitVectorKw,
                    "boolean" => VhdlTokenType::BooleanKw,
                    "character" => VhdlTokenType::CharacterKw,
                    "integer" => VhdlTokenType::IntegerKw,
                    "natural" => VhdlTokenType::NaturalKw,
                    "positive" => VhdlTokenType::PositiveKw,
                    "real" => VhdlTokenType::RealKw,
                    "string" => VhdlTokenType::StringKw,
                    "time" => VhdlTokenType::TimeKw,
                    "std_logic" => VhdlTokenType::StdLogicKw,
                    "std_logic_vector" => VhdlTokenType::StdLogicVectorKw,
                    "unsigned" => VhdlTokenType::UnsignedKw,
                    "signed" => VhdlTokenType::SignedKw,
                    // 逻辑操作符
                    "and" => VhdlTokenType::And,
                    "or" => VhdlTokenType::Or,
                    "nand" => VhdlTokenType::Nand,
                    "nor" => VhdlTokenType::Nor,
                    "xor" => VhdlTokenType::Xor,
                    "xnor" => VhdlTokenType::Xnor,
                    "not" => VhdlTokenType::Not,
                    "sll" => VhdlTokenType::Sll,
                    "srl" => VhdlTokenType::Srl,
                    "sla" => VhdlTokenType::Sla,
                    "sra" => VhdlTokenType::Sra,
                    "rol" => VhdlTokenType::Rol,
                    "ror" => VhdlTokenType::Ror,
                    "mod" => VhdlTokenType::Mod,
                    "rem" => VhdlTokenType::Rem,
                    "abs" => VhdlTokenType::Abs,
                    _ => VhdlTokenType::Identifier,
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
                '=' => {
                    if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Arrow
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Eq
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Ne
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Slash
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Le
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Ne
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::ShiftLeft
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Lt
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::ShiftRight
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Gt
                    }
                }
                '+' => {
                    state.advance(1);
                    VhdlTokenType::Plus
                }
                '-' => {
                    if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Arrow
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Minus
                    }
                }
                '*' => {
                    if let Some('*') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Pow
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Star
                    }
                }
                ':' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VhdlTokenType::Assign
                    }
                    else {
                        state.advance(1);
                        VhdlTokenType::Colon
                    }
                }
                '&' => {
                    state.advance(1);
                    VhdlTokenType::Ampersand
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
                '(' => VhdlTokenType::LeftParen,
                ')' => VhdlTokenType::RightParen,
                '[' => VhdlTokenType::LeftBracket,
                ']' => VhdlTokenType::RightBracket,
                ';' => VhdlTokenType::Semicolon,
                ',' => VhdlTokenType::Comma,
                '.' => VhdlTokenType::Dot,
                '|' => VhdlTokenType::Pipe,
                '#' => VhdlTokenType::Hash,
                '@' => VhdlTokenType::At,
                '?' => VhdlTokenType::Question,
                '$' => VhdlTokenType::Dollar,
                '%' => VhdlTokenType::Percent,
                '^' => VhdlTokenType::Caret,
                '~' => VhdlTokenType::Tilde,
                '\\' => VhdlTokenType::Backslash,
                '!' => VhdlTokenType::Exclamation,
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
