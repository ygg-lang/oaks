use crate::{kind::WitSyntaxKind, language::WitLanguage};
use oak_core::{
    lexer::{LexOutput, Lexer, LexerState},
    source::SourceText,
};

type State<'input> = LexerState<'input, WitLanguage>;

pub struct WitLexer<'config> {
    config: &'config WitLanguage,
}

impl<'config> WitLexer<'config> {
    pub fn new(config: &'config WitLanguage) -> Self {
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
            state.add_token(WitSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(WitSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(WitSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // WIT Component 行注释
        //
        if let Some('/') = state.peek() {
            let next_pos = state.get_position() + 1;
            if let Some('/') = source.get_char_at(next_pos) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(WitSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // WIT Component 块注释
        // ...
        if let Some('/') = state.peek() {
            let next_pos = state.get_position() + 1;
            if let Some('*') = source.get_char_at(next_pos) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if ch == '*' && source.get_char_at(next_pos) == Some('/') {
                        state.advance(2);
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(WitSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // WIT Component 字符串字面量 "..."
        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(WitSyntaxKind::StringLiteral, start_pos, state.get_position());
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
            state.add_token(WitSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 检查十六进制 0x
                if ch == '0' {
                    let next_pos = state.get_position() + 1;
                    if let Some('x') | Some('X') = source.get_char_at(next_pos) {
                        state.advance(2);
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_hexdigit() || digit == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        state.add_token(WitSyntaxKind::IntegerLiteral, start_pos, state.get_position());
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

                // 检查是否是浮点数
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = source.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            is_float = true;
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
                }

                // 检查科学计数法
                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        state.advance(1);
                        is_float = true;
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

                if is_float {
                    state.add_token(WitSyntaxKind::FloatLiteral, start_pos, state.get_position());
                }
                else {
                    state.add_token(WitSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                }
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    // 基本结构
                    "world" => WitSyntaxKind::WorldKw,
                    "interface" => WitSyntaxKind::InterfaceKw,
                    "package" => WitSyntaxKind::PackageKw,
                    "component" => WitSyntaxKind::ComponentKw,
                    "instance" => WitSyntaxKind::InstanceKw,
                    "module" => WitSyntaxKind::ModuleKw,
                    "core" => WitSyntaxKind::CoreKw,
                    "func" => WitSyntaxKind::FuncKw,
                    "type" => WitSyntaxKind::TypeKw,
                    "record" => WitSyntaxKind::RecordKw,
                    "variant" => WitSyntaxKind::VariantKw,
                    "enum" => WitSyntaxKind::EnumKw,
                    "flags" => WitSyntaxKind::FlagsKw,
                    "union" => WitSyntaxKind::UnionKw,
                    "tuple" => WitSyntaxKind::TupleKw,
                    "list" => WitSyntaxKind::ListKw,
                    "option" => WitSyntaxKind::OptionKw,
                    "result" => WitSyntaxKind::ResultKw,
                    "resource" => WitSyntaxKind::ResourceKw,

                    // 导入导出
                    "import" => WitSyntaxKind::ImportKw,
                    "export" => WitSyntaxKind::ExportKw,
                    "use" => WitSyntaxKind::UseKw,
                    "include" => WitSyntaxKind::IncludeKw,
                    "with" => WitSyntaxKind::WithKw,

                    // 类型相关
                    "static" => WitSyntaxKind::StaticKw,
                    "constructor" => WitSyntaxKind::ConstructorKw,
                    "method" => WitSyntaxKind::MethodKw,

                    // 基本类型
                    "bool" => WitSyntaxKind::BoolKw,
                    "u8" => WitSyntaxKind::U8Kw,
                    "u16" => WitSyntaxKind::U16Kw,
                    "u32" => WitSyntaxKind::U32Kw,
                    "u64" => WitSyntaxKind::U64Kw,
                    "s8" => WitSyntaxKind::S8Kw,
                    "s16" => WitSyntaxKind::S16Kw,
                    "s32" => WitSyntaxKind::S32Kw,
                    "s64" => WitSyntaxKind::S64Kw,
                    "f32" => WitSyntaxKind::F32Kw,
                    "f64" => WitSyntaxKind::F64Kw,
                    "char" => WitSyntaxKind::CharKw,
                    "string" => WitSyntaxKind::StringKw,

                    _ => WitSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理操作符和标点符号
    fn lex_operator_or_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        WitSyntaxKind::Arrow
                    }
                    else {
                        WitSyntaxKind::Minus
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        WitSyntaxKind::FatArrow
                    }
                    else {
                        WitSyntaxKind::Assign
                    }
                }
                ':' => {
                    state.advance(1);
                    WitSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    WitSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    WitSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    WitSyntaxKind::Dot
                }
                '?' => {
                    state.advance(1);
                    WitSyntaxKind::Question
                }
                '@' => {
                    state.advance(1);
                    WitSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    WitSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    WitSyntaxKind::Dollar
                }
                '%' => {
                    state.advance(1);
                    WitSyntaxKind::Percent
                }
                '&' => {
                    state.advance(1);
                    WitSyntaxKind::Ampersand
                }
                '*' => {
                    state.advance(1);
                    WitSyntaxKind::Star
                }
                '+' => {
                    state.advance(1);
                    WitSyntaxKind::Plus
                }
                '/' => {
                    state.advance(1);
                    WitSyntaxKind::Slash
                }
                '<' => {
                    state.advance(1);
                    WitSyntaxKind::Lt
                }
                '>' => {
                    state.advance(1);
                    WitSyntaxKind::Gt
                }
                '|' => {
                    state.advance(1);
                    WitSyntaxKind::Pipe
                }
                '^' => {
                    state.advance(1);
                    WitSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    WitSyntaxKind::Tilde
                }
                '!' => {
                    state.advance(1);
                    WitSyntaxKind::Bang
                }
                '(' => {
                    state.advance(1);
                    WitSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    WitSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    WitSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    WitSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    WitSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    WitSyntaxKind::RightBracket
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
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | '.' | ':' | '?' | '@' | '#'
                | '$' | '%' | '&' | '*' | '+' | '-' | '/' | '<' | '>' | '|' | '^' | '~' | '!' | '=' | '"' => break,
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(WitSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<WitLanguage> for WitLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<WitSyntaxKind> {
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

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(WitSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(WitSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
