use crate::{kind::WatSyntaxKind, language::WatLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, WatLanguage>;

pub struct WatLexer<'config> {
    config: &'config WatLanguage,
}

impl<'config> WatLexer<'config> {
    pub fn new(config: &'config WatLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(WatSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(WatSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(WatSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();
        // 行注释
        if let Some(';') = state.peek() {
            if let Some(';') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(WatSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // 块注释(;...;)
        if let Some('(') = state.peek() {
            if let Some(';') = source.get_char_at(start_pos + 1) {
                state.advance(2);

                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    let current_pos = state.get_position();
                    if let Some('(') = state.peek() {
                        if let Some(';') = source.get_char_at(current_pos + 1) {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }

                    if let Some(';') = state.peek() {
                        if let Some(')') = source.get_char_at(current_pos + 1) {
                            state.advance(2);
                            depth -= 1;
                            continue;
                        }
                    }

                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(WatSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // WAT 字符串字面量 "..."
        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(WatSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符

            state.add_token(WatSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // 处理负号
            let mut has_sign = false;
            if ch == '+' || ch == '-' {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1);
                        has_sign = true;
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return false;
                }
            }

            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    // 检查十六进0x
                    if ch == '0' {
                        if let Some('x') | Some('X') = state.peek_next_n(1) {
                            state.advance(2);
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_hexdigit() || digit == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            state.add_token(WatSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }

                    // 十进制数

                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() || digit == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    // 检查是否是浮点

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

                                state.add_token(WatSyntaxKind::FloatLiteral, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }

                    // 检查科学计数法（整数）
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
                            state.add_token(WatSyntaxKind::FloatLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }

                    state.add_token(WatSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                    return true;
                }
                else if has_sign {
                    // 如果有符号但后面不是数字，回退
                    state.set_position(start_pos);
                    return false;
                }
            }
        }

        false
    }

    /// 处理标识符和关键

    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    // 模块结构关键
                    "module" => WatSyntaxKind::ModuleKw,
                    "func" => WatSyntaxKind::FuncKw,
                    "export" => WatSyntaxKind::ExportKw,
                    "import" => WatSyntaxKind::ImportKw,
                    "type" => WatSyntaxKind::TypeKw,
                    "param" => WatSyntaxKind::ParamKw,
                    "result" => WatSyntaxKind::ResultKw,
                    "local" => WatSyntaxKind::LocalKw,
                    "global" => WatSyntaxKind::GlobalKw,
                    "memory" => WatSyntaxKind::MemoryKw,
                    "table" => WatSyntaxKind::TableKw,
                    "elem" => WatSyntaxKind::ElemKw,
                    "data" => WatSyntaxKind::DataKw,
                    "start" => WatSyntaxKind::StartKw,

                    // 控制流关键字
                    "block" => WatSyntaxKind::BlockKw,
                    "loop" => WatSyntaxKind::LoopKw,
                    "if" => WatSyntaxKind::IfKw,
                    "then" => WatSyntaxKind::ThenKw,
                    "else" => WatSyntaxKind::ElseKw,
                    "end" => WatSyntaxKind::EndKw,
                    "br" => WatSyntaxKind::BrKw,
                    "br_if" => WatSyntaxKind::BrIfKw,
                    "br_table" => WatSyntaxKind::BrTableKw,
                    "return" => WatSyntaxKind::ReturnKw,
                    "call" => WatSyntaxKind::CallKw,
                    "call_indirect" => WatSyntaxKind::CallIndirectKw,

                    // 变量操作关键
                    "local.get" => WatSyntaxKind::LocalGetKw,
                    "local.set" => WatSyntaxKind::LocalSetKw,
                    "local.tee" => WatSyntaxKind::LocalTeeKw,
                    "global.get" => WatSyntaxKind::GlobalGetKw,
                    "global.set" => WatSyntaxKind::GlobalSetKw,

                    // 内存操作关键
                    "i32.load" => WatSyntaxKind::I32LoadKw,
                    "i64.load" => WatSyntaxKind::I64LoadKw,
                    "f32.load" => WatSyntaxKind::F32LoadKw,
                    "f64.load" => WatSyntaxKind::F64LoadKw,
                    "i32.store" => WatSyntaxKind::I32StoreKw,
                    "i64.store" => WatSyntaxKind::I64StoreKw,
                    "f32.store" => WatSyntaxKind::F32StoreKw,
                    "f64.store" => WatSyntaxKind::F64StoreKw,

                    // 常量关键
                    "i32.const" => WatSyntaxKind::I32ConstKw,
                    "i64.const" => WatSyntaxKind::I64ConstKw,
                    "f32.const" => WatSyntaxKind::F32ConstKw,
                    "f64.const" => WatSyntaxKind::F64ConstKw,

                    // 算术操作关键
                    "i32.add" => WatSyntaxKind::I32AddKw,
                    "i64.add" => WatSyntaxKind::I64AddKw,
                    "f32.add" => WatSyntaxKind::F32AddKw,
                    "f64.add" => WatSyntaxKind::F64AddKw,
                    "i32.sub" => WatSyntaxKind::I32SubKw,
                    "i64.sub" => WatSyntaxKind::I64SubKw,
                    "f32.sub" => WatSyntaxKind::F32SubKw,
                    "f64.sub" => WatSyntaxKind::F64SubKw,
                    "i32.mul" => WatSyntaxKind::I32MulKw,
                    "i64.mul" => WatSyntaxKind::I64MulKw,
                    "f32.mul" => WatSyntaxKind::F32MulKw,
                    "f64.mul" => WatSyntaxKind::F64MulKw,
                    "i32.div_s" => WatSyntaxKind::I32DivSKw,
                    "i32.div_u" => WatSyntaxKind::I32DivUKw,
                    "i64.div_s" => WatSyntaxKind::I64DivSKw,
                    "i64.div_u" => WatSyntaxKind::I64DivUKw,
                    "f32.div" => WatSyntaxKind::F32DivKw,
                    "f64.div" => WatSyntaxKind::F64DivKw,

                    // 比较操作关键
                    "i32.eq" => WatSyntaxKind::I32EqKw,
                    "i64.eq" => WatSyntaxKind::I64EqKw,
                    "f32.eq" => WatSyntaxKind::F32EqKw,
                    "f64.eq" => WatSyntaxKind::F64EqKw,
                    "i32.ne" => WatSyntaxKind::I32NeKw,
                    "i64.ne" => WatSyntaxKind::I64NeKw,
                    "f32.ne" => WatSyntaxKind::F32NeKw,
                    "f64.ne" => WatSyntaxKind::F64NeKw,
                    "i32.lt_s" => WatSyntaxKind::I32LtSKw,
                    "i32.lt_u" => WatSyntaxKind::I32LtUKw,
                    "i64.lt_s" => WatSyntaxKind::I64LtSKw,
                    "i64.lt_u" => WatSyntaxKind::I64LtUKw,
                    "f32.lt" => WatSyntaxKind::F32LtKw,
                    "f64.lt" => WatSyntaxKind::F64LtKw,
                    "i32.gt_s" => WatSyntaxKind::I32GtSKw,
                    "i32.gt_u" => WatSyntaxKind::I32GtUKw,
                    "i64.gt_s" => WatSyntaxKind::I64GtSKw,
                    "i64.gt_u" => WatSyntaxKind::I64GtUKw,
                    "f32.gt" => WatSyntaxKind::F32GtKw,
                    "f64.gt" => WatSyntaxKind::F64GtKw,
                    "i32.le_s" => WatSyntaxKind::I32LeSKw,
                    "i32.le_u" => WatSyntaxKind::I32LeUKw,
                    "i64.le_s" => WatSyntaxKind::I64LeSKw,
                    "i64.le_u" => WatSyntaxKind::I64LeUKw,
                    "f32.le" => WatSyntaxKind::F32LeKw,
                    "f64.le" => WatSyntaxKind::F64LeKw,
                    "i32.ge_s" => WatSyntaxKind::I32GeSKw,
                    "i32.ge_u" => WatSyntaxKind::I32GeUKw,
                    "i64.ge_s" => WatSyntaxKind::I64GeSKw,
                    "i64.ge_u" => WatSyntaxKind::I64GeUKw,
                    "f32.ge" => WatSyntaxKind::F32GeKw,
                    "f64.ge" => WatSyntaxKind::F64GeKw,

                    // 其他操作关键
                    "drop" => WatSyntaxKind::DropKw,
                    "select" => WatSyntaxKind::SelectKw,
                    "unreachable" => WatSyntaxKind::UnreachableKw,
                    "nop" => WatSyntaxKind::NopKw,

                    _ => WatSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号
    fn lex_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    WatSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WatSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    WatSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WatSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    WatSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WatSyntaxKind::RightBrace
                }
                ':' => {
                    state.advance(1);
                    WatSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    WatSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    WatSyntaxKind::Dot
                }
                ',' => {
                    state.advance(1);
                    WatSyntaxKind::Comma
                }
                '?' => {
                    state.advance(1);
                    WatSyntaxKind::Question
                }
                '!' => {
                    state.advance(1);
                    WatSyntaxKind::Bang
                }
                '@' => {
                    state.advance(1);
                    WatSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    WatSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    WatSyntaxKind::Dollar
                }
                '%' => {
                    state.advance(1);
                    WatSyntaxKind::Percent
                }
                '^' => {
                    state.advance(1);
                    WatSyntaxKind::Caret
                }
                '&' => {
                    state.advance(1);
                    WatSyntaxKind::Ampersand
                }
                '*' => {
                    state.advance(1);
                    WatSyntaxKind::Star
                }
                '+' => {
                    state.advance(1);
                    WatSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    WatSyntaxKind::Minus
                }
                '=' => {
                    state.advance(1);
                    WatSyntaxKind::Eq
                }
                '<' => {
                    state.advance(1);
                    WatSyntaxKind::LessThan
                }
                '>' => {
                    state.advance(1);
                    WatSyntaxKind::GreaterThan
                }
                '/' => {
                    state.advance(1);
                    WatSyntaxKind::Slash
                }
                '\\' => {
                    state.advance(1);
                    WatSyntaxKind::Backslash
                }
                '|' => {
                    state.advance(1);
                    WatSyntaxKind::Pipe
                }
                '~' => {
                    state.advance(1);
                    WatSyntaxKind::Tilde
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
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | '{' | '}' | ':' | ';' | '.' | ',' | '?' | '!' | '@'
                | '#' | '$' | '%' | '^' | '&' | '*' | '+' | '-' | '=' | '<' | '>' | '/' | '\\' | '|' | '~' | '"' => break,
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(WatSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<WatLanguage> for WatLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<WatSyntaxKind> {
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

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WatSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(WatSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
